use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryPackage {
    pub name: String,
    pub version: String,
    pub source: String,
    pub checksum: String,
    /// Yanked releases remain addressable for lockfile verification but are not
    /// selected for new dependency resolution.
    pub yanked: bool,
    /// Registry dependency requirements keyed by package name.
    pub dependencies: BTreeMap<String, String>,
}

pub fn read_index(path: &Path) -> Result<Vec<RegistryPackage>, String> {
    let bytes = fs::read(path).map_err(|e| format!("registry index read failed: {e}"))?;
    parse_index_bytes(&bytes)
}

/// Read and verify a signed index using a shared secret. The index contains a
/// `signature` field equal to HMAC-SHA256(secret, canonical packages JSON).
#[allow(dead_code)]
pub fn read_signed_index(path: &Path, secret: &[u8]) -> Result<Vec<RegistryPackage>, String> {
    let bytes = fs::read(path).map_err(|e| format!("registry index read failed: {e}"))?;
    verify_signed_index_bytes(&bytes, secret)
}

#[allow(dead_code)]
pub fn verify_signed_index_bytes(
    bytes: &[u8],
    secret: &[u8],
) -> Result<Vec<RegistryPackage>, String> {
    let root: Value = serde_json::from_slice(bytes)
        .map_err(|e| format!("registry index JSON is invalid: {e}"))?;
    let signature = root
        .get("signature")
        .and_then(Value::as_str)
        .filter(|value| is_sha256(value))
        .ok_or_else(|| "signed registry index must contain a valid signature".to_string())?;
    let packages = root
        .get("packages")
        .and_then(Value::as_array)
        .ok_or_else(|| "registry index must contain a packages array".to_string())?;
    let canonical = serde_json::to_vec(packages)
        .map_err(|e| format!("registry index canonicalization failed: {e}"))?;
    let expected = hmac_sha256_hex(secret, &canonical);
    if !constant_time_equal(signature.as_bytes(), expected.as_bytes()) {
        return Err("registry index signature mismatch".to_string());
    }
    parse_packages(packages)
}

/// Canonical registry transport schemes supported by the B1 policy layer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegistryScheme {
    Http,
    Https,
    File,
}

/// Canonical identity for a registry endpoint.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RegistryOrigin {
    pub scheme: RegistryScheme,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub path_prefix: String,
}

#[allow(dead_code)]
impl RegistryOrigin {
    pub fn as_url(&self) -> String {
        let scheme = match self.scheme {
            RegistryScheme::Http => "http",
            RegistryScheme::Https => "https",
            RegistryScheme::File => "file",
        };
        match (&self.host, self.port) {
            (Some(host), Some(port)) => format!("{scheme}://{host}:{port}{}", self.path_prefix),
            (Some(host), None) => format!("{scheme}://{host}{}", self.path_prefix),
            (None, None) => format!("{scheme}://{}", self.path_prefix),
            (None, Some(_)) => format!("{scheme}://{}", self.path_prefix),
        }
    }

    pub fn is_secure(&self) -> bool {
        matches!(self.scheme, RegistryScheme::Https | RegistryScheme::File)
    }

    pub fn matches_source(&self, source: &str) -> Result<bool, String> {
        let candidate = normalize_registry_origin(source)?;
        if self.scheme != candidate.scheme
            || self.host != candidate.host
            || self.port != candidate.port
        {
            return Ok(false);
        }
        if self.path_prefix == "/" {
            return Ok(true);
        }
        Ok(candidate.path_prefix == self.path_prefix
            || candidate
                .path_prefix
                .strip_prefix(&self.path_prefix)
                .is_some_and(|suffix| suffix.starts_with('/')))
    }
}

#[allow(dead_code)]
const MAX_TRUSTED_REGISTRIES: usize = 64;

/// Deterministic bounded allowlist for registry origins.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TrustedRegistryPolicy {
    origins: BTreeSet<RegistryOrigin>,
}

#[allow(dead_code)]
impl TrustedRegistryPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, source: &str) -> Result<bool, String> {
        let origin = normalize_registry_origin(source)?;
        if self.origins.len() >= MAX_TRUSTED_REGISTRIES && !self.origins.contains(&origin) {
            return Err(format!(
                "trusted registry policy exceeds {} origins",
                MAX_TRUSTED_REGISTRIES
            ));
        }
        Ok(self.origins.insert(origin))
    }

    pub fn remove(&mut self, source: &str) -> Result<bool, String> {
        let origin = normalize_registry_origin(source)?;
        Ok(self.origins.remove(&origin))
    }

    pub fn is_trusted(&self, source: &str) -> Result<bool, String> {
        self.origins.iter().try_fold(false, |trusted, origin| {
            if trusted {
                Ok(true)
            } else {
                origin.matches_source(source)
            }
        })
    }

    pub fn origins(&self) -> impl Iterator<Item = &RegistryOrigin> {
        self.origins.iter()
    }

    /// Load an explicit allowlist from `ZAP_TRUSTED_REGISTRIES`.
    /// Entries are comma-separated canonicalizable registry URLs. An unset
    /// variable means no remote registry is trusted; local file sources retain
    /// their existing explicit-local behavior.
    pub fn from_environment() -> Result<Self, String> {
        let mut policy = Self::new();
        let Some(raw) = std::env::var_os("ZAP_TRUSTED_REGISTRIES") else {
            return Ok(policy);
        };
        let raw = raw
            .to_str()
            .ok_or_else(|| "trusted registry policy is not valid UTF-8".to_string())?;
        if raw.len() > 16 * 1024 {
            return Err("trusted registry policy exceeds 16 KiB".to_string());
        }
        for entry in raw
            .split(',')
            .map(str::trim)
            .filter(|entry| !entry.is_empty())
        {
            policy.add(entry)?;
        }
        Ok(policy)
    }

    pub fn require_trusted(&self, source: &str) -> Result<(), String> {
        if !source.contains("://") || source.starts_with("file://") {
            return Ok(());
        }
        let origin = normalize_registry_origin(source)?;
        if matches!(origin.scheme, RegistryScheme::File) {
            return Ok(());
        }
        if self.is_trusted(source)? {
            Ok(())
        } else {
            Err(format!("registry is not trusted: {}", origin.as_url()))
        }
    }
}

/// Origin-scoped bearer credential store.
///
/// Tokens are never included in `Debug` output or error messages. Entries are
/// keyed by canonical origins and resolution prefers the longest matching path
/// prefix, which keeps credential selection deterministic.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct RegistryCredentialStore {
    entries: BTreeMap<RegistryOrigin, String>,
}

#[allow(dead_code)]
impl RegistryCredentialStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, source: &str, token: &str) -> Result<bool, String> {
        let origin = normalize_registry_origin(source)?;
        if !origin.is_secure() {
            return Err("registry credentials require HTTPS or a local file origin".to_string());
        }
        validate_registry_token(token)?;
        Ok(self.entries.insert(origin, token.to_string()).is_none())
    }

    pub fn remove(&mut self, source: &str) -> Result<bool, String> {
        let origin = normalize_registry_origin(source)?;
        Ok(self.entries.remove(&origin).is_some())
    }

    pub fn origins(&self) -> impl Iterator<Item = &RegistryOrigin> {
        self.entries.keys()
    }

    pub fn resolve(&self, source: &str) -> Result<Option<&str>, String> {
        let candidate = normalize_registry_origin(source)?;
        if candidate.scheme == RegistryScheme::Http {
            return Ok(None);
        }
        self.entries
            .iter()
            .filter(|(origin, _)| origin.matches_source(source).unwrap_or(false))
            .max_by_key(|(origin, _)| origin.path_prefix.len())
            .map(|(_, token)| token.as_str())
            .map(Some)
            .ok_or_else(|| {
                if candidate.is_secure() {
                    "registry credential is not configured".to_string()
                } else {
                    "registry credentials require HTTPS or a local file origin".to_string()
                }
            })
            .or_else(|error| {
                if error == "registry credential is not configured" {
                    Ok(None)
                } else {
                    Err(error)
                }
            })
    }
}

pub fn registry_policy_config_path() -> PathBuf {
    if let Some(path) = std::env::var_os("ZAP_REGISTRY_CONFIG") {
        return PathBuf::from(path);
    }
    if let Some(home) = std::env::var_os("HOME") {
        return PathBuf::from(home).join(".config/zap/registry.json");
    }
    PathBuf::from(".zap/registry.json")
}

pub fn load_effective_trusted_registry_policy() -> Result<TrustedRegistryPolicy, String> {
    if std::env::var_os("ZAP_TRUSTED_REGISTRIES").is_some() {
        TrustedRegistryPolicy::from_environment()
    } else {
        load_trusted_registry_policy()
    }
}

fn load_registry_config_root() -> Result<Value, String> {
    let path = registry_policy_config_path();
    if !path.exists() {
        return Ok(serde_json::json!({}));
    }
    let metadata =
        fs::metadata(&path).map_err(|e| format!("registry config metadata failed: {e}"))?;
    if metadata.len() > 64 * 1024 {
        return Err("registry config exceeds 64 KiB".to_string());
    }
    let bytes = fs::read(&path).map_err(|e| format!("registry config read failed: {e}"))?;
    serde_json::from_slice(&bytes).map_err(|e| format!("registry config JSON is invalid: {e}"))
}

pub fn load_trusted_registry_policy() -> Result<TrustedRegistryPolicy, String> {
    let root = load_registry_config_root()?;
    let entries = root
        .get("trusted_registries")
        .and_then(Value::as_array)
        .map(|entries| entries.as_slice())
        .unwrap_or(&[]);
    let mut policy = TrustedRegistryPolicy::new();
    for entry in entries {
        let source = entry
            .as_str()
            .ok_or_else(|| "trusted registry entries must be strings".to_string())?;
        policy.add(source)?;
    }
    Ok(policy)
}

pub fn save_trusted_registry_policy(policy: &TrustedRegistryPolicy) -> Result<(), String> {
    let mut root = load_registry_config_root()?;
    let entries = policy
        .origins()
        .map(RegistryOrigin::as_url)
        .collect::<Vec<_>>();
    root["trusted_registries"] = serde_json::json!(entries);
    save_registry_config_root(&root)
}

pub fn load_registry_credentials() -> Result<RegistryCredentialStore, String> {
    let root = load_registry_config_root()?;
    let entries = root
        .get("credentials")
        .and_then(Value::as_array)
        .map(|entries| entries.as_slice())
        .unwrap_or(&[]);
    let mut credentials = RegistryCredentialStore::new();
    for entry in entries {
        let object = entry
            .as_object()
            .ok_or_else(|| "registry credential entries must be objects".to_string())?;
        let origin = object
            .get("origin")
            .and_then(Value::as_str)
            .ok_or_else(|| "registry credential origin must be a string".to_string())?;
        let token = object
            .get("token")
            .and_then(Value::as_str)
            .ok_or_else(|| "registry credential token must be a string".to_string())?;
        credentials.insert(origin, token)?;
    }
    Ok(credentials)
}

pub fn save_registry_credentials(credentials: &RegistryCredentialStore) -> Result<(), String> {
    let mut root = load_registry_config_root()?;
    let entries = credentials
        .entries
        .iter()
        .map(|(origin, token)| serde_json::json!({"origin": origin.as_url(), "token": token}))
        .collect::<Vec<_>>();
    root["credentials"] = serde_json::json!(entries);
    save_registry_config_root(&root)
}

fn save_registry_config_root(root: &Value) -> Result<(), String> {
    let path = registry_policy_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("registry config directory failed: {e}"))?;
    }
    let bytes = serde_json::to_vec_pretty(root)
        .map_err(|e| format!("registry config serialization failed: {e}"))?;
    if bytes.len() > 64 * 1024 {
        return Err("registry config exceeds 64 KiB".to_string());
    }
    let temporary = path.with_extension("json.tmp");
    fs::write(&temporary, bytes).map_err(|e| format!("registry config write failed: {e}"))?;
    fs::rename(&temporary, &path).map_err(|e| format!("registry config commit failed: {e}"))?;
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegistryAuthFailure {
    MissingCredentials,
    InvalidCredentials,
    InsufficientPermissions,
}

