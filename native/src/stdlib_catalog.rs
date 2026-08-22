//! Stable metadata for Zap's public standard-library surface.
//!
//! The evaluator remains the source of runtime behavior; this catalog gives
//! tooling and documentation one deterministic, domain-oriented view of the
//! public API without duplicating implementation dispatch.

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StabilityLabel {
    Stable,
    Experimental,
    Deprecated,
    PlatformSpecific,
}

#[allow(dead_code)]
impl StabilityLabel {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Experimental => "experimental",
            Self::Deprecated => "deprecated",
            Self::PlatformSpecific => "platform-specific",
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SemverPolicy {
    MinorCompatible,
    MajorBreaking,
}

#[allow(dead_code)]
impl SemverPolicy {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::MinorCompatible => "minor-compatible",
            Self::MajorBreaking => "major-breaking",
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PlatformSupport {
    ReleaseTargets,
    UnixOnly,
    WindowsOnly,
}

#[allow(dead_code)]
impl PlatformSupport {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseTargets => "linux,windows,macos-arm64",
            Self::UnixOnly => "linux,macos-arm64",
            Self::WindowsOnly => "windows",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PublicDomainPolicy {
    pub domain: &'static str,
    pub stability: StabilityLabel,
    pub since: &'static str,
    pub deprecation_window: Option<&'static str>,
    pub semver: SemverPolicy,
    pub platforms: PlatformSupport,
    pub input_limit: &'static str,
    pub output_limit: &'static str,
    pub timeout: &'static str,
    pub error_contract: &'static str,
    pub deterministic: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PublicBuiltin {
    pub name: &'static str,
    pub domain: &'static str,
    pub stability: StabilityLabel,
    pub since: &'static str,
    pub deprecation_window: Option<&'static str>,
    pub semver: SemverPolicy,
    pub platforms: PlatformSupport,
    pub input_limit: &'static str,
    pub output_limit: &'static str,
    pub timeout: &'static str,
    pub error_contract: &'static str,
    pub deterministic: bool,
}

#[allow(dead_code)]
pub(crate) const CATALOG_SCHEMA_VERSION: u32 = 1;

macro_rules! stable_domain {
    ($domain:literal, $input_limit:literal, $output_limit:literal, $timeout:literal) => {
        PublicDomainPolicy {
            domain: $domain,
            stability: StabilityLabel::Stable,
            since: "2.1.14",
            deprecation_window: None,
            semver: SemverPolicy::MinorCompatible,
            platforms: PlatformSupport::ReleaseTargets,
            input_limit: $input_limit,
            output_limit: $output_limit,
            timeout: $timeout,
            error_contract: "stable runtime diagnostic; malformed and oversized input fails closed",
            deterministic: true,
        }
    };
}

/// Public module/domain policy. Each public domain has one explicit stability
/// record so a future API review cannot silently omit platform or limit data.
#[allow(dead_code)]
pub(crate) const PUBLIC_DOMAINS: &[PublicDomainPolicy] = &[
    stable_domain!(
        "text",
        "8 KiB text argument",
        "8 KiB text result",
        "not applicable"
    ),
    stable_domain!(
        "math",
        "bounded integer arguments",
        "bounded integer result",
        "not applicable"
    ),
    stable_domain!(
        "collections",
        "8 MiB logical collection graph",
        "8 MiB logical collection graph",
        "not applicable"
    ),
    stable_domain!(
        "filesystem",
        "8 MiB path/content input",
        "8 MiB text/line output",
        "not applicable"
    ),
    stable_domain!(
        "json",
        "8 MiB JSON input",
        "8 MiB JSON output",
        "not applicable"
    ),
    stable_domain!(
        "system",
        "8 KiB environment/path input",
        "8 KiB text or structured result",
        "not applicable"
    ),
    stable_domain!(
        "time",
        "checked integer milliseconds",
        "checked duration map",
        "not applicable"
    ),
    stable_domain!(
        "logging",
        "8 KiB message and 64 fields",
        "64 KiB encoded record",
        "not applicable"
    ),
    stable_domain!(
        "runtime",
        "bounded diagnostic request",
        "bounded statistics map",
        "not applicable"
    ),
    stable_domain!(
        "async",
        "run-owned task and poll budgets",
        "bounded task result",
        "cooperative cancellation or poll-budget timeout"
    ),
    stable_domain!(
        "network",
        "8 KiB URL and 8 MiB request body",
        "8 MiB response body",
        "bounded connect/read/write; server wait is 10 seconds"
    ),
    stable_domain!(
        "process",
        "text command, text arguments, 1 MiB output",
        "1 MiB captured stdout/stderr",
        "bounded child wait and cleanup"
    ),
];

macro_rules! stable_builtin {
    ($name:literal, $domain:literal) => {
        PublicBuiltin {
            name: $name,
            domain: $domain,
            stability: StabilityLabel::Stable,
            since: "2.1.14",
            deprecation_window: None,
            semver: SemverPolicy::MinorCompatible,
            platforms: PlatformSupport::ReleaseTargets,
            input_limit: "see domain policy",
            output_limit: "see domain policy",
            timeout: "see domain policy",
            error_contract: "stable runtime diagnostic; malformed and oversized input fails closed",
            deterministic: true,
        }
    };
}

/// Public builtin inventory. Runtime dispatch remains in the evaluator; this
/// list is the single deterministic metadata surface for tooling and docs.
#[allow(dead_code)]
pub(crate) const PUBLIC_BUILTINS: &[PublicBuiltin] = &[
    stable_builtin!("len", "text"),
    stable_builtin!("str", "text"),
    stable_builtin!("type", "text"),
    stable_builtin!("memory_stats", "runtime"),
    stable_builtin!("upper", "text"),
    stable_builtin!("lower", "text"),
    stable_builtin!("trim", "text"),
    stable_builtin!("split", "text"),
    stable_builtin!("join", "text"),
    stable_builtin!("contains", "text"),
    stable_builtin!("replace", "text"),
    stable_builtin!("abs", "math"),
    stable_builtin!("min", "math"),
    stable_builtin!("max", "math"),
    stable_builtin!("pow", "math"),
    stable_builtin!("sum", "collections"),
    stable_builtin!("range", "collections"),
    stable_builtin!("keys", "collections"),
    stable_builtin!("entries", "collections"),
    stable_builtin!("enumerate", "collections"),
    stable_builtin!("count", "collections"),
    stable_builtin!("reverse", "collections"),
    stable_builtin!("read_text", "filesystem"),
    stable_builtin!("write_text", "filesystem"),
    stable_builtin!("read_lines", "filesystem"),
    stable_builtin!("write_lines", "filesystem"),
    stable_builtin!("file_metadata", "filesystem"),
    stable_builtin!("atomic_write", "filesystem"),
    stable_builtin!("json", "json"),
    stable_builtin!("from_json", "json"),
    stable_builtin!("from_json_typed", "json"),
    stable_builtin!("char_at", "text"),
    stable_builtin!("substring", "text"),
    stable_builtin!("codepoints", "text"),
    stable_builtin!("path_join", "system"),
    stable_builtin!("now", "system"),
    stable_builtin!("utc_now", "time"),
    stable_builtin!("duration_parts", "time"),
    stable_builtin!("duration_between", "time"),
    stable_builtin!("spawn", "async"),
    stable_builtin!("task_join", "async"),
    stable_builtin!("task_is_ready", "async"),
    stable_builtin!("task_cancel", "async"),
    stable_builtin!("task_join_timeout", "async"),
    stable_builtin!("async_capabilities", "async"),
    stable_builtin!("log_record", "logging"),
    stable_builtin!("log_json", "logging"),
    stable_builtin!("env", "system"),
    stable_builtin!("has_env", "system"),
    stable_builtin!("env_get", "system"),
    stable_builtin!("config_dir", "system"),
    stable_builtin!("config_path", "system"),
    stable_builtin!("basename", "system"),
    stable_builtin!("dirname", "system"),
    stable_builtin!("url_parse", "network"),
    stable_builtin!("url_encode", "network"),
    stable_builtin!("url_decode", "network"),
    stable_builtin!("http_get", "network"),
    stable_builtin!("http_request", "network"),
    stable_builtin!("http_serve_once", "network"),
    stable_builtin!("process_run", "process"),
];

#[allow(dead_code)]
pub(crate) fn contains(name: &str) -> bool {
    PUBLIC_BUILTINS.iter().any(|builtin| builtin.name == name)
}

#[cfg(test)]
mod tests {
    use super::{
        contains, SemverPolicy, StabilityLabel, CATALOG_SCHEMA_VERSION, PUBLIC_BUILTINS,
        PUBLIC_DOMAINS,
    };
    use std::collections::BTreeSet;

    #[test]
    fn standard_library_catalog_metadata_is_complete_and_unique() {
        assert_eq!(CATALOG_SCHEMA_VERSION, 1);
        assert_eq!(PUBLIC_DOMAINS.len(), 12);
        let domains = PUBLIC_DOMAINS
            .iter()
            .map(|policy| policy.domain)
            .collect::<BTreeSet<_>>();
        assert_eq!(domains.len(), PUBLIC_DOMAINS.len());
        for policy in PUBLIC_DOMAINS {
            assert_eq!(policy.stability, StabilityLabel::Stable);
            assert_eq!(policy.since, "2.1.14");
            assert!(policy.deprecation_window.is_none());
            assert_eq!(policy.semver, SemverPolicy::MinorCompatible);
            assert!(!policy.platforms.as_str().is_empty());
            assert!(!policy.input_limit.is_empty());
            assert!(!policy.output_limit.is_empty());
            assert!(!policy.timeout.is_empty());
            assert!(!policy.error_contract.is_empty());
            assert!(policy.deterministic);
        }

        let mut builtin_names = BTreeSet::new();
        for builtin in PUBLIC_BUILTINS {
            assert!(
                builtin_names.insert(builtin.name),
                "duplicate builtin {}",
                builtin.name
            );
            assert!(
                domains.contains(builtin.domain),
                "unknown domain {}",
                builtin.domain
            );
            assert_eq!(builtin.stability, StabilityLabel::Stable);
            assert_eq!(builtin.since, "2.1.14");
            assert!(builtin.deprecation_window.is_none());
            assert_eq!(builtin.semver, SemverPolicy::MinorCompatible);
            assert!(!builtin.platforms.as_str().is_empty());
            assert!(!builtin.input_limit.is_empty());
            assert!(!builtin.output_limit.is_empty());
            assert!(!builtin.timeout.is_empty());
            assert!(!builtin.error_contract.is_empty());
            assert!(builtin.deterministic);
        }
        assert!(contains("task_cancel"));
        assert!(contains("task_join_timeout"));
    }

    #[test]
    fn standard_library_catalog_domain_order_is_deterministic() {
        let names = PUBLIC_DOMAINS
            .iter()
            .map(|policy| policy.domain)
            .collect::<Vec<_>>();
        let mut sorted = names.clone();
        sorted.sort_unstable();
        assert_eq!(
            names,
            vec![
                "text",
                "math",
                "collections",
                "filesystem",
                "json",
                "system",
                "time",
                "logging",
                "runtime",
                "async",
                "network",
                "process",
            ]
        );
        assert_ne!(
            names, sorted,
            "domain order is intentionally documentation-oriented"
        );
    }
}
