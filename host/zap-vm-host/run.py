#!/usr/bin/env python3
"""Non-Rust Zap VM host.

This is a faithful re-implementation of the Zap stack VM defined in
`bootstrap/b3/vm.zp` (vm_run / vm_step). It executes the same bytecode
instruction stream that the Zap-written compiler (`bootstrap/b4/native_independent.zp`
via `seed_execute_owned_pipeline`) emits, so Zap programs can run without the
native Rust reference interpreter.

It is intentionally a *host* (a seed executor), not the compiler: the compiler
and its intermediate representations remain written in Zap. This host only
needs to interpret the final bytecode, which removes the requirement that the
Zap VM itself be executed by Rust.

Run a serialized program (newline/JSON-array of instruction objects):

    python3 run.py program.json

Or import `run(program)` and inspect the resulting state's `output` list.
"""
import json
import sys


def _fail(state, message):
    return dict(state, error=message, halted=True)


def _push(state, value):
    return dict(state, stack=state["stack"] + [value])


def _pop(state):
    stack = state["stack"]
    if len(stack) == 0:
        return {"error": "stack_underflow", "value": None, "state": state}
    return {"error": None, "value": stack[-1], "state": dict(state, stack=stack[:-1])}


def _store(state, name, value):
    return dict(state, locals=state["locals"] + [{"name": name, "value": value}])


def _load(state, name):
    for item in reversed(state["locals"]):
        if item["name"] == name:
            return {"error": None, "value": item["value"], "state": state}
    if name in state["functions"]:
        return {"error": None,
                "value": {"captures": [], "closure": True, "function_name": name},
                "state": state}
    return {"error": "unknown_local:" + name, "value": None, "state": state}


def _bind_arguments(state, params, values):
    for i, p in enumerate(params):
        state = _store(state, p, values[i])
    return state


def _pop_arguments(state, remaining, values):
    if remaining == 0:
        values.reverse()
        return {"error": None, "state": state, "values": values}
    popped = _pop(state)
    if popped["error"] is not None:
        return {"error": popped["error"], "state": state, "values": []}
    return _pop_arguments(popped["state"], remaining - 1, values + [popped["value"]])


def _resolve_function(state, name, argc, owner):
    """Mirror vm_find_function_arity / method arity resolution (arity-only)."""
    functions = state["functions"]
    if owner is not None:
        for fn in functions.values():
            if fn.get("op") == "function_def" and fn.get("name") == name \
                    and len(fn.get("params", [])) == argc and fn.get("owner") == owner:
                return fn
    for fn in functions.values():
        if fn.get("op") == "function_def" and fn.get("name") == name \
                and len(fn.get("params", [])) == argc:
            return fn
    return None


# ---------------------------------------------------------------------------
# Object model, classes, method dispatch, MRO, super, try/raise helpers.
# Objects are mutable dicts; field stores mutate in place so a `self`
# reference and its holder observe the same state (single-reference semantics
# match the Zap reference VM for typical class usage).
# ---------------------------------------------------------------------------

def _object_field(obj, field):
    if not isinstance(obj, dict) or not obj.get("object"):
        return None
    for f in reversed(obj["fields"]):
        if f["name"] == field:
            return f["value"]
    return None


def _object_set_field(obj, field, value):
    for f in obj["fields"]:
        if f["name"] == field:
            f["value"] = value
            return obj
    obj["fields"].append({"name": field, "value": value})
    return obj


def _object_set_path(obj, path, value):
    if not isinstance(obj, dict) or not obj.get("object"):
        return obj
    cur = obj
    for name in path[1:-1]:
        fv = _object_field(cur, name)
        if not isinstance(fv, dict) or not fv.get("object"):
            fv = {"class_name": "Dynamic", "fields": [], "object": True}
            _object_set_field(cur, name, fv)
            cur = fv
        else:
            cur = fv
    return _object_set_field(cur, path[-1], value)