#[allow(dead_code)]
impl RegistryAuthFailure {
    pub fn code(self) -> &'static str {
        match self {
            Self::MissingCredentials => "ZAP-REG-AUTH-001",
            Self::InvalidCredentials => "ZAP-REG-AUTH-002",
            Self::InsufficientPermissions => "ZAP-REG-AUTH-003",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::MissingCredentials => "credentials required",
            Self::InvalidCredentials => "credentials rejected",
            Self::InsufficientPermissions => "permission denied",
        }
    }
}

fn registry_auth_failure_message(status: u16, source: &str, token: Option<&str>) -> Option<String> {
    let failure = match status {
        401 if token.is_some() => RegistryAuthFailure::InvalidCredentials,
        401 => RegistryAuthFailure::MissingCredentials,
        403 => RegistryAuthFailure::InsufficientPermissions,
        _ => return None,
    };
    let origin = normalize_registry_origin(source)
        .map(|origin| origin.as_url())
        .unwrap_or_else(|_| "registry origin".to_string());
    Some(format!(
        "registry authentication error [{}]: {} for {}",
        failure.code(),
        failure.label(),
        origin
    ))
}

fn validate_registry_token(token: &str) -> Result<(), String> {
    if token.is_empty()
        || token.len() > 4096
        || token.chars().any(|c| c.is_control() || c.is_whitespace())
    {
        return Err("registry authentication token is invalid".to_string());
    }
    Ok(())
}

/// Resolve a token with deterministic precedence: explicit argument, scoped
/// credential store, then `ZAP_REGISTRY_TOKEN` for HTTPS requests.
pub fn resolve_registry_token(
    source: &str,
    explicit: Option<&str>,
    credentials: &RegistryCredentialStore,
) -> Result<Option<String>, String> {
    let origin = normalize_registry_origin(source)?;
    if let Some(token) = explicit {
        validate_registry_token(token)?;
        if !origin.is_secure() {
            return Err("registry credentials require HTTPS or a local file origin".to_string());
        }
        return Ok(Some(token.to_string()));
    }
    if let Some(token) = credentials.resolve(source)? {
        return Ok(Some(token.to_string()));
    }
    let Some(token) = std::env::var_os("ZAP_REGISTRY_TOKEN") else {
        return Ok(None);
    };
    let token = token
        .to_str()
        .ok_or_else(|| "registry authentication token is not valid UTF-8".to_string())?;
    validate_registry_token(token)?;
    if !origin.is_secure() {
        return Err("registry credentials require HTTPS or a local file origin".to_string());
    }
    Ok(Some(token.to_string()))
}

/// Redact a secret from a diagnostic before it is shown to a user or written
/// to a log. This is intentionally simple and deterministic for all text.
pub fn redact_registry_secret(message: &str, secret: Option<&str>) -> String {
    match secret.filter(|value| !value.is_empty()) {
        Some(value) => message.replace(value, "<redacted>"),
        None => message.to_string(),
    }
}

/// Normalize a registry URL into a deterministic origin.
///
/// This B1 primitive accepts only explicit `http://`, `https://`, and
/// `file://` URLs. Userinfo, query strings, fragments, control characters,
/// traversal segments, and malformed ports are rejected.
pub fn normalize_registry_origin(source: &str) -> Result<RegistryOrigin, String> {
    let source = source.trim();
    if source.is_empty() || source.chars().any(|character| character.is_control()) {
        return Err("registry URL is invalid".to_string());
    }
    let (scheme, remainder) = source
        .split_once("://")
        .ok_or_else(|| "registry URL must include an explicit scheme".to_string())?;
    let scheme = match scheme.to_ascii_lowercase().as_str() {
        "http" => RegistryScheme::Http,
        "https" => RegistryScheme::Https,
        "file" => RegistryScheme::File,
        _ => return Err("registry URL scheme is unsupported".to_string()),
    };
    if remainder.contains('?') || remainder.contains('#') || remainder.contains('\\') {
        return Err("registry URL contains unsupported query, fragment, or separator".to_string());
    }
    if matches!(scheme, RegistryScheme::File) {
        if remainder.starts_with('@') {
            return Err("registry file URL must not contain credentials".to_string());
        }
        return Ok(RegistryOrigin {
            scheme,
            host: None,
            port: None,
            path_prefix: normalize_registry_path(remainder)?,
        });
    }
    let (authority, path) = remainder.split_once('/').unwrap_or((remainder, ""));
    if authority.is_empty() || authority.contains('@') {
        return Err("registry URL authority is invalid".to_string());
    }
    let (host, port) = parse_registry_authority(authority)?;
    let port = match (scheme, port) {
        (RegistryScheme::Http, Some(80)) | (RegistryScheme::Https, Some(443)) => None,
        (_, value) => value,
    };
    Ok(RegistryOrigin {
        scheme,
        host: Some(host),
        port,
        path_prefix: normalize_registry_path(path)?,
    })
}

fn parse_registry_authority(authority: &str) -> Result<(String, Option<u16>), String> {
    if authority.starts_with('[') {
        let end = authority
            .find(']')
            .ok_or_else(|| "registry URL host is invalid".to_string())?;
        let host = authority[1..end].to_ascii_lowercase();
        if host.is_empty() {
            return Err("registry URL host is invalid".to_string());
        }
        let port = if end + 1 == authority.len() {
            None
        } else {
            let suffix = authority[end + 1..]
                .strip_prefix(':')
                .ok_or_else(|| "registry URL port is invalid".to_string())?;
            Some(parse_registry_port(suffix)?)
        };
        return Ok((format!("[{host}]"), port));
    }
    let mut parts = authority.split(':');
    let host = parts.next().unwrap_or_default();
    if host.is_empty() || parts.clone().count() > 1 {
        return Err("registry URL host is invalid".to_string());
    }
    let port = parts.next().map(parse_registry_port).transpose()?;
    Ok((host.to_ascii_lowercase(), port))
}

fn parse_registry_port(value: &str) -> Result<u16, String> {
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err("registry URL port is invalid".to_string());
    }
    value
        .parse::<u16>()
        .map_err(|_| "registry URL port is invalid".to_string())
}

fn normalize_registry_path(path: &str) -> Result<String, String> {
    let mut segments = Vec::new();
    for segment in path.split('/') {
        match segment {
            "" | "." => {}
            ".." => return Err("registry URL path traversal is invalid".to_string()),
            value if value.chars().any(char::is_whitespace) => {
                return Err("registry URL path contains whitespace".to_string())
            }
            value => segments.push(value),
        }
    }
    if segments.is_empty() {
        Ok("/".to_string())
    } else {
        Ok(format!("/{}", segments.join("/")))
    }
}

/// Read a registry index from a local file, `file://` URL, or HTTP(S) URL.
/// Remote access is deterministic at the response-byte level and is restricted
/// to HTTPS unless `ZAP_ALLOW_INSECURE_HTTP=1` is explicitly set for fixtures.
pub fn read_index_source(source: &str) -> Result<Vec<RegistryPackage>, String> {
    read_index_source_with_credentials(source, &RegistryCredentialStore::new())
}

pub fn read_index_source_with_credentials(
    source: &str,
    credentials: &RegistryCredentialStore,
) -> Result<Vec<RegistryPackage>, String> {
    parse_index_bytes(&fetch_source_with_credentials(source, credentials)?)
}

pub(crate) fn parse_index_bytes(bytes: &[u8]) -> Result<Vec<RegistryPackage>, String> {
    let root: Value = serde_json::from_slice(bytes)
        .map_err(|e| format!("registry index JSON is invalid: {e}"))?;
    let entries = root
        .get("packages")
        .and_then(Value::as_array)
        .ok_or_else(|| "registry index must contain a packages array".to_string())?;
    parse_packages(entries)
}

fn parse_packages(entries: &[Value]) -> Result<Vec<RegistryPackage>, String> {
    let mut packages = entries
        .iter()
        .map(parse_package)
        .collect::<Result<Vec<_>, _>>()?;
    packages.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then_with(|| left.version.cmp(&right.version))
            .then_with(|| left.checksum.cmp(&right.checksum))
    });
    for pair in packages.windows(2) {
        if pair[0].name == pair[1].name && pair[0].version == pair[1].version {
            return Err(format!(
                "registry index contains duplicate package: {} {}",
                pair[0].name, pair[0].version
            ));
        }
    }
    Ok(packages)
}

pub fn find_package(
    index: &[RegistryPackage],
    name: &str,
    version: &str,
) -> Result<RegistryPackage, String> {
    if let Some(package) = index
        .iter()
        .find(|package| package.name == name && package.version == version && !package.yanked)
        .cloned()
    {
        return Ok(package);
    }
    if index
        .iter()
        .any(|package| package.name == name && package.version == version && package.yanked)
    {
        return Err(format!("registry package is yanked: {name} {version}"));
    }
    Err(format!("registry package not found: {name} {version}"))
}

/// Check whether a concrete version satisfies a requirement without applying
/// yanked-release selection policy. Locked installs use this semantic check
/// while allowing an already-cached yanked artifact.
pub fn version_satisfies_requirement(version: &str, requirement: &str) -> Result<bool, String> {
    let version = Version::parse(version)?;
    Ok(VersionRequirement::parse(requirement)?.matches(version))
}

/// Select the highest registry version satisfying a deterministic requirement.
/// Supported forms are exact versions, partial versions (`1` or `1.2`),
/// caret/tilde requirements, and comma-separated comparison clauses.
pub fn find_package_requirement(
    index: &[RegistryPackage],
    name: &str,
    requirement: &str,
) -> Result<RegistryPackage, String> {
    let requirement = VersionRequirement::parse(requirement)?;
    let matching_yanked = index
        .iter()
        .filter(|package| package.name == name && package.yanked)
        .filter_map(|package| {
            Version::parse(&package.version)
                .ok()
                .filter(|version| requirement.matches(*version))
        })
        .count();
    index
        .iter()
        .filter(|package| package.name == name && !package.yanked)
        .filter_map(|package| {
            Version::parse(&package.version)
                .ok()
                .filter(|version| requirement.matches(*version))
                .map(|version| (version, package))
        })
        .max_by(
            |(left_version, left_package), (right_version, right_package)| {
                left_version
                    .cmp(right_version)
                    .then_with(|| left_package.version.cmp(&right_package.version))
            },
        )
        .map(|(_, package)| package.clone())
        .ok_or_else(|| {
            if matching_yanked > 0 {
                format!("all matching registry packages are yanked: {name} {requirement}")
            } else {
                format!("registry package does not satisfy requirement: {name} {requirement}")
            }
        })
}

/// Resolve registry dependency requirements recursively.
///
/// Dependencies are visited in lexical order, each requirement selects the
/// highest compatible version, and a package name may resolve to only one
/// version in the graph. Cycles and incompatible repeated requirements produce
/// deterministic diagnostics.
pub fn resolve_dependency_graph(
    index: &[RegistryPackage],
    roots: &BTreeMap<String, String>,
) -> Result<Vec<RegistryPackage>, String> {
    let mut selected = BTreeMap::new();
    let mut active = Vec::new();
    for (name, requirement) in roots {
        resolve_dependency(index, name, requirement, &mut selected, &mut active)?;
    }
    Ok(selected.into_values().collect())
}

