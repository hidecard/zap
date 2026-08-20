use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryPackage {
    pub name: String,
    pub version: String,
    pub source: String,
    pub checksum: String,
}

pub fn read_index(path: &Path) -> Result<Vec<RegistryPackage>, String> {
    let bytes = fs::read(path).map_err(|e| format!("registry index read failed: {e}"))?;
    parse_index_bytes(&bytes)
}

/// Read a registry index from a local file, `file://` URL, or HTTP(S) URL.
/// Remote access is deterministic at the response-byte level and is restricted
/// to HTTPS unless `ZAP_ALLOW_INSECURE_HTTP=1` is explicitly set for fixtures.
pub fn read_index_source(source: &str) -> Result<Vec<RegistryPackage>, String> {
    parse_index_bytes(&fetch_source(source)?)
}

fn parse_index_bytes(bytes: &[u8]) -> Result<Vec<RegistryPackage>, String> {
    let root: Value = serde_json::from_slice(bytes)
        .map_err(|e| format!("registry index JSON is invalid: {e}"))?;
    let entries = root
        .get("packages")
        .and_then(Value::as_array)
        .ok_or_else(|| "registry index must contain a packages array".to_string())?;
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
    index
        .iter()
        .find(|package| package.name == name && package.version == version)
        .cloned()
        .ok_or_else(|| format!("registry package not found: {name} {version}"))
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
    index
        .iter()
        .filter(|package| package.name == name)
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
            format!("registry package does not satisfy requirement: {name} {requirement}")
        })
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
    cache_bytes(&fetch_source(source)?, cache_root, package)
}

fn cache_bytes(
    bytes: &[u8],
    cache_root: &Path,
    package: &RegistryPackage,
) -> Result<PathBuf, String> {
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
    Ok(RegistryPackage {
        name,
        version,
        source,
        checksum,
    })
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

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
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
    require_secure_transport(registry_url)?;
    let bytes = fs::read(archive).map_err(|e| format!("package archive read failed: {e}"))?;
    let actual = sha256_hex(&bytes);
    if actual != package.checksum {
        return Err(format!(
            "publish checksum mismatch for {} {}: expected {}, got {}",
            package.name, package.version, package.checksum, actual
        ));
    }
    let mut request = ureq::post(registry_url)
        .set("Content-Type", "application/octet-stream")
        .set("X-Zap-Package-Name", &package.name)
        .set("X-Zap-Package-Version", &package.version)
        .set("X-Zap-Package-Checksum", &package.checksum);
    if let Some(token) = token {
        request = request.set("Authorization", &format!("Bearer {token}"));
    }
    let response = request
        .send_bytes(&bytes)
        .map_err(|e| format!("registry publish failed: {e}"))?;
    if !(200..300).contains(&response.status()) {
        return Err(format!(
            "registry publish failed with HTTP {}",
            response.status()
        ));
    }
    Ok(())
}

fn fetch_source(source: &str) -> Result<Vec<u8>, String> {
    if let Some(path) = source.strip_prefix("file://") {
        return fs::read(path).map_err(|e| format!("registry source read failed: {e}"));
    }
    if source.starts_with("http://") || source.starts_with("https://") {
        require_secure_transport(source)?;
        let response = ureq::get(source)
            .call()
            .map_err(|e| format!("registry HTTP fetch failed: {e}"))?;
        let mut bytes = Vec::new();
        response
            .into_reader()
            .read_to_end(&mut bytes)
            .map_err(|e| format!("registry response read failed: {e}"))?;
        return Ok(bytes);
    }
    fs::read(source).map_err(|e| format!("registry source read failed: {e}"))
}

fn require_secure_transport(source: &str) -> Result<(), String> {
    if source.starts_with("http://")
        && std::env::var("ZAP_ALLOW_INSECURE_HTTP").as_deref() != Ok("1")
    {
        return Err("insecure HTTP registry transport is disabled; use HTTPS or set ZAP_ALLOW_INSECURE_HTTP=1 for local fixtures".to_string());
    }
    if !(source.starts_with("http://") || source.starts_with("https://")) {
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

#[cfg(test)]
mod tests {
    use super::{
        cache_package, cache_package_source, find_package, publish_package, read_index,
        read_index_source, sha256_hex, verify_cached_package, RegistryPackage,
    };
    use std::fs;
    use std::path::Path;

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
        fs::write(
            &index_path,
            format!(r#"{{"packages":[{{"name":"demo","version":"1.0.0","source":"file://{}","checksum":"{}"}}]}}"#, package_path.display(), checksum),
        )
        .unwrap();
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
        };
        let error = publish_package("https://registry.invalid/publish", &archive, &package, None)
            .unwrap_err();
        assert!(error.contains("publish checksum mismatch"));
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
        };
        let cached = cache_package(Path::new(&source), &root.join("cache"), &package).unwrap();
        verify_cached_package(&cached, &package).unwrap();
        assert!(cached.ends_with(format!("demo/1.0.0/{checksum}.pkg")));
        fs::remove_dir_all(&root).unwrap();
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
            },
            RegistryPackage {
                name: "demo".into(),
                version: "1.4.2".into(),
                source: "b".into(),
                checksum: checksum.clone(),
            },
            RegistryPackage {
                name: "demo".into(),
                version: "2.0.0".into(),
                source: "c".into(),
                checksum,
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
            },
            RegistryPackage {
                name: "demo".into(),
                version: "1.2.9".into(),
                source: "b".into(),
                checksum: checksum.clone(),
            },
            RegistryPackage {
                name: "demo".into(),
                version: "1.3.0".into(),
                source: "c".into(),
                checksum,
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
    fn range_selection_reports_deterministic_no_match_errors() {
        let index = vec![RegistryPackage {
            name: "demo".into(),
            version: "1.0.0".into(),
            source: "a".into(),
            checksum: sha256_hex(b"package"),
        }];
        let error = super::find_package_requirement(&index, "demo", "^2.0.0").unwrap_err();
        assert_eq!(
            error,
            "registry package does not satisfy requirement: demo ^2.0.0"
        );
    }
}