def _class_mro(functions, name, visiting=None):
    if visiting is None:
        visiting = []
    if name in visiting:
        return None
    cls = functions.get(name)
    if cls is None or cls.get("op") not in ("class_def", "trait_def"):
        return None
    base = cls.get("base")
    parents = [base] if base is not None else list(cls.get("parents") or [])
    sequences = []
    for parent in parents:
        pmro = _class_mro(functions, parent, visiting + [name])
        if pmro is None:
            return None
        sequences.append(pmro)
    sequences.append(parents)
    merged = _c3_merge(sequences)
    if merged is None:
        return None
    return [name] + merged


def _c3_merge(sequences):
    active = [s for s in sequences if len(s) > 0]
    if len(active) == 0:
        return []
    candidates = [s[0] for s in active]
    for cand in candidates:
        if not _mro_tail_has_name(active, cand):
            reduced = _mro_remove_head(active, cand)
            sub = _c3_merge(reduced)
            if sub is None:
                return None
            return [cand] + sub
    return None


def _mro_tail_has_name(sequences, name):
    for seq in sequences:
        for item in seq[1:]:
            if item == name:
                return True
    return False


def _mro_remove_head(sequences, candidate):
    result = []
    for seq in sequences:
        if len(seq) > 0 and seq[0] == candidate:
            result.append(seq[1:])
        else:
            result.append(seq)
    return result


def _find_method_direct(functions, owner, name):
    for fn in functions.values():
        if fn.get("op") == "function_def" and fn.get("owner") == owner \
                and fn.get("name") == name:
            if fn.get("required") is not True:
                return fn
    return None


def _resolve_trait_method(functions, cls, name):
    trait_names = []
    if "traits" in cls:
        trait_names += cls["traits"]
    if "interfaces" in cls:
        trait_names += cls["interfaces"]
    candidates = []
    for tn in trait_names:
        m = _find_method_direct(functions, tn, name)
        if m is not None:
            candidates.append({"method": m, "owner": tn})
    if len(candidates) == 1:
        return candidates[0]
    return None


def _resolve_method_in_mro(functions, mro, name, index=0):
    if index >= len(mro):
        return None
    method = _find_method_direct(functions, mro[index], name)
    if method is not None:
        return {"method": method, "owner": mro[index]}
    cls = functions.get(mro[index])
    if cls is not None and cls.get("op") in ("class_def", "trait_def"):
        trait = _resolve_trait_method(functions, cls, name)
        if trait is not None:
            return trait
    return _resolve_method_in_mro(functions, mro, name, index + 1)


def _make_call_state(state, definition, owner, argc, args, receiver=None,
                     is_constructor=False, constructor_instance=None):
    """Build the callee state for a call/method-call/super-call."""
    if receiver is not None:
        args = [receiver] + args
    call_argc = len(args)
    if len(definition.get("params", [])) != call_argc:
        return _fail(state, "arity_error:" + definition.get("name", "?"))
    frame = {"function": definition.get("name"), "owner": definition.get("owner"),
             "return_ip": state["ip"], "locals": state["locals"],
             "handlers": state["handlers"], "stack": state["stack"],
             "is_constructor": is_constructor,
             "constructor_instance": constructor_instance,
             "closure_name": None, "capture_names": []}
    callee = dict(state, ip=definition["entry"], locals=[], handlers=[],
                  stack=[], frames=state["frames"] + [frame])
    return _bind_arguments(callee, definition["params"], args)


def _raise_value(state, value):
    if len(state["handlers"]) > 0:
        handlers = state["handlers"]
        handler = handlers[-1]
        st = dict(state, handlers=handlers[:-1], ip=handler["target"])
        if handler.get("binding") is not None:
            st = _store(st, handler["binding"], value)
        return st
    if len(state["frames"]) > 0:
        frame = state["frames"][-1]
        st = dict(state, handlers=frame["handlers"], ip=frame["return_ip"],
                  locals=frame["locals"], stack=frame["stack"],
                  frames=state["frames"][:-1])
        return _raise_value(st, value)
    return _fail(state, "raised")