fn resolve_dependency(
    index: &[RegistryPackage],
    name: &str,
    requirement: &str,
    selected: &mut BTreeMap<String, RegistryPackage>,
    active: &mut Vec<String>,
) -> Result<(), String> {
    let parsed_requirement = VersionRequirement::parse(requirement)?;
    if let Some(position) = active.iter().position(|entry| entry == name) {
        let mut cycle = active[position..].to_vec();
        cycle.push(name.to_string());
        return Err(format!(
            "registry dependency cycle detected: {}",
            cycle.join(" -> ")
        ));
    }
    if let Some(package) = selected.get(name) {
        let version = Version::parse(&package.version)?;
        if parsed_requirement.matches(version) {
            return Ok(());
        }
        return Err(format!(
            "registry dependency version conflict for {name}: selected {} does not satisfy {requirement}",
            package.version
        ));
    }
    let package = find_package_requirement(index, name, requirement)?;
    selected.insert(name.to_string(), package.clone());
    active.push(name.to_string());
    for (dependency_name, dependency_requirement) in &package.dependencies {
        if let Err(error) = resolve_dependency(
            index,
            dependency_name,
            dependency_requirement,
            selected,
            active,
        ) {
            active.pop();
            return Err(error);
        }
    }
    active.pop();
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Version {
    major: u64,
    minor: u64,
    patch: u64,
}

impl Version {
    fn parse(raw: &str) -> Result<Self, String> {
        let core = raw.split(['-', '+']).next().unwrap_or(raw);
        let parts = core.split('.').collect::<Vec<_>>();
        if parts.is_empty() || parts.len() > 3 || parts.iter().any(|part| part.is_empty()) {
            return Err(format!("invalid registry version: {raw}"));
        }
        let mut values = parts
            .iter()
            .map(|part| {
                part.parse::<u64>()
                    .map_err(|_| format!("invalid registry version: {raw}"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        while values.len() < 3 {
            values.push(0);
        }
        Ok(Self {
            major: values[0],
            minor: values[1],
            patch: values[2],
        })
    }
}

#[derive(Clone, Debug)]
struct VersionRequirement {
    clauses: Vec<VersionClause>,
    display: String,
}

#[derive(Clone, Copy, Debug)]
enum VersionClause {
    Exact(Version),
    Greater(Version),
    GreaterOrEqual(Version),
    Less(Version),
    LessOrEqual(Version),
}

impl VersionRequirement {
    fn parse(raw: &str) -> Result<Self, String> {
        let raw = raw.trim();
        if raw.is_empty() {
            return Err("registry version requirement must not be empty".to_string());
        }
        let mut clauses = Vec::new();
        for token in raw
            .split(',')
            .map(str::trim)
            .filter(|token| !token.is_empty())
        {
            let (operator, version_text) = if let Some(value) = token.strip_prefix(">=") {
                (">=", value)
            } else if let Some(value) = token.strip_prefix("<=") {
                ("<=", value)
            } else if let Some(value) = token.strip_prefix('>') {
                (">", value)
            } else if let Some(value) = token.strip_prefix('<') {
                ("<", value)
            } else if let Some(value) = token.strip_prefix('^') {
                let version = Version::parse(value.trim())?;
                clauses.push(VersionClause::GreaterOrEqual(version));
                clauses.push(VersionClause::Less(Self::caret_upper(version)));
                continue;
            } else if let Some(value) = token.strip_prefix('~') {
                let version = Version::parse(value.trim())?;
                clauses.push(VersionClause::GreaterOrEqual(version));
                clauses.push(VersionClause::Less(Self::tilde_upper(version)));
                continue;
            } else {
                ("", token)
            };
            let version_text = version_text.trim();
            let version = Version::parse(version_text)?;
            clauses.push(match operator {
                ">" => VersionClause::Greater(version),
                ">=" => VersionClause::GreaterOrEqual(version),
                "<" => VersionClause::Less(version),
                "<=" => VersionClause::LessOrEqual(version),
                _ => VersionClause::Exact(version),
            });
        }
        if clauses.is_empty() {
            return Err(format!("invalid registry version requirement: {raw}"));
        }
        Ok(Self {
            clauses,
            display: raw.to_string(),
        })
    }

    fn matches(&self, version: Version) -> bool {
        self.clauses.iter().all(|clause| match clause {
            VersionClause::Exact(expected) => version == *expected,
            VersionClause::Greater(expected) => version > *expected,
            VersionClause::GreaterOrEqual(expected) => version >= *expected,
            VersionClause::Less(expected) => version < *expected,
            VersionClause::LessOrEqual(expected) => version <= *expected,
        })
    }

    fn caret_upper(version: Version) -> Version {
        if version.major > 0 {
            Version {
                major: version.major + 1,
                minor: 0,
                patch: 0,
            }
        } else if version.minor > 0 {
            Version {
                major: 0,
                minor: version.minor + 1,
                patch: 0,
            }
        } else {
            Version {
                major: 0,
                minor: 0,
                patch: version.patch + 1,
            }
        }
    }

    fn tilde_upper(version: Version) -> Version {
        Version {
            major: version.major,
            minor: version.minor + 1,
            patch: 0,
        }
    }
}

impl std::fmt::Display for VersionRequirement {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.display)
    }
}

pub fn cache_package(
    source: &Path,
    cache_root: &Path,
    package: &RegistryPackage,
) -> Result<PathBuf, String> {
    let bytes = fs::read(source).map_err(|e| format!("package source read failed: {e}"))?;
    cache_bytes(&bytes, cache_root, package)
}

/// Fetch and cache an artifact from a local or HTTP(S) source after verifying SHA-256.
pub fn cache_package_source(
    source: &str,
    cache_root: &Path,
    package: &RegistryPackage,
) -> Result<PathBuf, String> {
    cache_package_source_with_credentials(
        source,
        cache_root,
        package,
        &RegistryCredentialStore::new(),
    )
}

pub fn cache_package_source_with_credentials(
    source: &str,
    cache_root: &Path,
    package: &RegistryPackage,
    credentials: &RegistryCredentialStore,
) -> Result<PathBuf, String> {
    cache_bytes(
        &fetch_source_with_credentials(source, credentials)?,
        cache_root,
        package,
    )
}

fn cache_bytes(
    bytes: &[u8],
    cache_root: &Path,
    package: &RegistryPackage,
) -> Result<PathBuf, String> {
    validate_package_identity(package)?;
    let actual = sha256_hex(bytes);
    if actual != package.checksum {
        return Err(format!(
            "package checksum mismatch for {} {}: expected {}, got {}",
            package.name, package.version, package.checksum, actual
        ));
    }
    let directory = cache_root.join(&package.name).join(&package.version);
    fs::create_dir_all(&directory).map_err(|e| format!("package cache directory failed: {e}"))?;
    let destination = directory.join(format!("{}.pkg", package.checksum));
    let temporary = directory.join(format!("{}.pkg.tmp", package.checksum));
    fs::write(&temporary, bytes).map_err(|e| format!("package cache write failed: {e}"))?;
    fs::rename(&temporary, &destination)
        .map_err(|e| format!("package cache commit failed: {e}"))?;
    Ok(destination)
}

pub fn verify_cached_package(path: &Path, package: &RegistryPackage) -> Result<(), String> {
    let bytes = fs::read(path).map_err(|e| format!("cached package read failed: {e}"))?;
    let actual = sha256_hex(&bytes);
    if actual != package.checksum {
        return Err(format!(
            "cached package checksum mismatch for {} {}: expected {}, got {}",
            package.name, package.version, package.checksum, actual
        ));
    }
    Ok(())
}

fn parse_package(value: &Value) -> Result<RegistryPackage, String> {
    let name = required_string(value, "name")?;
    let version = required_string(value, "version")?;
    let source = required_string(value, "source")?;
    let checksum = required_string(value, "checksum")?.to_ascii_lowercase();
    if !is_sha256(&checksum) {
        return Err(format!("registry checksum is invalid for {name} {version}"));
    }
    let yanked = match value.get("yanked") {
        None => false,
        Some(raw) => raw.as_bool().ok_or_else(|| {
            format!("registry yanked field must be a boolean for {name} {version}")
        })?,
    };
    let dependencies = parse_registry_dependencies(value, &name, &version)?;
    let package = RegistryPackage {
        name,
        version,
        source,
        checksum,
        yanked,
        dependencies,
    };
    validate_package_identity(&package)?;
    Ok(package)
}

fn parse_registry_dependencies(
    value: &Value,
    package_name: &str,
    package_version: &str,
) -> Result<BTreeMap<String, String>, String> {
    let Some(raw_dependencies) = value.get("dependencies") else {
        return Ok(BTreeMap::new());
    };
    let object = raw_dependencies.as_object().ok_or_else(|| {
        format!("registry dependencies for {package_name} {package_version} must be an object")
    })?;
    let mut dependencies = BTreeMap::new();
    for (name, requirement) in object {
        validate_package_name(name).map_err(|error| {
            format!("registry dependency in {package_name} {package_version}: {error}")
        })?;
        let requirement = requirement.as_str().ok_or_else(|| {
            format!(
                "registry dependency `{name}` in {package_name} {package_version} must be a string"
            )
        })?;
        VersionRequirement::parse(requirement).map_err(|error| {
            format!("registry dependency `{name}` in {package_name} {package_version}: {error}")
        })?;
        if dependencies
            .insert(name.clone(), requirement.to_string())
            .is_some()
        {
            return Err(format!(
                "duplicate registry dependency `{name}` in {package_name} {package_version}"
            ));
        }
    }
    Ok(dependencies)
}

fn required_string(value: &Value, field: &str) -> Result<String, String> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|text| !text.trim().is_empty())
        .map(ToOwned::to_owned)
        .ok_or_else(|| format!("registry package field '{field}' must be a non-empty string"))
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut output = String::with_capacity(digest.len() * 2);
    for byte in digest {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}

/// Publish a verified package archive to a registry endpoint.
/// The endpoint receives the archive as the request body and package identity
/// as stable headers; authentication is supplied through `ZAP_REGISTRY_TOKEN`.
pub fn publish_package(
    registry_url: &str,
    archive: &Path,
    package: &RegistryPackage,
    token: Option<&str>,
) -> Result<(), String> {
    let agent = ureq::builder().build();
    publish_package_with_agent(&agent, registry_url, archive, package, token)
}

fn publish_package_with_agent(
    agent: &ureq::Agent,
    registry_url: &str,
    archive: &Path,
    package: &RegistryPackage,
    token: Option<&str>,
) -> Result<(), String> {
    require_secure_transport(registry_url)?;
    let token = resolve_registry_token(registry_url, token, &RegistryCredentialStore::new())?;
    let bytes = fs::read(archive).map_err(|e| format!("package archive read failed: {e}"))?;
    let actual = sha256_hex(&bytes);
    if actual != package.checksum {
        return Err(format!(
            "publish checksum mismatch for {} {}: expected {}, got {}",
            package.name, package.version, package.checksum, actual
        ));
    }
    let mut request = agent
        .post(registry_url)
        .set("Content-Type", "application/octet-stream")
        .set("X-Zap-Package-Name", &package.name)
        .set("X-Zap-Package-Version", &package.version)
        .set("X-Zap-Package-Checksum", &package.checksum);
    if let Some(token) = token.as_deref() {
        request = request.set("Authorization", &format!("Bearer {token}"));
    }
    let response = request.send_bytes(&bytes).map_err(|error| {
        let message = match error {
            ureq::Error::Status(status, _) => {
                registry_auth_failure_message(status, registry_url, token.as_deref())
                    .unwrap_or_else(|| format!("registry publish failed with HTTP {status}"))
            }
            other => format!("registry publish failed: {other}"),
        };
        redact_registry_secret(&message, token.as_deref())
    })?;
    if !(200..300).contains(&response.status()) {
        return Err(registry_auth_failure_message(
            response.status(),
            registry_url,
            token.as_deref(),
        )
        .unwrap_or_else(|| format!("registry publish failed with HTTP {}", response.status())));
    }
    Ok(())
}

/// Persist a verified package into a registry directory using a bearer token.
/// This models the server-side publish boundary without requiring a network server:
/// artifacts and the signed index are committed atomically to disk.
#[allow(dead_code)]
pub fn persist_registry_package(
    registry_root: &Path,
    archive: &Path,
    package: &RegistryPackage,
    token: Option<&str>,
    signing_secret: &[u8],
) -> Result<PathBuf, String> {
    if match token {
        None => true,
        Some(value) => value.trim().is_empty(),
    } {
        return Err("registry persistence requires an authentication token".to_string());
    }
    if signing_secret.is_empty() {
        return Err("registry persistence requires a signing secret".to_string());
    }
    let bytes = fs::read(archive).map_err(|e| format!("package archive read failed: {e}"))?;
    let actual = sha256_hex(&bytes);
    if actual != package.checksum {
        return Err(format!(
            "publish checksum mismatch for {} {}: expected {}, got {}",
            package.name, package.version, package.checksum, actual
        ));
    }

    let artifact_root = registry_root.join("packages");
    let artifact_path = package_cache_path(&artifact_root, package);
    if let Some(parent) = artifact_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("registry package directory failed: {e}"))?;
    }
    let artifact_tmp = artifact_path.with_extension("pkg.tmp");
    fs::write(&artifact_tmp, &bytes).map_err(|e| format!("registry package write failed: {e}"))?;
    fs::rename(&artifact_tmp, &artifact_path)
        .map_err(|e| format!("registry package commit failed: {e}"))?;

    let index_path = registry_root.join("index.json");
    let mut packages = if index_path.exists() {
        read_index(&index_path)?
    } else {
        Vec::new()
    };
    if let Some(existing) = packages
        .iter_mut()
        .find(|entry| entry.name == package.name && entry.version == package.version)
    {
        if existing.checksum != package.checksum {
            return Err(format!(
                "registry package already exists with a different checksum: {} {}",
                package.name, package.version
            ));
        }
        *existing = package.clone();
    } else {
        packages.push(package.clone());
    }
    packages.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then_with(|| {
                Version::parse(&left.version)
                    .ok()
                    .cmp(&Version::parse(&right.version).ok())
            })
            .then_with(|| left.version.cmp(&right.version))
    });
    let package_values = packages
        .iter()
        .map(|entry| {
            serde_json::json!({
                "name": entry.name,
                "version": entry.version,
                "source": entry.source,
                "checksum": entry.checksum,
                "dependencies": entry.dependencies,
            })
        })
        .collect::<Vec<_>>();
    let package_array = Value::Array(package_values);
    let canonical = serde_json::to_vec(&package_array)
        .map_err(|e| format!("registry index canonicalization failed: {e}"))?;
    let index = serde_json::json!({
        "signature": hmac_sha256_hex(signing_secret, &canonical),
        "packages": package_array,
    });
    let index_tmp = index_path.with_extension("json.tmp");
    let index_bytes = serde_json::to_vec_pretty(&index)
        .map_err(|e| format!("registry index serialization failed: {e}"))?;
    fs::write(&index_tmp, index_bytes).map_err(|e| format!("registry index write failed: {e}"))?;
    fs::rename(&index_tmp, &index_path)
        .map_err(|e| format!("registry index commit failed: {e}"))?;
    Ok(artifact_path)
}

