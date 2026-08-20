//! Stable metadata for Zap's public standard-library surface.
//!
//! The evaluator remains the source of runtime behavior; this catalog gives
//! tooling and documentation one deterministic, domain-oriented view of the
//! public API without duplicating implementation dispatch.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PublicBuiltin {
    pub name: &'static str,
    pub domain: &'static str,
}

#[allow(dead_code)]
pub(crate) const PUBLIC_BUILTINS: &[PublicBuiltin] = &[
    PublicBuiltin {
        name: "len",
        domain: "text",
    },
    PublicBuiltin {
        name: "str",
        domain: "text",
    },
    PublicBuiltin {
        name: "type",
        domain: "text",
    },
    PublicBuiltin {
        name: "upper",
        domain: "text",
    },
    PublicBuiltin {
        name: "lower",
        domain: "text",
    },
    PublicBuiltin {
        name: "trim",
        domain: "text",
    },
    PublicBuiltin {
        name: "split",
        domain: "text",
    },
    PublicBuiltin {
        name: "join",
        domain: "text",
    },
    PublicBuiltin {
        name: "contains",
        domain: "text",
    },
    PublicBuiltin {
        name: "replace",
        domain: "text",
    },
    PublicBuiltin {
        name: "abs",
        domain: "math",
    },
    PublicBuiltin {
        name: "min",
        domain: "math",
    },
    PublicBuiltin {
        name: "max",
        domain: "math",
    },
    PublicBuiltin {
        name: "pow",
        domain: "math",
    },
    PublicBuiltin {
        name: "sum",
        domain: "collections",
    },
    PublicBuiltin {
        name: "range",
        domain: "collections",
    },
    PublicBuiltin {
        name: "keys",
        domain: "collections",
    },
    PublicBuiltin {
        name: "count",
        domain: "collections",
    },
    PublicBuiltin {
        name: "reverse",
        domain: "collections",
    },
    PublicBuiltin {
        name: "read_text",
        domain: "filesystem",
    },
    PublicBuiltin {
        name: "write_text",
        domain: "filesystem",
    },
    PublicBuiltin {
        name: "read_lines",
        domain: "filesystem",
    },
    PublicBuiltin {
        name: "write_lines",
        domain: "filesystem",
    },
    PublicBuiltin {
        name: "json",
        domain: "json",
    },
    PublicBuiltin {
        name: "from_json",
        domain: "json",
    },
    PublicBuiltin {
        name: "path_join",
        domain: "system",
    },
    PublicBuiltin {
        name: "now",
        domain: "system",
    },
];

#[allow(dead_code)]
pub(crate) fn contains(name: &str) -> bool {
    PUBLIC_BUILTINS.iter().any(|builtin| builtin.name == name)
}