def _capture_values(state, names, index=None, result=None):
    if index is None:
        index = 0
        result = []
    if index >= len(names):
        return result
    loaded = _load(state, names[index])
    if loaded["error"] is None:
        result = result + [{"name": names[index], "value": loaded["value"]}]
    return _capture_values(state, names, index + 1, result)


def _bind_captures(state, captures, index=0):
    if index >= len(captures):
        return state
    bound = _store(state, captures[index]["name"], captures[index]["value"])
    return _bind_captures(bound, captures, index + 1)


def _step(state, instr, program):
    op = instr["op"]
    adv = dict(state, ip=state["ip"] + 1)

    if op == "const":
        return _push(adv, instr["value"])

    if op == "store":
        popped = _pop(adv)
        if popped["error"] is not None:
            return _fail(adv, popped["error"])
        return _store(popped["state"], instr["name"], popped["value"])

    if op == "load":
        loaded = _load(adv, instr["name"])
        if loaded["error"] is not None:
            return _fail(adv, loaded["error"])
        return _push(loaded["state"], loaded["value"])

    if op == "dup":
        popped = _pop(adv)
        if popped["error"] is not None:
            return _fail(adv, popped["error"])
        return _push(_push(popped["state"], popped["value"]), popped["value"])

    if op == "pop":
        popped = _pop(adv)
        if popped["error"] is not None:
            return _fail(adv, popped["error"])
        return popped["state"]

    if op == "not":
        popped = _pop(adv)
        if popped["error"] is not None:
            return _fail(adv, popped["error"])
        return _push(popped["state"], not popped["value"])

    if op == "print":
        popped = _pop(adv)
        if popped["error"] is not None:
            return _fail(adv, popped["error"])
        return dict(popped["state"], output=popped["state"]["output"] + [popped["value"]])

    if op == "halt":
        return dict(adv, halted=True)

    if op in ("break_jump", "continue_jump"):
        # The compiler rewrites these to `jump` before vm_run; handle defensively.
        return dict(adv, ip=instr["target"])

    if op == "jump":
        return dict(adv, ip=instr["target"])

    if op in ("jump_if_true", "jump_if_false"):
        popped = _pop(adv)
        if popped["error"] is not None:
            return _fail(adv, popped["error"])
        if (op == "jump_if_true" and popped["value"]) or \
           (op == "jump_if_false" and not popped["value"]):
            return dict(popped["state"], ip=instr["target"])
        return popped["state"]

    if op == "function_def":
        # A closure binding captures the enclosing locals as a value snapshot.
        if instr.get("binding") is not None:
            captured = _capture_values(adv, instr.get("captures", []))
            closure = {"captures": captured, "closure": True,
                       "function_name": instr["name"]}
            st = _store(adv, instr["binding"], closure)
            return dict(st, ip=instr["end"])
        # Plain definition: skip the body at definition time (calls jump to entry).
        return dict(adv, ip=instr["end"])

    if op in ("return_value", "return_none"):
        if op == "return_value":
            popped = _pop(adv)
            if popped["error"] is not None:
                return _fail(adv, popped["error"])
            value = popped["value"]
            st = popped["state"]
        else:
            value = None
            st = adv
        if len(st["frames"]) == 0:
            # Top-level return: halt with the value printed.
            return dict(st, halted=True, output=st["output"] + [value])
        frame = st["frames"][-1]
        # Constructor with no explicit return yields the instance.
        if frame.get("is_constructor") and value is None:
            value = frame.get("constructor_instance")
        restored_locals = frame["locals"]
        if frame.get("function") == "__init__" and len(st["frames"]) > 0:
            restored_locals = frame["locals"] + [{"name": "self", "value": value}]
        restored = dict(st, ip=frame["return_ip"], locals=restored_locals,
                        handlers=frame["handlers"], stack=frame["stack"],
                        frames=st["frames"][:-1])
        # Push the returned value onto the caller's stack (mirrors vm_return_value).
        return _push(restored, value)

    if op == "call":
        argc = instr.get("argc", 0)
        popped = _pop_arguments(adv, argc, [])
        if popped["error"] is not None:
            return _fail(adv, popped["error"])
        args = popped["values"]
        name = instr["name"]

        # A local closure binding shadows global function resolution.
        local_ref = _load(popped["state"], name)
        closure_def_name = None
        captures = []
        if local_ref["error"] is None and isinstance(local_ref["value"], dict) \
                and local_ref["value"].get("closure"):
            closure_def_name = local_ref["value"]["function_name"]
            captures = local_ref["value"].get("captures", [])

        if name == "identity" and argc == 1:
            return _push(popped["state"], args[0])
        if name == "abs" and argc == 1:
            v = args[0]
            return _push(popped["state"], -v if v < 0 else v)

        owner = instr.get("owner")
        resolved_name = closure_def_name if closure_def_name is not None else name
        definition = _resolve_function(popped["state"], resolved_name, argc, owner)
        if definition is None:
            # Class constructor path: resolve __init__ with arity + 1 (self).
            cls = popped["state"]["functions"].get(name)
            if cls is not None and cls.get("op") in ("class_def", "trait_def"):
                init_def = _resolve_function(popped["state"], "__init__", argc + 1, name)
                if init_def is not None:
                    instance = {"class_name": name, "fields": [], "object": True}
                    return _make_call_state(popped["state"], init_def, name, argc,
                                            args, receiver=instance,
                                            is_constructor=True,
                                            constructor_instance=instance)
                if argc == 0:
                    return _push(popped["state"],
                                 {"class_name": name, "fields": [], "object": True})
                return _fail(popped["state"], "arity_error:" + name)
            return _fail(popped["state"], "unknown_call:" + name)

        callee = _make_call_state(popped["state"], definition, definition.get("owner"),
                                  argc, args)
        if captures:
            callee = _bind_captures(callee, captures)
        return callee

    # Arithmetic / comparison / logical binary ops.
    if op in ("add", "subtract", "multiply", "divide", "remainder",
              "equal", "not_equal", "less", "less_equal", "greater",
              "greater_equal", "and", "or"):
        right = _pop(adv)
        if right["error"] is not None:
            return _fail(adv, right["error"])
        left = _pop(right["state"])
        if left["error"] is not None:
            return _fail(adv, left["error"])
        a, b = left["value"], right["value"]
        if op == "add":
            v = a + b
        elif op == "subtract":
            v = a - b
        elif op == "multiply":
            v = a * b
        elif op == "divide":
            v = a / b if not isinstance(a, int) or not isinstance(b, int) else a // b
        elif op == "remainder":
            v = a % b
        elif op == "equal":
            v = a == b
        elif op == "not_equal":
            v = a != b
        elif op == "less":
            v = a < b
        elif op == "less_equal":
            v = a <= b
        elif op == "greater":
            v = a > b
        elif op == "greater_equal":
            v = a >= b
        elif op == "and":
            v = a and b
        else:
            v = a or b
        return _push(left["state"], v)

    # Object field access.
    if op == "field_load":
        receiver = _load(adv, instr["receiver"])
        if receiver["error"] is not None:
            return _fail(adv, receiver["error"])
        obj = receiver["value"]
        field = _object_field(obj, instr["field"])
        if field is None:
            return _fail(adv, "unknown_field:" + instr["field"])
        return _push(adv, field)

    if op in ("field_load_value", "field_load_path"):
        popped = _pop(adv)
        if popped["error"] is not None:
            return _fail(adv, popped["error"])
        obj = popped["value"]
        if op == "field_load_value":
            field = _object_field(obj, instr["field"])
            if field is None:
                return _fail(popped["state"], "unknown_field:" + instr["field"])
            return _push(popped["state"], field)
        # field_load_path: path[0] is a local name, rest are field names.
        path = instr["path"]
        root = _load(adv, path[0])
        if root["error"] is not None:
            return _fail(adv, root["error"])
        cur = root["value"]
        for name in path[1:]:
            cur = _object_field(cur, name)
            if cur is None:
                return _fail(adv, "unknown_field:" + name)
        return _push(adv, cur)

    if op in ("field_store", "field_store_path"):
        popped = _pop(adv)
        if popped["error"] is not None:
            return _fail(adv, popped["error"])
        st = popped["state"]
        value = popped["value"]
        if op == "field_store":
            receiver = _load(st, instr["receiver"])
            if receiver["error"] is not None:
                return _fail(st, receiver["error"])
            updated = _object_set_field(receiver["value"], instr["field"], value)
            return _store(st, instr["receiver"], updated)
        # field_store_path: path[0] local, rest fields.
        path = instr["path"]
        receiver = _load(st, path[0])
        if receiver["error"] is not None:
            return _fail(st, receiver["error"])
        updated = _object_set_path(receiver["value"], path, value)
        return _store(st, path[0], updated)

    # Class / trait definitions are collected into the functions table at load
    # time; at execution they simply skip their body.
    if op in ("class_def", "trait_def"):
        return dict(adv, ip=instr["end"])

    # Method dispatch: resolve on the receiver's class MRO, then call.
    if op == "method_call":
        popped = _pop_arguments(adv, instr["argc"], [])
        if popped["error"] is not None:
            return _fail(adv, popped["error"])
        receiver = _load(popped["state"], instr["receiver"])
        if receiver["error"] is not None:
            return _fail(popped["state"], receiver["error"])
        obj = receiver["value"]
        if not isinstance(obj, dict) or not obj.get("object"):
            return _fail(popped["state"], "not_object")
        mro = _class_mro(popped["state"]["functions"], obj["class_name"])
        if mro is None:
            return _fail(popped["state"], "unknown_class:" + obj["class_name"])
        resolution = _resolve_method_in_mro(popped["state"]["functions"], mro, instr["method"])
        if resolution is None:
            return _fail(popped["state"], "unknown_method:" + instr["method"])
        callee = _make_call_state(popped["state"], resolution["method"],
                                  resolution["owner"], instr["argc"], popped["values"],
                                  receiver=obj)
        return callee

    # try / raise.
    if op == "try_begin":
        handler = {"binding": instr.get("binding"), "target": instr["target"]}
        return dict(adv, handlers=adv["handlers"] + [handler])

    if op == "try_end":
        return dict(adv, handlers=adv["handlers"][:-1])

    if op == "raise":
        popped = _pop(adv)
        if popped["error"] is not None:
            return _fail(adv, popped["error"])
        return _raise_value(popped["state"], popped["value"])

    # super_call: resolve the next method in the current class MRO.
    if op == "super_call":
        if len(adv["frames"]) == 0:
            return _fail(adv, "super_outside_method")
        current_frame = adv["frames"][-1]
        if current_frame.get("owner") is None:
            return _fail(adv, "super_outside_class")
        popped = _pop_arguments(adv, instr["argc"], [])
        if popped["error"] is not None:
            return _fail(adv, popped["error"])
        receiver = _load(popped["state"], "self")
        if receiver["error"] is not None:
            return _fail(popped["state"], receiver["error"])
        obj = receiver["value"]
        mro = _class_mro(popped["state"]["functions"], current_frame["owner"])
        if mro is None:
            return _fail(popped["state"], "super_owner_not_in_mro:" + current_frame["owner"])
        current_index = mro.index(current_frame["owner"]) if current_frame["owner"] in mro else -1
        if current_index < 0:
            return _fail(popped["state"], "super_owner_not_in_mro:" + current_frame["owner"])
        if current_index + 1 >= len(mro):
            return _fail(popped["state"], "super_no_parent:" + current_frame["owner"])
        parent_method = _resolve_method_in_mro(popped["state"]["functions"], mro, instr["method"], current_index + 1)
        if parent_method is None:
            return _fail(popped["state"], "unknown_super_method:" + instr["method"])
        callee = _make_call_state(popped["state"], parent_method["method"],
                                  parent_method["owner"], instr["argc"], popped["values"],
                                  receiver=obj)
        return callee

    return _fail(adv, "unknown_opcode:" + op)