/// Hard limits for the built-in registry service request parser.
const REGISTRY_SERVICE_MAX_HEADERS: usize = 64 * 1024;
const REGISTRY_SERVICE_MAX_BODY: usize = 16 * 1024 * 1024;

/// Serve a small authenticated HTTP registry endpoint using only the standard
/// library. The listener is non-blocking and exits when `stop` is cancelled.
/// `POST /publish` persists an authenticated package and atomically rewrites the
/// signed index; `GET /index.json` and safe `/packages/...` paths serve artifacts.
#[allow(dead_code)]
pub fn serve_registry(
    bind: &str,
    registry_root: PathBuf,
    token: String,
    signing_secret: Vec<u8>,
    stop: Arc<AtomicBool>,
) -> Result<(), String> {
    if token.trim().is_empty() {
        return Err("registry service requires a non-empty authentication token".to_string());
    }
    if signing_secret.is_empty() {
        return Err("registry service requires a non-empty signing secret".to_string());
    }
    fs::create_dir_all(&registry_root)
        .map_err(|error| format!("registry service root failed: {error}"))?;
    let listener = TcpListener::bind(bind)
        .map_err(|error| format!("registry service bind failed: {error}"))?;
    serve_registry_listener(listener, registry_root, token, signing_secret, stop)
}

fn serve_registry_listener(
    listener: TcpListener,
    registry_root: PathBuf,
    token: String,
    signing_secret: Vec<u8>,
    stop: Arc<AtomicBool>,
) -> Result<(), String> {
    listener
        .set_nonblocking(true)
        .map_err(|error| format!("registry service non-blocking setup failed: {error}"))?;
    while !stop.load(Ordering::Acquire) {
        match listener.accept() {
            Ok((stream, _)) => {
                let _ = handle_registry_connection(stream, &registry_root, &token, &signing_secret);
            }
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(2));
            }
            Err(error) => return Err(format!("registry service accept failed: {error}")),
        }
    }
    Ok(())
}

/// A managed registry service suitable for local deployment and integration
/// tests. Dropping it does not silently leave a worker running; call `stop` to
/// request shutdown and join the service thread.
#[allow(dead_code)]
pub struct RegistryService {
    address: SocketAddr,
    stop: Arc<AtomicBool>,
    join: Option<JoinHandle<Result<(), String>>>,
}

