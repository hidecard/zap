# Zap Standard Library Public Modules

Zap's standard library is organized into stable public domains. The runtime dispatch remains centralized for compatibility, while this domain index provides a deterministic public organization for documentation, tooling, and future package modules.

| Public module | Scope | Representative APIs |
|---|---|---|
| `text` | Text conversion and manipulation | `len`, `str`, `type`, `upper`, `lower`, `trim`, `split`, `join`, `contains`, `replace` |
| `math` | Numeric operations | `abs`, `min`, `max`, `pow`, `sqrt` |
| `collections` | Lists and maps | `sum`, `range`, `keys`, `count`, `reverse`, `get` |
| `filesystem` | Bounded text and line I/O | `read_text`, `write_text`, `read_lines`, `write_lines`, `exists` |
| `json` | JSON serialization | `json`, `from_json` |
| `system` | Environment, paths, and time | `env`, `has_env`, `path_join`, `basename`, `dirname`, `now`, `sleep` |

All public builtins use explicit argument validation and return structured runtime errors rather than silently accepting invalid input. Filesystem and JSON operations use the documented 8 MiB safety limit.

The public catalog is deterministic: each builtin appears once and belongs to one domain. The native runtime includes a catalog used by tooling and tests so documentation and future module exports can remain synchronized with implementation.

```zap
let words = split(upper("zap language"), " ")
say join(reverse(words), "-")
```

The current release line exposes these APIs as direct builtins. Namespace import syntax and remote standard-library packages remain later ecosystem milestones after P1 verification.