def run(program):
    functions = {}
    for instr in program:
        if instr.get("op") in ("function_def", "class_def", "trait_def"):
            functions[instr["name"]] = instr
    state = {"ip": 0, "halted": False, "error": None, "handlers": [],
             "locals": [], "output": [], "stack": [], "frames": [],
             "functions": functions}
    while state["ip"] < len(program) and not state["halted"]:
        state = _step(state, program[state["ip"]], program)
    return state


def _self_test():
    # Program A: let x = 7; say x  -> output [7]
    a = [
        {"op": "const", "value": 7},
        {"op": "store", "name": "x"},
        {"op": "load", "name": "x"},
        {"op": "print"},
        {"op": "halt"},
    ]
    ra = run(a)
    assert ra["output"] == [7], ra
    assert ra["error"] is None, ra

    # Program B: fn add(a,b): return a+b; say add(2,3)  -> output [5]
    b = [
        {"op": "function_def", "name": "add", "params": ["a", "b"], "entry": 1, "end": 5},
        {"op": "load", "name": "a"},          # idx1 (body start = entry)
        {"op": "load", "name": "b"},          # idx2
        {"op": "add"},                         # idx3
        {"op": "return_value"},               # idx4 (end = 5)
        {"op": "const", "value": 2},          # idx5
        {"op": "const", "value": 3},          # idx6
        {"op": "call", "name": "add", "argc": 2},  # idx7
        {"op": "print"},                      # idx8
        {"op": "halt"},                       # idx9
    ]
    rb = run(b)
    assert rb["output"] == [5], rb
    assert rb["error"] is None, rb

    # Program C: deterministic replay (same input -> same output)
    rc = run(b)
    assert rc["output"] == rb["output"], (rc, rb)
    assert rc["error"] == rb["error"]

    # Program D: branch via jump_if_false (if/else lowering shape)
    d = [
        {"op": "const", "value": 1},       # idx0
        {"op": "store", "name": "n"},      # idx1
        {"op": "load", "name": "n"},       # idx2
        {"op": "const", "value": 0},       # idx3
        {"op": "greater"},                  # idx4  n > 0 ?
        {"op": "jump_if_false", "target": 8},  # idx5 if false -> halt
        {"op": "const", "value": 100},      # idx6 positive branch result
        {"op": "print"},                    # idx7
        {"op": "halt"},                     # idx8 both paths terminate here
    ]
    rd = run(d)
    assert rd["output"] == [100], rd
    assert rd["error"] is None, rd

    # Program E: class with constructor field + method dispatch -> output [5]
    e = [
        {"op": "class_def", "name": "Point", "base": None, "entry": 1, "end": 10},
        {"op": "function_def", "name": "__init__", "owner": "Point", "params": ["self", "x"], "entry": 2, "end": 6},
        {"op": "load", "name": "self"},                 # idx2
        {"op": "load", "name": "x"},                    # idx3
        {"op": "field_store", "receiver": "self", "field": "x"},  # idx4
        {"op": "return_none"},                          # idx5
        {"op": "function_def", "name": "get", "owner": "Point", "params": ["self"], "entry": 7, "end": 10},
        {"op": "load", "name": "self"},                 # idx7
        {"op": "field_load", "receiver": "self", "field": "x"},  # idx8
        {"op": "return_value"},                         # idx9
        {"op": "const", "value": 5},                    # idx10
        {"op": "call", "name": "Point", "argc": 1},     # idx11
        {"op": "store", "name": "p"},                   # idx12
        {"op": "load", "name": "p"},                    # idx13
        {"op": "method_call", "receiver": "p", "method": "get", "argc": 0},  # idx14
        {"op": "print"},                                # idx15
        {"op": "halt"},                                 # idx16
    ]
    re_ = run(e)
    assert re_["output"] == [5], re_
    assert re_["error"] is None, re_

    # Program F: try/raise caught by handler -> output [1]
    f = [
        {"op": "try_begin", "binding": "err", "target": 6},  # idx0
        {"op": "const", "value": "boom"},                     # idx1
        {"op": "raise"},                                      # idx2
        {"op": "const", "value": 999},                        # idx3 (skipped)
        {"op": "print"},                                      # idx4 (skipped)
        {"op": "jump", "target": 9},                          # idx5 skip catch
        {"op": "const", "value": 1},                          # idx6 catch
        {"op": "print"},                                      # idx7
        {"op": "try_end"},                                     # idx8
        {"op": "halt"},                                       # idx9
    ]
    rf = run(f)
    assert rf["output"] == [1], rf
    assert rf["error"] is None, rf

    # Program G: closure capturing an enclosing variable -> output [15]
    g = [
        {"op": "function_def", "name": "make_adder", "params": ["n"], "entry": 1, "end": 8},
        {"op": "function_def", "name": "add", "binding": "add", "captures": ["n"], "params": ["x"], "entry": 2, "end": 6},
        {"op": "load", "name": "x"},                  # idx2
        {"op": "load", "name": "n"},                  # idx3
        {"op": "add"},                                # idx4
        {"op": "return_value"},                       # idx5
        {"op": "load", "name": "add"},                # idx6
        {"op": "return_value"},                       # idx7
        {"op": "const", "value": 10},                 # idx8
        {"op": "call", "name": "make_adder", "argc": 1},  # idx9
        {"op": "store", "name": "f"},                 # idx10
        {"op": "load", "name": "f"},                  # idx11
        {"op": "const", "value": 5},                  # idx12
        {"op": "call", "name": "f", "argc": 1},       # idx13
        {"op": "print"},                              # idx14
        {"op": "halt"},                               # idx15
    ]
    rg = run(g)
    assert rg["output"] == [15], rg
    assert rg["error"] is None, rg

    # Program H: loop (while with break pattern) summing 1..5 -> output [15]
    h = [
        {"op": "const", "value": 1},
        {"op": "store", "name": "i"},          # idx1
        {"op": "const", "value": 0},
        {"op": "store", "name": "total"},      # idx3
        {"op": "load", "name": "i"},           # idx4 header
        {"op": "const", "value": 5},
        {"op": "greater"},                     # i > 5
        {"op": "jump_if_false", "target": 9},  # idx7 -> body if not past 5
        {"op": "jump", "target": 18},         # idx8 -> exit
        {"op": "load", "name": "total"},      # idx9 body
        {"op": "load", "name": "i"},
        {"op": "add"},
        {"op": "store", "name": "total"},     # idx12
        {"op": "load", "name": "i"},
        {"op": "const", "value": 1},
        {"op": "add"},
        {"op": "store", "name": "i"},         # idx16
        {"op": "jump", "target": 4},          # idx17 loop back
        {"op": "load", "name": "total"},      # idx18 exit
        {"op": "print"},
        {"op": "halt"},                      # idx20
    ]
    rh = run(h)
    assert rh["output"] == [15], rh
    assert rh["error"] is None, rh

    print("zap-vm-host self-test passed: const/store/load/print/halt, "
          "function call + return, deterministic replay, branch jump")


if __name__ == "__main__":
    if len(sys.argv) > 1:
        with open(sys.argv[1]) as fh:
            program = json.load(fh)
        result = run(program)
        if result["error"] is not None:
            sys.stderr.write("vm error: " + str(result["error"]) + "\n")
            sys.exit(1)
        for value in result["output"]:
            print(value)
    else:
        _self_test()