#[allow(dead_code)]
impl RegistryService {
    pub fn start(
        bind: &str,
        registry_root: PathBuf,
        token: String,
        signing_secret: Vec<u8>,
    ) -> Result<Self, String> {
        if token.trim().is_empty() {
            return Err("registry service requires a non-empty authentication token".to_string());
        }
        if signing_secret.is_empty() {
            return Err("registry service requires a non-empty signing secret".to_string());
        }
        fs::create_dir_all(&registry_root)
            .map_err(|error| format!("registry service root failed: {error}"))?;
        let listener = TcpListener::bind(bind)
            .map_err(|error| format!("registry service bind failed: {error}"))?;
        let address = listener
            .local_addr()
            .map_err(|error| format!("registry service address failed: {error}"))?;
        let stop = Arc::new(AtomicBool::new(false));
        let worker_stop = Arc::clone(&stop);
        let join = std::thread::spawn(move || {
            serve_registry_listener(listener, registry_root, token, signing_secret, worker_stop)
        });
        Ok(Self {
            address,
            stop,
            join: Some(join),
        })
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub fn stop(mut self) -> Result<(), String> {
        self.stop.store(true, Ordering::Release);
        self.join
            .take()
            .ok_or_else(|| "registry service already stopped".to_string())?
            .join()
            .map_err(|_| "registry service worker panicked".to_string())?
    }
}

impl Drop for RegistryService {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

fn handle_registry_connection(
    mut stream: TcpStream,
    registry_root: &Path,
    token: &str,
    signing_secret: &[u8],
) -> Result<(), String> {
    // The listener is intentionally non-blocking for its accept loop. Some
    // Unix targets can propagate that mode to accepted sockets, so normalize
    // each connection before the request parser performs blocking reads.
    stream
        .set_nonblocking(false)
        .map_err(|error| format!("registry connection blocking-mode setup failed: {error}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .map_err(|error| format!("registry connection timeout failed: {error}"))?;
    let mut request = Vec::new();
    let mut buffer = [0_u8; 4096];
    let header_end = loop {
        let count = stream
            .read(&mut buffer)
            .map_err(|error| format!("registry request read failed: {error}"))?;
        if count == 0 {
            return Err("registry request ended before headers".to_string());
        }
        request.extend_from_slice(&buffer[..count]);
        if request.len() > REGISTRY_SERVICE_MAX_HEADERS + REGISTRY_SERVICE_MAX_BODY {
            return write_registry_response(&mut stream, 413, b"request too large");
        }
        if let Some(position) = request.windows(4).position(|window| window == b"\r\n\r\n") {
            break position + 4;
        }
        if request.len() > REGISTRY_SERVICE_MAX_HEADERS {
            return write_registry_response(&mut stream, 431, b"request headers too large");
        }
    };
    let header_text = std::str::from_utf8(&request[..header_end])
        .map_err(|_| "registry request headers are not UTF-8".to_string())?;
    let mut lines = header_text.split("\r\n");
    let request_line = lines
        .next()
        .ok_or_else(|| "registry request line is missing".to_string())?;
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts.next().unwrap_or_default().to_owned();
    let path = request_parts.next().unwrap_or_default().to_owned();
    let mut content_length = 0_usize;
    let mut authorization = None;
    let mut package_name = None;
    let mut package_version = None;
    let mut package_checksum = None;
    for line in lines {
        if line.is_empty() {
            break;
        }
        let Some((name, value)) = line.split_once(':') else {
            return write_registry_response(&mut stream, 400, b"malformed header");
        };
        let value = value.trim();
        match name.to_ascii_lowercase().as_str() {
            "content-length" => {
                content_length = value
                    .parse()
                    .map_err(|_| "invalid content length".to_string())?;
            }
            "authorization" => authorization = Some(value.to_string()),
            "x-zap-package-name" => package_name = Some(value.to_string()),
            "x-zap-package-version" => package_version = Some(value.to_string()),
            "x-zap-package-checksum" => package_checksum = Some(value.to_string()),
            _ => {}
        }
    }
    if content_length > REGISTRY_SERVICE_MAX_BODY {
        return write_registry_response(&mut stream, 413, b"request body too large");
    }
    while request.len() - header_end < content_length {
        let count = stream
            .read(&mut buffer)
            .map_err(|error| format!("registry request body read failed: {error}"))?;
        if count == 0 {
            return write_registry_response(&mut stream, 400, b"request body is incomplete");
        }
        request.extend_from_slice(&buffer[..count]);
    }
    let authorized = authorization
        .as_deref()
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(|candidate| constant_time_equal(candidate.as_bytes(), token.as_bytes()))
        .unwrap_or(false);
    if !authorized {
        return write_registry_response(&mut stream, 401, b"registry authentication failed");
    }
    match method.as_str() {
        "POST" if path == "/publish" || path == "/" => {
            let name = package_name.ok_or_else(|| "package name header is missing".to_string())?;
            let version =
                package_version.ok_or_else(|| "package version header is missing".to_string())?;
            let checksum =
                package_checksum.ok_or_else(|| "package checksum header is missing".to_string())?;
            if !is_safe_registry_segment(&name) || !is_safe_registry_segment(&version) {
                return write_registry_response(&mut stream, 400, b"invalid package identity");
            }
            let body = request[header_end..header_end + content_length].to_vec();
            let package = RegistryPackage {
                name,
                version,
                source: path.to_string(),
                checksum,
                yanked: false,
                dependencies: BTreeMap::new(),
            };
            let temporary = registry_root.join(".zap-registry-request.pkg");
            fs::write(&temporary, body)
                .map_err(|error| format!("registry request staging failed: {error}"))?;
            let result = persist_registry_package(
                registry_root,
                &temporary,
                &package,
                Some(token),
                signing_secret,
            );
            let _ = fs::remove_file(&temporary);
            match result {
                Ok(_) => write_registry_response(&mut stream, 201, b"published"),
                Err(error) => write_registry_response(&mut stream, 422, error.as_bytes()),
            }
        }
        "GET" => {
            let relative = path.strip_prefix('/').unwrap_or(&path);
            if relative.is_empty() || relative.contains("..") {
                return write_registry_response(&mut stream, 400, b"invalid registry path");
            }
            let requested = registry_root.join(relative);
            if !is_safe_registry_path(registry_root, &requested) {
                return write_registry_response(&mut stream, 400, b"invalid registry path");
            }
            match fs::read(requested) {
                Ok(bytes) => write_registry_response(&mut stream, 200, &bytes),
                Err(_) => write_registry_response(&mut stream, 404, b"not found"),
            }
        }
        _ => write_registry_response(&mut stream, 405, b"method not allowed"),
    }
}

fn write_registry_response(stream: &mut TcpStream, status: u16, body: &[u8]) -> Result<(), String> {
    let reason = match status {
        200 => "OK",
        201 => "Created",
        400 => "Bad Request",
        401 => "Unauthorized",
        404 => "Not Found",
        405 => "Method Not Allowed",
        413 => "Payload Too Large",
        422 => "Unprocessable Entity",
        431 => "Request Header Fields Too Large",
        _ => "Error",
    };
    write!(
        stream,
        "HTTP/1.1 {status} {reason}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )
    .and_then(|_| stream.write_all(body))
    .map_err(|error| format!("registry response write failed: {error}"))
}

fn is_safe_registry_segment(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value != "."
        && value != ".."
        && !value.contains('/')
        && !value.contains('\\')
        && !value.chars().any(|character| character.is_control())
}

fn is_safe_registry_path(root: &Path, requested: &Path) -> bool {
    let relative = requested.strip_prefix(root).ok();
    relative
        .map(|path| {
            path.components()
                .all(|component| matches!(component, Component::Normal(value) if !value.is_empty()))
        })
        .unwrap_or(false)
}

fn fetch_source_with_credentials(
    source: &str,
    credentials: &RegistryCredentialStore,
) -> Result<Vec<u8>, String> {
    let agent = ureq::builder().build();
    fetch_source_with_agent(&agent, source, credentials)
}

fn fetch_source_with_agent(
    agent: &ureq::Agent,
    source: &str,
    credentials: &RegistryCredentialStore,
) -> Result<Vec<u8>, String> {
    let untrusted = std::env::var("ZAP_UNTRUSTED").as_deref() == Ok("1");
    if let Some(path) = source.strip_prefix("file://") {
        if untrusted {
            return Err(
                "local registry sources are disabled in untrusted mode; use an approved remote registry"
                    .into(),
            );
        }
        return fs::read(path).map_err(|e| format!("registry source read failed: {e}"));
    }
    if source.starts_with("http://") || source.starts_with("https://") {
        require_secure_transport(source)?;
        let token = resolve_registry_token(source, None, credentials)?;
        let mut request = agent.get(source);
        if let Some(token) = token.as_deref() {
            request = request.set("Authorization", &format!("Bearer {token}"));
        }
        let response = request.call().map_err(|error| {
            let message = match error {
                ureq::Error::Status(status, _) => {
                    registry_auth_failure_message(status, source, token.as_deref())
                        .unwrap_or_else(|| format!("registry HTTP fetch failed with HTTP {status}"))
                }
                other => format!("registry HTTP fetch failed: {other}"),
            };
            redact_registry_secret(&message, token.as_deref())
        })?;
        let mut bytes = Vec::new();
        response
            .into_reader()
            .read_to_end(&mut bytes)
            .map_err(|e| format!("registry response read failed: {e}"))?;
        return Ok(bytes);
    }
    if untrusted {
        return Err(
            "bare local registry sources are disabled in untrusted mode; use an approved remote registry"
                .into(),
        );
    }
    fs::read(source).map_err(|e| format!("registry source read failed: {e}"))
}

fn require_secure_transport(source: &str) -> Result<(), String> {
    let origin = normalize_registry_origin(source)?;
    if origin.scheme == RegistryScheme::Http
        && std::env::var("ZAP_ALLOW_INSECURE_HTTP").as_deref() != Ok("1")
    {
        return Err("insecure HTTP registry transport is disabled; use HTTPS or set ZAP_ALLOW_INSECURE_HTTP=1 for local fixtures".to_string());
    }
    if !matches!(origin.scheme, RegistryScheme::Http | RegistryScheme::Https) {
        return Err(format!(
            "registry publish requires an HTTP(S) URL: {source}"
        ));
    }
    Ok(())
}

pub fn package_cache_path(cache_root: &Path, package: &RegistryPackage) -> PathBuf {
    cache_root
        .join(&package.name)
        .join(&package.version)
        .join(format!("{}.pkg", package.checksum))
}

pub(crate) fn validate_package_name(name: &str) -> Result<(), String> {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return Err("package name must not be empty".to_string());
    };
    if !first.is_ascii_alphanumeric()
        || !chars.all(|character| {
            character.is_ascii_alphanumeric() || character == '-' || character == '_'
        })
    {
        return Err(format!("invalid package name `{name}`"));
    }
    Ok(())
}

fn validate_package_version(version: &str) -> Result<(), String> {
    let core = version.split(['-', '+']).next().unwrap_or(version);
    let parts = core.split('.').collect::<Vec<_>>();
    if parts.is_empty()
        || parts.len() > 3
        || parts.iter().any(|part| {
            part.is_empty()
                || (part.len() > 1 && part.starts_with('0'))
                || !part.bytes().all(|byte| byte.is_ascii_digit())
        })
    {
        return Err(format!("invalid registry version: {version}"));
    }
    if version
        .chars()
        .any(|character| character.is_control() || character == '/' || character == '\\')
    {
        return Err(format!("invalid registry version: {version}"));
    }
    Ok(())
}

fn validate_package_identity(package: &RegistryPackage) -> Result<(), String> {
    validate_package_name(&package.name)?;
    validate_package_version(&package.version)?;
    if package.checksum.len() != 64 || !is_sha256(&package.checksum) {
        return Err(format!(
            "invalid package checksum for {} {}",
            package.name, package.version
        ));
    }
    Ok(())
}

/// A deterministic cache-GC plan and its execution result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheGcReport {
    pub candidates: Vec<PathBuf>,
    pub removed: usize,
    pub dry_run: bool,
}

/// Plan and optionally remove unreferenced package artifacts and temporary files.
/// Candidate paths are traversed and returned in lexical order.
pub fn gc_cache(
    cache_root: &Path,
    referenced: &[RegistryPackage],
    dry_run: bool,
) -> Result<CacheGcReport, String> {
    if !cache_root.exists() {
        return Ok(CacheGcReport {
            candidates: Vec::new(),
            removed: 0,
            dry_run,
        });
    }
    let keep = referenced
        .iter()
        .map(|package| package_cache_path(cache_root, package))
        .collect::<std::collections::BTreeSet<_>>();
    let mut files = Vec::new();
    collect_cache_files(cache_root, &mut files)?;
    files.sort();
    let mut candidates = Vec::new();
    for path in files {
        if path.extension().and_then(|value| value.to_str()) == Some("tmp") || !keep.contains(&path)
        {
            candidates.push(path);
        }
    }
    if !dry_run {
        for path in &candidates {
            fs::remove_file(path).map_err(|e| format!("package cache cleanup failed: {e}"))?;
        }
    }
    Ok(CacheGcReport {
        removed: if dry_run { 0 } else { candidates.len() },
        candidates,
        dry_run,
    })
}

/// Remove unreferenced package artifacts and temporary files from a cache.
#[allow(dead_code)]
pub fn prune_cache(cache_root: &Path, referenced: &[RegistryPackage]) -> Result<usize, String> {
    Ok(gc_cache(cache_root, referenced, false)?.removed)
}

fn collect_cache_files(root: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let mut entries = fs::read_dir(root)
        .map_err(|e| format!("package cache read failed: {e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("package cache read failed: {e}"))?;
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            collect_cache_files(&path, files)?;
        } else {
            files.push(path);
        }
    }
    Ok(())
}

fn hmac_sha256_hex(secret: &[u8], message: &[u8]) -> String {
    let mut key = secret.to_vec();
    if key.len() > 64 {
        key = Sha256::digest(&key).to_vec();
    }
    key.resize(64, 0);
    let mut inner = vec![0x36; 64];
    let mut outer = vec![0x5c; 64];
    for (index, byte) in key.iter().enumerate() {
        inner[index] ^= byte;
        outer[index] ^= byte;
    }
    let mut inner_input = inner;
    inner_input.extend_from_slice(message);
    let inner_digest = Sha256::digest(&inner_input);
    let mut outer_input = outer;
    outer_input.extend_from_slice(&inner_digest);
    sha256_hex(&outer_input)
}

fn constant_time_equal(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    left.iter()
        .zip(right)
        .fold(0u8, |difference, (a, b)| difference | (a ^ b))
        == 0
}

#[cfg(test)]
mod tests {
    use super::{
        cache_package, cache_package_source, fetch_source_with_agent, find_package,
        hmac_sha256_hex, normalize_registry_origin, persist_registry_package, prune_cache,
        publish_package, publish_package_with_agent, read_index, read_index_source,
        read_signed_index, redact_registry_secret, registry_auth_failure_message,
        resolve_dependency_graph, resolve_registry_token, sha256_hex, validate_package_name,
        verify_cached_package, verify_signed_index_bytes, RegistryCredentialStore, RegistryPackage,
        RegistryScheme, RegistryService, TrustedRegistryPolicy,
    };
    use std::collections::BTreeMap;
    use std::fs;
    use std::io::{Read, Write};
    use std::net::{SocketAddr, TcpListener, TcpStream};
    use std::path::Path;
    use std::sync::{Arc, Mutex, OnceLock};
    use std::thread;

    fn authenticated_tls_fixture(
        method: &'static str,
        expected_token: &'static str,
        response_body: &'static str,
        response_status: &'static str,
    ) -> (String, ureq::Agent, thread::JoinHandle<()>) {
        let certificate =
            rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
        let certificate_der = certificate.serialize_der().unwrap();
        let private_key = certificate.serialize_private_key_der();
        let server_certificate = rustls::pki_types::CertificateDer::from(certificate_der.clone());
        let server_key = rustls::pki_types::PrivateKeyDer::Pkcs8(
            rustls::pki_types::PrivatePkcs8KeyDer::from(private_key),
        );
        let server_config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(vec![server_certificate.clone()], server_key)
            .unwrap();
        let mut roots = rustls::RootCertStore::empty();
        roots.add(server_certificate).unwrap();
        let client_config = rustls::ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth();
        let agent = ureq::builder().tls_config(Arc::new(client_config)).build();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let handle = thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let connection = rustls::ServerConnection::new(Arc::new(server_config)).unwrap();
            let mut tls = rustls::StreamOwned::new(connection, stream);
            let mut request = Vec::new();
            let mut buffer = [0u8; 1024];
            while !request.windows(4).any(|window| window == b"\r\n\r\n") {
                let count = tls.read(&mut buffer).unwrap();
                if count == 0 {
                    break;
                }
                request.extend_from_slice(&buffer[..count]);
            }
            let header_end = request
                .windows(4)
                .position(|window| window == b"\r\n\r\n")
                .map(|position| position + 4)
                .unwrap_or(request.len());
            let request_headers = String::from_utf8_lossy(&request[..header_end]);
            let content_length = request_headers
                .lines()
                .find_map(|line| {
                    line.strip_prefix("Content-Length:")
                        .and_then(|value| value.trim().parse::<usize>().ok())
                })
                .unwrap_or(0);
            let mut body_received = request.len().saturating_sub(header_end);
            while body_received < content_length {
                let count = tls.read(&mut buffer).unwrap();
                if count == 0 {
                    break;
                }
                body_received += count;
            }
            assert!(request_headers.starts_with(method));
            assert!(request_headers
                .lines()
                .any(|line| line == format!("Authorization: Bearer {expected_token}")));
            let response = format!(
                "HTTP/1.1 {response_status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response_body}",
                response_body.len()
            );
            tls.write_all(response.as_bytes()).unwrap();
            tls.flush().unwrap();
        });
        (
            format!("https://localhost:{}/index.json", address.port()),
            agent,
            handle,
        )
    }

    #[test]
    fn authenticated_https_fetch_sends_bearer_token() {
        let (url, agent, handle) =
            authenticated_tls_fixture("GET", "fixture-token", "[]", "200 OK");
        let mut credentials = RegistryCredentialStore::new();
        credentials.insert(&url, "fixture-token").unwrap();
        let bytes = fetch_source_with_agent(&agent, &url, &credentials).unwrap();
        assert_eq!(bytes, b"[]");
        handle.join().unwrap();
    }

