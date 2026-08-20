use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
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
    let root: Value = serde_json::from_slice(&bytes)
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

pub fn cache_package(
    source: &Path,
    cache_root: &Path,
    package: &RegistryPackage,
) -> Result<PathBuf, String> {
    let bytes = fs::read(source).map_err(|e| format!("package source read failed: {e}"))?;
    let actual = sha256_hex(&bytes);
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
    fs::write(&temporary, &bytes).map_err(|e| format!("package cache write failed: {e}"))?;
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

#[cfg(test)]
mod tests {
    use super::{cache_package, find_package, read_index, sha256_hex, verify_cached_package};
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
}

pub fn sha256_for_bytes(bytes: &[u8]) -> String {
    sha256_hex(bytes)
}
pub fn package_cache_path(cache_root: &Path, package: &RegistryPackage) -> PathBuf {
    cache_root
        .join(&package.name)
        .join(&package.version)
        .join(format!("{}.pkg", package.checksum))
}