    #[test]
    fn authenticated_https_publish_sends_bearer_token() {
        let (url, agent, handle) =
            authenticated_tls_fixture("POST", "publish-token", "", "201 Created");
        let root =
            std::env::temp_dir().join(format!("zap-registry-tls-publish-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let archive = root.join("source.pkg");
        fs::write(&archive, b"package").unwrap();
        let package = RegistryPackage {
            name: "demo".into(),
            version: "1.0.0".into(),
            source: "demo.pkg".into(),
            checksum: sha256_hex(b"package"),
            yanked: false,
            dependencies: BTreeMap::new(),
        };
        publish_package_with_agent(&agent, &url, &archive, &package, Some("publish-token"))
            .unwrap();
        handle.join().unwrap();
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn credential_store_prefers_the_longest_matching_origin_path() {
        let mut credentials = RegistryCredentialStore::new();
        assert!(credentials
            .insert("https://example.test", "root-token")
            .unwrap());
        assert!(credentials
            .insert("https://example.test/team", "team-token")
            .unwrap());
        assert_eq!(
            credentials
                .resolve("https://example.test/team/package.pkg")
                .unwrap(),
            Some("team-token")
        );
        assert_eq!(
            credentials
                .resolve("https://example.test/other/package.pkg")
                .unwrap(),
            Some("root-token")
        );
        assert_eq!(
            credentials
                .resolve("https://other.test/package.pkg")
                .unwrap(),
            None
        );
    }

    #[test]
    fn credential_store_rejects_insecure_and_invalid_tokens() {
        let mut credentials = RegistryCredentialStore::new();
        assert_eq!(
            credentials
                .insert("http://example.test", "token")
                .unwrap_err(),
            "registry credentials require HTTPS or a local file origin"
        );
        for token in ["", "contains whitespace", "contains\nnewline"] {
            assert_eq!(
                credentials
                    .insert("https://example.test", token)
                    .unwrap_err(),
                "registry authentication token is invalid"
            );
        }
    }

    #[test]
    fn explicit_token_and_redaction_are_deterministic() {
        let credentials = RegistryCredentialStore::new();
        assert_eq!(
            resolve_registry_token("https://example.test", Some("explicit-token"), &credentials)
                .unwrap(),
            Some("explicit-token".to_string())
        );
        assert_eq!(
            redact_registry_secret(
                "registry request failed with token explicit-token",
                Some("explicit-token")
            ),
            "registry request failed with token <redacted>"
        );
    }

    #[test]
    fn authentication_errors_have_stable_codes_and_no_secret_content() {
        assert_eq!(
            registry_auth_failure_message(401, "https://Example.test/api", None).unwrap(),
            "registry authentication error [ZAP-REG-AUTH-001]: credentials required for https://example.test/api"
        );
        assert_eq!(
            registry_auth_failure_message(401, "https://example.test/api", Some("secret-token"))
                .unwrap(),
            "registry authentication error [ZAP-REG-AUTH-002]: credentials rejected for https://example.test/api"
        );
        assert_eq!(
            registry_auth_failure_message(403, "https://example.test/api", Some("secret-token"))
                .unwrap(),
            "registry authentication error [ZAP-REG-AUTH-003]: permission denied for https://example.test/api"
        );
        assert!(!registry_auth_failure_message(
            401,
            "https://example.test/api",
            Some("secret-token")
        )
        .unwrap()
        .contains("secret-token"));
        assert!(registry_auth_failure_message(503, "https://example.test/api", None).is_none());
    }

    #[test]
    fn security_property_origin_normalization_is_idempotent_for_adversarial_corpus() {
        let valid_sources = [
            "https://EXAMPLE.test",
            "https://example.test:443/api///",
            "http://registry.test:8080/team/./packages",
            "file:///var/lib/zap/registry///",
            "file://relative/path",
            "https://[::1]:443/api",
        ];
        for source in valid_sources {
            let first = normalize_registry_origin(source).unwrap();
            let canonical = first.as_url();
            let second = normalize_registry_origin(&canonical).unwrap();
            assert_eq!(
                first, second,
                "normalization changed {source} -> {canonical}"
            );
        }

        let invalid_sources = [
            "",
            "example.test/index",
            "ftp://example.test/index",
            "https://user:secret@example.test/index",
            "https://example.test/index?token=secret",
            "https://example.test/index#fragment",
            "https://example.test/a/../b",
            "https://example.test:65536/index",
            "https://example.test\\index",
            "https://example.test/with whitespace",
        ];
        for source in invalid_sources {
            assert!(
                normalize_registry_origin(source).is_err(),
                "accepted {source}"
            );
        }
    }

    #[test]
    fn security_property_trust_and_credential_scopes_do_not_cross_boundaries() {
        let mut policy = TrustedRegistryPolicy::new();
        assert!(policy.add("https://example.test/team").unwrap());
        assert!(policy.is_trusted("https://example.test/team/pkg").unwrap());
        assert!(!policy
            .is_trusted("https://example.test/teammate/pkg")
            .unwrap());
        assert!(!policy.is_trusted("http://example.test/team/pkg").unwrap());
        assert!(!policy.is_trusted("https://other.test/team/pkg").unwrap());

        let mut credentials = RegistryCredentialStore::new();
        credentials
            .insert("https://example.test", "root-token")
            .unwrap();
        credentials
            .insert("https://example.test/team", "team-token")
            .unwrap();
        assert_eq!(
            credentials
                .resolve("https://example.test/team/pkg")
                .unwrap(),
            Some("team-token")
        );
        assert_eq!(
            credentials
                .resolve("https://example.test/teammate/pkg")
                .unwrap(),
            Some("root-token")
        );
        assert_eq!(
            credentials.resolve("http://example.test/team/pkg").unwrap(),
            None
        );
        assert!(credentials
            .insert("https://example.test", "token with spaces")
            .is_err());
    }

    #[test]
    fn security_property_trusted_registry_allowlist_is_bounded_and_deterministic() {
        let mut policy = TrustedRegistryPolicy::new();
        for index in 0..64 {
            assert!(policy
                .add(&format!("https://registry-{index}.example.test"))
                .unwrap());
        }
        assert_eq!(policy.origins().count(), 64);
        assert!(policy.add("https://overflow.example.test").is_err());
        assert!(!policy.add("https://registry-63.example.test/").unwrap());
        let origins = policy
            .origins()
            .map(|origin| origin.as_url())
            .collect::<Vec<_>>();
        let mut sorted = origins.clone();
        sorted.sort();
        assert_eq!(origins, sorted);
    }

    #[test]
    fn security_property_signed_index_mutations_never_panic_or_accept_tampering() {
        let package = serde_json::json!([{
            "name": "demo",
            "version": "1.0.0",
            "source": "demo.pkg",
            "checksum": sha256_hex(b"package")
        }]);
        let canonical = serde_json::to_vec(&package).unwrap();
        let signature = hmac_sha256_hex(b"test-secret", &canonical);
        let valid = serde_json::to_vec(&serde_json::json!({
            "signature": signature,
            "packages": package
        }))
        .unwrap();
        assert!(verify_signed_index_bytes(&valid, b"test-secret").is_ok());
        assert!(verify_signed_index_bytes(&valid, b"wrong-secret").is_err());
        let mut corpus = vec![Vec::new(), b"null".to_vec(), b"[]".to_vec(), valid.clone()];
        corpus.extend((0..valid.len()).map(|index| {
            let mut mutated = valid.clone();
            mutated[index] ^= 0x01;
            mutated
        }));
        for bytes in corpus {
            let result =
                std::panic::catch_unwind(|| verify_signed_index_bytes(&bytes, b"test-secret"));
            assert!(
                result.is_ok(),
                "signed-index parser panicked for corpus input"
            );
            if bytes != valid {
                assert!(result.unwrap().is_err(), "accepted mutated signed index");
            }
        }
    }

    #[test]
    fn rejects_non_boolean_yanked_metadata_without_fallback() {
        let package = serde_json::json!([{
            "name": "demo",
            "version": "1.0.0",
            "source": "demo.pkg",
            "checksum": sha256_hex(b"package"),
            "yanked": "true"
        }]);
        let canonical = serde_json::to_vec(&package).unwrap();
        let signature = hmac_sha256_hex(b"test-secret", &canonical);
        let index = serde_json::to_vec(&serde_json::json!({
            "signature": signature,
            "packages": package
        }))
        .unwrap();
        let error = verify_signed_index_bytes(&index, b"test-secret").unwrap_err();
        assert_eq!(
            error,
            "registry yanked field must be a boolean for demo 1.0.0"
        );
    }
    #[test]
    fn security_property_secret_redaction_removes_all_token_occurrences() {
        let token = "s3cr3t-token-123";
        let message = format!("token={token}; retry token={token}; bearer {token}");
        let redacted = redact_registry_secret(&message, Some(token));
        assert!(!redacted.contains(token));
        assert_eq!(redacted.matches("<redacted>").count(), 3);
    }

    #[test]
    fn canonical_origin_normalizes_scheme_host_port_and_path() {
        let origin = normalize_registry_origin(" HTTPS://Registry.Example:443/api/// ").unwrap();
        assert_eq!(origin.scheme, RegistryScheme::Https);
        assert_eq!(origin.host.as_deref(), Some("registry.example"));
        assert_eq!(origin.port, None);
        assert_eq!(origin.path_prefix, "/api");
        assert!(origin.is_secure());
    }

    #[test]
    fn package_name_validation_rejects_path_traversal_components() {
        for name in [
            "..",
            "../escape",
            "nested/name",
            "nested\\\\name",
            "bad\u{7f}name",
        ] {
            assert!(
                validate_package_name(name).is_err(),
                "accepted unsafe package name {name:?}"
            );
        }
        for name in ["demo", "zap_core", "zap-runtime-2"] {
            assert!(
                validate_package_name(name).is_ok(),
                "rejected safe package name {name:?}"
            );
        }
    }

    #[test]
    fn canonical_origin_rejects_credentials_queries_fragments_and_traversal() {
        for source in [
            "https://user:secret@example.test/index",
            "https://example.test/index?token=secret",
            "https://example.test/index#fragment",
            "https://example.test/a/../b",
            "https://example.test:bad/index",
            "example.test/index",
        ] {
            assert!(
                normalize_registry_origin(source).is_err(),
                "accepted {source}"
            );
        }
    }

    #[test]
    fn trusted_policy_is_deterministic_idempotent_and_path_scoped() {
        let mut policy = TrustedRegistryPolicy::new();
        assert!(policy.add("https://EXAMPLE.test/api/").unwrap());
        assert!(!policy.add("https://example.test:443/api").unwrap());
        assert!(policy
            .is_trusted("https://example.test/api/packages")
            .unwrap());
        assert!(!policy
            .is_trusted("https://example.test/apix/packages")
            .unwrap());
        assert!(!policy
            .is_trusted("https://other.test/api/packages")
            .unwrap());
        assert_eq!(
            policy
                .origins()
                .map(|origin| origin.path_prefix.as_str())
                .collect::<Vec<_>>(),
            vec!["/api"]
        );
        assert!(policy.remove("https://example.test/api").unwrap());
        assert!(!policy
            .is_trusted("https://example.test/api/packages")
            .unwrap());
    }

    #[test]
    fn trusted_policy_has_a_bounded_origin_count() {
        let mut policy = TrustedRegistryPolicy::new();
        for index in 0..64 {
            assert!(policy
                .add(&format!("https://registry-{index}.example.test"))
                .unwrap());
        }
        let error = policy
            .add("https://registry-overflow.example.test")
            .unwrap_err();
        assert_eq!(error, "trusted registry policy exceeds 64 origins");
    }

    #[test]
    fn index_is_sorted_and_exact_lookup_is_deterministic() {
        let root = std::env::temp_dir().join(format!("zap-registry-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let index = root.join("index.json");
        let checksum = sha256_hex(b"package");
        fs::write(&index, format!(r#"{{"packages":[{{"name":"z","version":"1","source":"z.pkg","checksum":"{checksum}"}},{{"name":"a","version":"1","source":"a.pkg","checksum":"{checksum}"}}]}}"#)).unwrap();
        let packages = read_index(&index).unwrap();
        assert_eq!(packages[0].name, "a");
        assert_eq!(find_package(&packages, "z", "1").unwrap().source, "z.pkg");
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn file_backed_index_and_source_use_the_same_validation() {
        let root = std::env::temp_dir().join(format!("zap-registry-source-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let package_path = root.join("package.pkg");
        let index_path = root.join("index.json");
        fs::write(&package_path, b"package").unwrap();
        let checksum = sha256_hex(b"package");
        let source = format!("file://{}", package_path.display());
        let index = serde_json::json!({
            "packages": [{
                "name": "demo",
                "version": "1.0.0",
                "source": source,
                "checksum": checksum,
            }]
        });
        fs::write(&index_path, serde_json::to_vec(&index).unwrap()).unwrap();
        let packages = read_index_source(&format!("file://{}", index_path.display())).unwrap();
        let cached =
            cache_package_source(&packages[0].source, &root.join("cache"), &packages[0]).unwrap();
        verify_cached_package(&cached, &packages[0]).unwrap();
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn publish_rejects_bad_checksum_before_network_access() {
        let root = std::env::temp_dir().join(format!("zap-publish-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let archive = root.join("package.pkg");
        fs::write(&archive, b"package").unwrap();
        let package = RegistryPackage {
            name: "demo".into(),
            version: "1.0.0".into(),
            source: archive.display().to_string(),
            checksum: "0".repeat(64),
            yanked: false,
            dependencies: std::collections::BTreeMap::new(),
        };
        let error = publish_package("https://registry.invalid/publish", &archive, &package, None)
            .unwrap_err();
        assert!(error.contains("publish checksum mismatch"));
        fs::remove_dir_all(&root).unwrap();
    }

    fn with_insecure_http<T>(operation: impl FnOnce() -> T) -> T {
        static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        let _guard = ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        std::env::set_var("ZAP_ALLOW_INSECURE_HTTP", "1");
        let result = operation();
        std::env::remove_var("ZAP_ALLOW_INSECURE_HTTP");
        result
    }

    fn with_secure_http<T>(operation: impl FnOnce() -> T) -> T {
        static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        let _guard = ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        std::env::remove_var("ZAP_ALLOW_INSECURE_HTTP");
        operation()
    }

    fn local_http_response(status: &str, body: &[u8]) -> (String, std::thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let body = body.to_vec();
        let status = status.to_string();
        let handle = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = Vec::with_capacity(4096);
            let mut buffer = [0_u8; 1024];
            let header_end = loop {
                if let Some(position) = request.windows(4).position(|window| window == b"\r\n\r\n")
                {
                    break position + 4;
                }
                let read = stream.read(&mut buffer).unwrap();
                if read == 0 || request.len() >= 64 * 1024 {
                    break request.len();
                }
                request.extend_from_slice(&buffer[..read]);
            };
            let content_length = String::from_utf8_lossy(&request[..header_end.min(request.len())])
                .lines()
                .find_map(|line| line.strip_prefix("Content-Length:"))
                .and_then(|value| value.trim().parse::<usize>().ok())
                .unwrap_or(0);
            let request_end = header_end.saturating_add(content_length);
            while request.len() < request_end && request.len() < 64 * 1024 {
                let read = stream.read(&mut buffer).unwrap();
                if read == 0 {
                    break;
                }
                request.extend_from_slice(&buffer[..read]);
            }
            let headers = format!(
                "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            stream.write_all(headers.as_bytes()).unwrap();
            stream.write_all(&body).unwrap();
        });
        (format!("http://{address}"), handle)
    }

    #[test]
    fn insecure_http_registry_transport_is_rejected_by_default() {
        with_secure_http(|| {
            let error = read_index_source("http://127.0.0.1:1/index.json").unwrap_err();
            assert!(error.contains("insecure HTTP registry transport is disabled"));
        });
    }

    #[test]
    fn malformed_remote_index_has_stable_diagnostic() {
        let (url, handle) = local_http_response("200 OK", b"not-json");
        let error =
            with_insecure_http(|| read_index_source(&format!("{url}/index.json"))).unwrap_err();
        handle.join().unwrap();
        assert!(error.starts_with("registry index JSON is invalid:"));
    }

    #[test]
    fn registry_index_reports_missing_credentials_status() {
        let (url, handle) = local_http_response("401 Unauthorized", b"authentication required");
        let error =
            with_insecure_http(|| read_index_source(&format!("{url}/index.json"))).unwrap_err();
        handle.join().unwrap();
        assert_eq!(
            error,
            format!(
                "registry authentication error [ZAP-REG-AUTH-001]: credentials required for {url}/index.json"
            )
        );
    }

    #[test]
    fn registry_index_reports_permission_status() {
        let (url, handle) = local_http_response("403 Forbidden", b"forbidden");
        let error =
            with_insecure_http(|| read_index_source(&format!("{url}/index.json"))).unwrap_err();
        handle.join().unwrap();
        assert_eq!(
            error,
            format!(
                "registry authentication error [ZAP-REG-AUTH-003]: permission denied for {url}/index.json"
            )
        );
    }

    #[test]
    fn registry_index_reports_service_http_status() {
        let (url, handle) =
            local_http_response("503 Service Unavailable", b"temporarily unavailable");
        let error =
            with_insecure_http(|| read_index_source(&format!("{url}/index.json"))).unwrap_err();
        handle.join().unwrap();
        assert_eq!(error, "registry HTTP fetch failed with HTTP 503");
    }

    #[test]
    fn registry_publish_reports_service_http_status() {
        let root = std::env::temp_dir().join(format!("zap-publish-http-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let archive = root.join("package.pkg");
        fs::write(&archive, b"package").unwrap();
        let package = RegistryPackage {
            name: "demo".into(),
            version: "1.0.0".into(),
            source: archive.display().to_string(),
            checksum: sha256_hex(b"package"),
            yanked: false,
            dependencies: BTreeMap::new(),
        };
        let (url, handle) =
            local_http_response("503 Service Unavailable", b"temporarily unavailable");
        let error = with_insecure_http(|| {
            publish_package(&format!("{url}/publish"), &archive, &package, None)
        })
        .unwrap_err();
        handle.join().unwrap();
        assert_eq!(error, "registry publish failed with HTTP 503");
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn cache_write_and_verification_use_checksum_path() {
        let root = std::env::temp_dir().join(format!("zap-cache-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let source = root.join("source.pkg");
        fs::write(&source, b"package").unwrap();
        let checksum = sha256_hex(b"package");
        let package = super::RegistryPackage {
            name: "demo".into(),
            version: "1.0.0".into(),
            source: "file://source.pkg".into(),
            checksum: checksum.clone(),
            yanked: false,
            dependencies: std::collections::BTreeMap::new(),
        };
        let cached = cache_package(Path::new(&source), &root.join("cache"), &package).unwrap();
        verify_cached_package(&cached, &package).unwrap();
        let locked_yanked = RegistryPackage {
            yanked: true,
            ..package.clone()
        };
        verify_cached_package(&cached, &locked_yanked).unwrap();
        assert!(cached.ends_with(format!("demo/1.0.0/{checksum}.pkg")));
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn recursive_resolution_is_sorted_and_selects_transitive_packages() {
        let checksum = sha256_hex(b"package");
        let mut a_dependencies = BTreeMap::new();
        a_dependencies.insert("b".to_string(), "^1.0.0".to_string());
        let mut b_dependencies = BTreeMap::new();
        b_dependencies.insert("c".to_string(), "^1.0.0".to_string());
        let index = vec![
            RegistryPackage {
                name: "c".into(),
                version: "1.2.0".into(),
                source: "c.pkg".into(),
                checksum: checksum.clone(),
                yanked: false,
                dependencies: BTreeMap::new(),
            },
            RegistryPackage {
                name: "a".into(),
                version: "1.0.0".into(),
                source: "a.pkg".into(),
                checksum: checksum.clone(),
                yanked: false,
                dependencies: a_dependencies,
            },
            RegistryPackage {
                name: "b".into(),
                version: "1.0.0".into(),
                source: "b.pkg".into(),
                checksum,
                yanked: false,
                dependencies: b_dependencies,
            },
        ];
        let mut roots = BTreeMap::new();
        roots.insert("a".to_string(), "^1.0.0".to_string());
        let resolved = resolve_dependency_graph(&index, &roots).unwrap();
        assert_eq!(
            resolved
                .iter()
                .map(|package| package.name.as_str())
                .collect::<Vec<_>>(),
            vec!["a", "b", "c"]
        );
        assert_eq!(resolved[2].version, "1.2.0");
    }

    #[test]
    fn recursive_resolution_reports_cycles_and_conflicts() {
        let checksum = sha256_hex(b"package");
        let mut a_dependencies = BTreeMap::new();
        a_dependencies.insert("b".to_string(), "^1.0.0".to_string());
        let mut b_dependencies = BTreeMap::new();
        b_dependencies.insert("a".to_string(), "^1.0.0".to_string());
        let cycle_index = vec![
            RegistryPackage {
                name: "a".into(),
                version: "1.0.0".into(),
                source: "a.pkg".into(),
                checksum: checksum.clone(),
                yanked: false,
                dependencies: a_dependencies,
            },
            RegistryPackage {
                name: "b".into(),
                version: "1.0.0".into(),
                source: "b.pkg".into(),
                checksum: checksum.clone(),
                yanked: false,
                dependencies: b_dependencies,
            },
        ];
        let mut roots = BTreeMap::new();
        roots.insert("a".to_string(), "^1.0.0".to_string());
        let cycle_error = resolve_dependency_graph(&cycle_index, &roots).unwrap_err();
        assert_eq!(
            cycle_error,
            "registry dependency cycle detected: a -> b -> a"
        );

        let mut left_dependencies = BTreeMap::new();
        left_dependencies.insert("shared".to_string(), "^1.0.0".to_string());
        let mut right_dependencies = BTreeMap::new();
        right_dependencies.insert("shared".to_string(), "^2.0.0".to_string());
        let conflict_index = vec![
            RegistryPackage {
                name: "left".into(),
                version: "1.0.0".into(),
                source: "left.pkg".into(),
                checksum: checksum.clone(),
                yanked: false,
                dependencies: left_dependencies,
            },
            RegistryPackage {
                name: "right".into(),
                version: "1.0.0".into(),
                source: "right.pkg".into(),
                checksum: checksum.clone(),
                yanked: false,
                dependencies: right_dependencies,
            },
            RegistryPackage {
                name: "shared".into(),
                version: "1.5.0".into(),
                source: "shared-1.pkg".into(),
                checksum: checksum.clone(),
                yanked: false,
                dependencies: BTreeMap::new(),
            },
            RegistryPackage {
                name: "shared".into(),
                version: "2.1.0".into(),
                source: "shared-2.pkg".into(),
                checksum,
                yanked: false,
                dependencies: BTreeMap::new(),
            },
        ];
        let mut conflict_roots = BTreeMap::new();
        conflict_roots.insert("left".to_string(), "1.0.0".to_string());
        conflict_roots.insert("right".to_string(), "1.0.0".to_string());
        let conflict_error =
            resolve_dependency_graph(&conflict_index, &conflict_roots).unwrap_err();
        assert_eq!(
            conflict_error,
            "registry dependency version conflict for shared: selected 1.5.0 does not satisfy ^2.0.0"
        );
    }

    #[test]
    fn yanked_releases_are_not_selected_for_exact_or_range_requests() {
        let checksum = sha256_hex(b"package");
        let index = vec![
            RegistryPackage {
                name: "demo".into(),
                version: "1.0.0".into(),
                source: "stable.pkg".into(),
                checksum: checksum.clone(),
                yanked: false,
                dependencies: BTreeMap::new(),
            },
            RegistryPackage {
                name: "demo".into(),
                version: "2.0.0".into(),
                source: "yanked.pkg".into(),
                checksum,
                yanked: true,
                dependencies: BTreeMap::new(),
            },
        ];
        assert_eq!(
            super::find_package(&index, "demo", "2.0.0").unwrap_err(),
            "registry package is yanked: demo 2.0.0"
        );
        assert_eq!(
            super::find_package_requirement(&index, "demo", ">=1.0.0")
                .unwrap()
                .version,
            "1.0.0"
        );
    }

    #[test]
    fn range_selection_prefers_highest_compatible_version() {
        let checksum = sha256_hex(b"package");
        let index = vec![
            RegistryPackage {
                name: "demo".into(),
                version: "1.2.0".into(),
                source: "a".into(),
                checksum: checksum.clone(),
                yanked: false,
                dependencies: std::collections::BTreeMap::new(),
            },
            RegistryPackage {
                name: "demo".into(),
                version: "1.4.2".into(),
                source: "b".into(),
                checksum: checksum.clone(),
                yanked: false,
                dependencies: std::collections::BTreeMap::new(),
            },
            RegistryPackage {
                name: "demo".into(),
                version: "2.0.0".into(),
                source: "c".into(),
                checksum,
                yanked: false,
                dependencies: std::collections::BTreeMap::new(),
            },
        ];
        assert_eq!(
            super::find_package_requirement(&index, "demo", "^1.2.0")
                .unwrap()
                .version,
            "1.4.2"
        );
    }

    #[test]
    fn range_selection_supports_tilde_and_comparator_intersections() {
        let checksum = sha256_hex(b"package");
        let index = vec![
            RegistryPackage {
                name: "demo".into(),
                version: "1.2.0".into(),
                source: "a".into(),
                checksum: checksum.clone(),
                yanked: false,
                dependencies: std::collections::BTreeMap::new(),
            },
            RegistryPackage {
                name: "demo".into(),
                version: "1.2.9".into(),
                source: "b".into(),
                checksum: checksum.clone(),
                yanked: false,
                dependencies: std::collections::BTreeMap::new(),
            },
            RegistryPackage {
                name: "demo".into(),
                version: "1.3.0".into(),
                source: "c".into(),
                checksum,
                yanked: false,
                dependencies: std::collections::BTreeMap::new(),
            },
        ];
        assert_eq!(
            super::find_package_requirement(&index, "demo", "~1.2.0")
                .unwrap()
                .version,
            "1.2.9"
        );
        assert_eq!(
            super::find_package_requirement(&index, "demo", ">=1.2.0,<1.3.0")
                .unwrap()
                .version,
            "1.2.9"
        );
    }

    #[test]
    fn signed_index_verification_accepts_only_the_expected_secret() {
        let checksum = sha256_hex(b"package");
        let packages = serde_json::json!([{
            "name": "demo",
            "version": "1.0.0",
            "source": "demo.pkg",
            "checksum": checksum
        }]);
        let canonical = serde_json::to_vec(&packages).unwrap();
        let signature = hmac_sha256_hex(b"secret", &canonical);
        let bytes = serde_json::to_vec(&serde_json::json!({
            "signature": signature,
            "packages": packages
        }))
        .unwrap();
        assert_eq!(
            verify_signed_index_bytes(&bytes, b"secret").unwrap().len(),
            1
        );
        assert_eq!(
            verify_signed_index_bytes(&bytes, b"wrong").unwrap_err(),
            "registry index signature mismatch"
        );
    }

    #[test]
    fn cache_pruning_keeps_referenced_packages_and_removes_temporary_files() {
        let root = std::env::temp_dir().join(format!("zap-cache-prune-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let checksum = sha256_hex(b"package");
        let keep = RegistryPackage {
            name: "demo".into(),
            version: "1.0.0".into(),
            source: "keep".into(),
            checksum: checksum.clone(),
            yanked: false,
            dependencies: std::collections::BTreeMap::new(),
        };
        let stale = RegistryPackage {
            name: "demo".into(),
            version: "2.0.0".into(),
            source: "stale".into(),
            checksum,
            yanked: false,
            dependencies: std::collections::BTreeMap::new(),
        };
        let cache = root.join("cache");
        fs::create_dir_all(cache.join("demo/1.0.0")).unwrap();
        fs::create_dir_all(cache.join("demo/2.0.0")).unwrap();
        fs::write(super::package_cache_path(&cache, &keep), b"package").unwrap();
        fs::write(super::package_cache_path(&cache, &stale), b"package").unwrap();
        fs::write(cache.join("demo/1.0.0/temp.pkg.tmp"), b"partial").unwrap();
        assert_eq!(prune_cache(&cache, std::slice::from_ref(&keep)).unwrap(), 2);
        assert!(super::package_cache_path(&cache, &keep).exists());
        assert!(!super::package_cache_path(&cache, &stale).exists());
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn cache_gc_dry_run_is_sorted_and_non_destructive() {
        let root =
            std::env::temp_dir().join(format!("zap-cache-gc-dry-run-{}", std::process::id()));
        let cache = root.join("cache");
        fs::create_dir_all(cache.join("zeta/1.0.0")).unwrap();
        fs::create_dir_all(cache.join("alpha/1.0.0")).unwrap();
        fs::write(cache.join("zeta/1.0.0/stale.pkg"), b"stale").unwrap();
        fs::write(cache.join("alpha/1.0.0/partial.tmp"), b"partial").unwrap();
        let report = super::gc_cache(&cache, &[], true).unwrap();
        assert_eq!(report.removed, 0);
        assert!(report.dry_run);
        assert_eq!(report.candidates, {
            let mut expected = vec![
                cache.join("alpha/1.0.0/partial.tmp"),
                cache.join("zeta/1.0.0/stale.pkg"),
            ];
            expected.sort();
            expected
        });
        assert!(cache.join("zeta/1.0.0/stale.pkg").exists());
        assert!(cache.join("alpha/1.0.0/partial.tmp").exists());
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn authenticated_persistence_commits_artifact_and_signed_index() {
        let root =
            std::env::temp_dir().join(format!("zap-registry-persist-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let archive = root.join("source.pkg");
        fs::write(&archive, b"package").unwrap();
        let package = RegistryPackage {
            name: "demo".into(),
            version: "1.0.0".into(),
            source: "demo.pkg".into(),
            checksum: sha256_hex(b"package"),
            yanked: false,
            dependencies: std::collections::BTreeMap::new(),
        };
        let artifact = persist_registry_package(
            &root.join("registry"),
            &archive,
            &package,
            Some("token"),
            b"secret",
        )
        .unwrap();
        assert!(artifact.exists());
        let packages = read_signed_index(&root.join("registry/index.json"), b"secret").unwrap();
        assert_eq!(packages, vec![package.clone()]);
        assert_eq!(
            persist_registry_package(&root.join("registry"), &archive, &package, None, b"secret")
                .unwrap_err(),
            "registry persistence requires an authentication token"
        );
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn range_selection_reports_deterministic_no_match_errors() {
        let index = vec![RegistryPackage {
            name: "demo".into(),
            version: "1.0.0".into(),
            source: "a".into(),
            checksum: sha256_hex(b"package"),
            yanked: false,
            dependencies: std::collections::BTreeMap::new(),
        }];
        let error = super::find_package_requirement(&index, "demo", "^2.0.0").unwrap_err();
        assert_eq!(
            error,
            "registry package does not satisfy requirement: demo ^2.0.0"
        );

        let yanked_index = vec![RegistryPackage {
            name: "demo".into(),
            version: "2.0.0".into(),
            source: "yanked.pkg".into(),
            checksum: sha256_hex(b"package"),
            yanked: true,
            dependencies: BTreeMap::new(),
        }];
        assert_eq!(
            super::find_package_requirement(&yanked_index, "demo", "^2.0.0").unwrap_err(),
            "all matching registry packages are yanked: demo ^2.0.0"
        );
    }

    fn raw_registry_request(address: SocketAddr, request: &[u8]) -> Vec<u8> {
        let mut stream = TcpStream::connect(address).unwrap();
        stream.write_all(request).unwrap();
        let mut response = Vec::new();
        stream.read_to_end(&mut response).unwrap();
        response
    }

    #[test]
    fn managed_registry_service_authenticates_and_persists_packages() {
        let root = std::env::temp_dir().join(format!(
            "zap-registry-service-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let service = RegistryService::start(
            "127.0.0.1:0",
            root.clone(),
            "token".to_owned(),
            b"secret".to_vec(),
        )
        .unwrap();
        let address = service.address();
        let bytes = b"package";
        let checksum = sha256_hex(bytes);
        let unauthorized_publish = format!(
            "POST /publish HTTP/1.1\r\nHost: localhost\r\nX-Zap-Package-Name: demo\r\nX-Zap-Package-Version: 1.0.0\r\nX-Zap-Package-Checksum: {checksum}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            bytes.len()
        );
        let mut unauthorized_publish_bytes = unauthorized_publish.into_bytes();
        unauthorized_publish_bytes.extend_from_slice(bytes);
        let unauthorized_publish_response =
            raw_registry_request(address, &unauthorized_publish_bytes);
        assert!(String::from_utf8_lossy(&unauthorized_publish_response).starts_with("HTTP/1.1 401"));

        let invalid_identity_request = format!(
            "POST /publish HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer token\r\nX-Zap-Package-Name: ../escape\r\nX-Zap-Package-Version: 1.0.0\r\nX-Zap-Package-Checksum: {checksum}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            bytes.len()
        );
        let mut invalid_identity_bytes = invalid_identity_request.into_bytes();
        invalid_identity_bytes.extend_from_slice(bytes);
        let invalid_identity_response = raw_registry_request(address, &invalid_identity_bytes);
        assert!(String::from_utf8_lossy(&invalid_identity_response).starts_with("HTTP/1.1 400"));

        let publish_request = format!(
            "POST /publish HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer token\r\nX-Zap-Package-Name: demo\r\nX-Zap-Package-Version: 1.0.0\r\nX-Zap-Package-Checksum: {checksum}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            bytes.len()
        );
        let mut publish_bytes = publish_request.into_bytes();
        publish_bytes.extend_from_slice(bytes);
        let publish_response = raw_registry_request(address, &publish_bytes);
        assert!(String::from_utf8_lossy(&publish_response).starts_with("HTTP/1.1 201"));

        let bad_checksum_request = format!(
            "POST /publish HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer token\r\nX-Zap-Package-Name: mismatch\r\nX-Zap-Package-Version: 1.0.0\r\nX-Zap-Package-Checksum: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            "0".repeat(64),
            bytes.len()
        );
        let mut bad_checksum_bytes = bad_checksum_request.into_bytes();
        bad_checksum_bytes.extend_from_slice(bytes);
        let bad_checksum_response = raw_registry_request(address, &bad_checksum_bytes);
        let bad_checksum_text = String::from_utf8_lossy(&bad_checksum_response);
        assert!(bad_checksum_text.starts_with("HTTP/1.1 422"));
        assert!(bad_checksum_text.contains("publish checksum mismatch"));

        let index_response = raw_registry_request(
            address,
            b"GET /index.json HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer token\r\nConnection: close\r\n\r\n",
        );
        assert!(String::from_utf8_lossy(&index_response).starts_with("HTTP/1.1 200"));
        let header_end = index_response
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .unwrap()
            + 4;
        let index_bytes = &index_response[header_end..];
        assert_eq!(
            verify_signed_index_bytes(index_bytes, b"secret")
                .unwrap()
                .len(),
            1
        );
        let unauthorized_bytes = raw_registry_request(
            address,
            b"GET /index.json HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        );
        assert!(String::from_utf8_lossy(&unauthorized_bytes).starts_with("HTTP/1.1 401"));
        let traversal_bytes = raw_registry_request(
            address,
            b"GET /packages/../index.json HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer token\r\nConnection: close\r\n\r\n",
        );
        assert!(String::from_utf8_lossy(&traversal_bytes).starts_with("HTTP/1.1 400"));
        service.stop().unwrap();
        fs::remove_dir_all(root).unwrap();
    }
}
