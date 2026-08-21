use super::project::{
    install_dependencies, migrate_lockfile, update_dependencies, write_lockfile, TestOptions,
};
use super::*;
use crate::project::{add_dependency, registry_packages_from_lockfile};

pub const EXIT_PROGRAM_FAILURE: i32 = 1;
pub const EXIT_USAGE_ERROR: i32 = 2;

pub const CLI_HELP: &str = r#"Zap native runtime

Usage:
  zap <file.zp>                         Run a Zap source file
  zap run <file.zp>                     Run a source file explicitly
  zap fmt <file.zp>                     Format a source file
  zap lint <file.zp>                    Check formatting and style
  zap check [dir]                       Validate a Zap project
  zap check --json [dir]                Validate with JSON diagnostics
  zap test [dir]                        Run *_test.zp files
  zap lock [dir]                        Generate zap.lock
  zap lock-migrate [dir]                Upgrade a legacy registry lockfile to v2
  zap add <name> <ver> [dir]            Add a manifest dependency and invalidate zap.lock
  zap install [dir]    Validate and install dependencies from zap.lock
  zap install --locked [dir]             Require and validate the existing zap.lock
  zap update [dir]     Regenerate zap.lock from zap.toml
  zap registry gc [--dry-run] [dir]     Remove unreferenced registry cache files
  zap registry serve <root> [bind]     Serve an authenticated local registry
  zap registry trust list              List trusted registry origins
  zap registry trust add <url>         Add a trusted registry origin
  zap registry trust remove <url>      Remove a trusted registry origin
  zap registry credential list         List configured credential origins
  zap registry credential set <url> --token-env <name>
                                      Store a token read from an environment variable
  zap registry credential remove <url> Remove a configured credential
  zap build [dir]                       Validate and prepare a project
  zap build --locked [dir]              Require and validate the existing zap.lock
  zap init <dir>                        Create a new project
  zap lsp                               Run the LSP server over stdio
  zap async-check                       Validate the async runtime
  zap --version                         Show the version
  zap --help                            Show this help"#;

fn parse_test_args(args: &[String]) -> Result<(PathBuf, TestOptions), String> {
    let mut options = TestOptions::default();
    let mut dir = PathBuf::from("tests");
    let mut index = 2;
    while index < args.len() {
        match args[index].as_str() {
            "--filter" => {
                index += 1;
                let value = args
                    .get(index)
                    .ok_or_else(|| "--filter requires a test name or path".to_string())?;
                options.filter = Some(value.clone());
            }
            "--fail-fast" => options.fail_fast = true,
            "--json" => options.json = true,
            value if value.starts_with('-') => {
                return Err(format!("unknown test option: {value}"));
            }
            value => {
                if dir != Path::new("tests") {
                    return Err("test command accepts only one directory".to_string());
                }
                dir = PathBuf::from(value);
            }
        }
        index += 1;
    }
    Ok((dir, options))
}

fn run_test_command(args: &[String]) {
    let (dir, options) = match parse_test_args(args) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Zap test usage error: {error}");
            process::exit(EXIT_USAGE_ERROR);
        }
    };
    if let Err(error) = run_zap_tests(&dir, &options) {
        if !options.json {
            eprintln!("Zap test error: {error}");
        }
        process::exit(EXIT_PROGRAM_FAILURE);
    }
}

/// Dispatches Zap command-line arguments and owns CLI exit behavior.
pub fn run_cli(args: &[String]) {
    if args.len() == 2 && (args[1] == "--version" || args[1] == "-V") {
        println!("zap {} (native)", env!("CARGO_PKG_VERSION"));
        return;
    }
    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        println!("{CLI_HELP}");
        return;
    }
    if args.len() == 3 && args[1] == "init" {
        let dir = Path::new(&args[2]);
        if dir.exists() {
            eprintln!("cannot initialize existing path: {}", dir.display());
            process::exit(EXIT_PROGRAM_FAILURE);
        }
        fs::create_dir_all(dir).unwrap_or_else(|e| {
            eprintln!("cannot create {}: {e}", dir.display());
            process::exit(EXIT_PROGRAM_FAILURE);
        });
        fs::write(
            dir.join("zap.toml"),
            "[package]\nname = \"hello-zap\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
        )
        .unwrap_or_else(|e| {
            eprintln!("cannot write manifest: {e}");
            process::exit(EXIT_PROGRAM_FAILURE);
        });
        fs::write(
            dir.join("main.zp"),
            "fn main():\n    say \"Hello from Zap\"\n\nmain()\n",
        )
        .unwrap_or_else(|e| {
            eprintln!("cannot write entry file: {e}");
            process::exit(EXIT_PROGRAM_FAILURE);
        });
        fs::create_dir_all(dir.join("tests")).unwrap_or_else(|e| {
            eprintln!("cannot create test directory: {e}");
            process::exit(EXIT_PROGRAM_FAILURE);
        });
        fs::write(dir.join("tests").join("smoke_test.zp"),"let total = 2 + 3\nassert(total == 5, \"basic arithmetic failed\")\nsay \"smoke test passed\"\n").unwrap_or_else(|e|{eprintln!("cannot write starter test: {e}");process::exit(EXIT_PROGRAM_FAILURE);});
        println!("Created Zap project: {}", dir.display());
        return;
    }
    if args.len() == 2 && args[1] == "check" {
        let dir = Path::new(".");
        match validate_project(dir) {
            Ok(info) => println!("valid Zap project: {info}"),
            Err(e) => {
                eprintln!("Zap check error: {e}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 3 && args[1] == "check" {
        if args[2] == "--json" {
            print_project_json(Path::new("."));
            return;
        }
        let dir = Path::new(&args[2]);
        match validate_project(dir) {
            Ok(info) => println!("valid Zap project: {info}"),
            Err(e) => {
                eprintln!("Zap check error: {e}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 4 && args[1] == "check" && args[2] == "--json" {
        print_project_json(Path::new(&args[3]));
        return;
    }
    if args.len() >= 2 && args[1] == "test" {
        run_test_command(args);
        return;
    }
    if args.len() == 2 && args[1] == "lock" {
        match write_lockfile(Path::new(".")) {
            Ok(info) => println!("{info}"),
            Err(e) => {
                eprintln!("Zap lock error: {e}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 3 && args[1] == "lock" {
        match write_lockfile(Path::new(&args[2])) {
            Ok(info) => println!("{info}"),
            Err(e) => {
                eprintln!("Zap lock error: {e}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 2 && args[1] == "lock-migrate" {
        match migrate_lockfile(Path::new(".")) {
            Ok(info) => println!("{info}"),
            Err(e) => {
                eprintln!("Zap lock migration error: {e}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 3 && args[1] == "lock-migrate" {
        match migrate_lockfile(Path::new(&args[2])) {
            Ok(info) => println!("{info}"),
            Err(e) => {
                eprintln!("Zap lock migration error: {e}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if (args.len() == 4 || args.len() == 5) && args[1] == "add" {
        let dir = if args.len() == 5 {
            Path::new(&args[4])
        } else {
            Path::new(".")
        };
        match add_dependency(dir, &args[2], &args[3]) {
            Ok(info) => println!("{info}"),
            Err(e) => {
                eprintln!("Zap add error: {e}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 2 && args[1] == "install" {
        match install_dependencies(Path::new(".")) {
            Ok(info) => println!("{info}"),
            Err(e) => {
                eprintln!("Zap install error: {e}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 3 && args[1] == "install" && args[2] == "--locked" {
        match install_dependencies(Path::new(".")) {
            Ok(info) => println!("{info}"),
            Err(e) => {
                eprintln!("Zap install error: {e}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 4 && args[1] == "install" && args[2] == "--locked" {
        match install_dependencies(Path::new(&args[3])) {
            Ok(info) => println!("{info}"),
            Err(e) => {
                eprintln!("Zap install error: {e}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 3 && args[1] == "install" {
        match install_dependencies(Path::new(&args[2])) {
            Ok(info) => println!("{info}"),
            Err(e) => {
                eprintln!("Zap install error: {e}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 2 && args[1] == "update" {
        match update_dependencies(Path::new(".")) {
            Ok(info) => println!("{info}"),
            Err(e) => {
                eprintln!("Zap update error: {e}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 3 && args[1] == "update" {
        match update_dependencies(Path::new(&args[2])) {
            Ok(info) => println!("{info}"),
            Err(e) => {
                eprintln!("Zap update error: {e}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() >= 4 && args.len() <= 5 && args[1] == "registry" && args[2] == "trust" {
        let action = args[3].as_str();
        let mut policy = match crate::registry::load_trusted_registry_policy() {
            Ok(policy) => policy,
            Err(error) => {
                eprintln!("Zap registry trust error: {error}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        };
        match action {
            "list" if args.len() == 4 => {
                for origin in policy.origins() {
                    println!("{}", origin.as_url());
                }
            }
            "add" | "remove" if args.len() == 5 => {
                let changed = if action == "add" {
                    policy.add(&args[4])
                } else {
                    policy.remove(&args[4])
                };
                let changed = match changed {
                    Ok(changed) => changed,
                    Err(error) => {
                        eprintln!("Zap registry trust error: {error}");
                        process::exit(EXIT_USAGE_ERROR);
                    }
                };
                if let Err(error) = crate::registry::save_trusted_registry_policy(&policy) {
                    eprintln!("Zap registry trust error: {error}");
                    process::exit(EXIT_PROGRAM_FAILURE);
                }
                let verb = if action == "add" {
                    "trusted"
                } else {
                    "removed"
                };
                let suffix = if changed { "" } else { " (unchanged)" };
                println!("{verb} registry origin: {}{suffix}", args[4]);
            }
            _ => {
                eprintln!("Zap registry trust usage: list | add <url> | remove <url>");
                process::exit(EXIT_USAGE_ERROR);
            }
        }
        return;
    }
    if args[1..].starts_with(&["registry".to_string(), "credential".to_string()]) {
        let action = args.get(3).map(String::as_str).unwrap_or_default();
        let mut credentials = match crate::registry::load_registry_credentials() {
            Ok(credentials) => credentials,
            Err(error) => {
                eprintln!("Zap registry credential error: {error}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        };
        match action {
            "list" if args.len() == 4 => {
                for origin in credentials.origins() {
                    println!("{}", origin.as_url());
                }
            }
            "set" if args.len() == 7 && args[5] == "--token-env" => {
                let token = match std::env::var(&args[6]) {
                    Ok(token) => token,
                    Err(_) => {
                        eprintln!("Zap registry credential error: environment variable is missing or invalid");
                        process::exit(EXIT_USAGE_ERROR);
                    }
                };
                if let Err(error) = credentials.insert(&args[4], &token) {
                    eprintln!("Zap registry credential error: {error}");
                    process::exit(EXIT_USAGE_ERROR);
                }
                if let Err(error) = crate::registry::save_registry_credentials(&credentials) {
                    eprintln!("Zap registry credential error: {error}");
                    process::exit(EXIT_PROGRAM_FAILURE);
                }
                println!("configured registry credential: {}", args[4]);
            }
            "remove" if args.len() == 5 => {
                let changed = match credentials.remove(&args[4]) {
                    Ok(changed) => changed,
                    Err(error) => {
                        eprintln!("Zap registry credential error: {error}");
                        process::exit(EXIT_USAGE_ERROR);
                    }
                };
                if let Err(error) = crate::registry::save_registry_credentials(&credentials) {
                    eprintln!("Zap registry credential error: {error}");
                    process::exit(EXIT_PROGRAM_FAILURE);
                }
                let suffix = if changed { "" } else { " (unchanged)" };
                println!("removed registry credential: {}{suffix}", args[4]);
            }
            _ => {
                eprintln!("Zap registry credential usage: list | set <url> --token-env <name> | remove <url>");
                process::exit(EXIT_USAGE_ERROR);
            }
        }
        return;
    }
    if args.len() == 4 && args[1] == "registry" && args[2] == "check" {
        match crate::registry::read_index(Path::new(&args[3])) {
            Ok(packages) => println!("valid registry index: {} packages", packages.len()),
            Err(error) => {
                eprintln!("Zap registry error: {error}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 4 && args[1] == "registry" && args[2] == "fetch" {
        let policy = match crate::registry::load_effective_trusted_registry_policy() {
            Ok(policy) => policy,
            Err(error) => {
                eprintln!("Zap registry error: {error}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        };
        if let Err(error) = policy.require_trusted(&args[3]) {
            eprintln!("Zap registry error: {error}");
            process::exit(EXIT_PROGRAM_FAILURE);
        }
        match crate::registry::read_index_source(&args[3]) {
            Ok(packages) => println!("valid remote registry index: {} packages", packages.len()),
            Err(error) => {
                eprintln!("Zap registry error: {error}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() >= 3 && args.len() <= 5 && args[1] == "registry" && args[2] == "gc" {
        let mut dry_run = false;
        let mut dir = PathBuf::from(".");
        for argument in &args[3..] {
            if argument == "--dry-run" {
                if dry_run {
                    eprintln!("Zap registry error: duplicate --dry-run");
                    process::exit(EXIT_USAGE_ERROR);
                }
                dry_run = true;
            } else if dir != Path::new(".") {
                eprintln!("Zap registry error: gc accepts one project directory");
                process::exit(EXIT_USAGE_ERROR);
            } else {
                dir = PathBuf::from(argument);
            }
        }
        let cache = std::env::var_os("ZAP_CACHE_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| dir.join(".zap/cache"));
        let referenced = match registry_packages_from_lockfile(&dir) {
            Ok(packages) => packages,
            Err(error) => {
                eprintln!("Zap registry error: {error}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        };
        match crate::registry::gc_cache(&cache, &referenced, dry_run) {
            Ok(report) => {
                let action = if report.dry_run {
                    "would remove"
                } else {
                    "removed"
                };
                println!(
                    "registry cache gc: {} candidate(s)",
                    report.candidates.len()
                );
                for path in report.candidates {
                    let portable_path = path.to_string_lossy().replace('\\', "/");
                    println!("{action}: {portable_path}");
                }
            }
            Err(error) => {
                eprintln!("Zap registry error: {error}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if (args.len() == 7 || args.len() == 8) && args[1] == "registry" && args[2] == "cache" {
        let index = match crate::registry::read_index(Path::new(&args[3])) {
            Ok(index) => index,
            Err(error) => {
                eprintln!("Zap registry error: {error}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        };
        let package = match crate::registry::find_package(&index, &args[5], &args[6]) {
            Ok(package) => package,
            Err(error) => {
                eprintln!("Zap registry error: {error}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        };
        let cache = args
            .get(7)
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(".zap/cache"));
        let policy = match crate::registry::load_effective_trusted_registry_policy() {
            Ok(policy) => policy,
            Err(error) => {
                eprintln!("Zap registry error: {error}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        };
        if let Err(error) = policy.require_trusted(&args[4]) {
            eprintln!("Zap registry error: {error}");
            process::exit(EXIT_PROGRAM_FAILURE);
        }
        match crate::registry::cache_package_source(&args[4], &cache, &package) {
            Ok(path) => println!("cached package: {}", path.display()),
            Err(error) => {
                eprintln!("Zap registry error: {error}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if (args.len() == 4 || args.len() == 5) && args[1] == "registry" && args[2] == "serve" {
        let root = PathBuf::from(&args[3]);
        let bind = args.get(4).map(String::as_str).unwrap_or("127.0.0.1:8080");
        let token = match std::env::var("ZAP_REGISTRY_TOKEN") {
            Ok(value) if !value.trim().is_empty() => value,
            _ => {
                eprintln!("Zap registry service error: ZAP_REGISTRY_TOKEN is required");
                process::exit(EXIT_USAGE_ERROR);
            }
        };
        let signing_secret = match std::env::var("ZAP_REGISTRY_SIGNING_SECRET") {
            Ok(value) if !value.is_empty() => value.into_bytes(),
            _ => {
                eprintln!("Zap registry service error: ZAP_REGISTRY_SIGNING_SECRET is required");
                process::exit(EXIT_USAGE_ERROR);
            }
        };
        let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        println!("serving registry at http://{bind}");
        if let Err(error) = crate::registry::serve_registry(bind, root, token, signing_secret, stop)
        {
            eprintln!("Zap registry service error: {error}");
            process::exit(EXIT_PROGRAM_FAILURE);
        }
        return;
    }
    if args.len() == 8 && args[1] == "registry" && args[2] == "publish" {
        let package = crate::registry::RegistryPackage {
            name: args[5].clone(),
            version: args[6].clone(),
            source: args[4].clone(),
            checksum: args[7].to_ascii_lowercase(),
            yanked: false,
            dependencies: std::collections::BTreeMap::new(),
        };
        let policy = match crate::registry::load_effective_trusted_registry_policy() {
            Ok(policy) => policy,
            Err(error) => {
                eprintln!("Zap registry publish error: {error}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        };
        if let Err(error) = policy.require_trusted(&args[3]) {
            eprintln!("Zap registry publish error: {error}");
            process::exit(EXIT_PROGRAM_FAILURE);
        }
        let token = std::env::var("ZAP_REGISTRY_TOKEN").ok();
        match crate::registry::publish_package(
            &args[3],
            Path::new(&args[4]),
            &package,
            token.as_deref(),
        ) {
            Ok(()) => println!("published package: {} {}", package.name, package.version),
            Err(error) => {
                eprintln!("Zap registry publish error: {error}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 2 && args[1] == "lsp" {
        if let Err(error) = crate::lsp::run_stdio() {
            eprintln!("Zap LSP error: {error}");
            process::exit(EXIT_PROGRAM_FAILURE);
        }
        return;
    }
    if args.len() == 2 && args[1] == "async-check" {
        let mut runtime = crate::async_runtime::AsyncRuntime::new();
        runtime.spawn(async {});
        runtime.run_until_idle();
        println!("async runtime foundation ready");
        return;
    }
    if args.len() == 2 && args[1] == "build" {
        match validate_project(Path::new(".")) {
            Ok(info) => println!("built Zap project: {info}"),
            Err(e) => {
                eprintln!("Zap build error: {e}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 3 && args[1] == "build" && args[2] == "--locked" {
        match validate_project(Path::new(".")) {
            Ok(info) => println!("built Zap project: {info}"),
            Err(e) => {
                eprintln!("Zap build error: {e}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 4 && args[1] == "build" && args[2] == "--locked" {
        let dir = Path::new(&args[3]);
        match validate_project(dir) {
            Ok(info) => println!("built Zap project: {info}"),
            Err(e) => {
                eprintln!("Zap build error: {e}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 3 && args[1] == "build" {
        let dir = Path::new(&args[2]);
        match validate_project(dir) {
            Ok(info) => println!("built Zap project: {info}"),
            Err(e) => {
                eprintln!("Zap build error: {e}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 3 && args[1] == "run" {
        let source = read_limited_text(Path::new(&args[2]), "source read").unwrap_or_else(|e| {
            eprintln!("{e}");
            process::exit(EXIT_PROGRAM_FAILURE);
        });
        let base = Path::new(&args[2])
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        if let Err(e) = run_checked(&source, base) {
            eprintln!("Zap error: {e}");
            process::exit(EXIT_PROGRAM_FAILURE);
        }
        return;
    }
    if args.len() == 3 && args[1] == "lint" {
        let path = Path::new(&args[2]);
        let source = read_limited_text(path, "source read").unwrap_or_else(|e| {
            eprintln!("cannot read {}: {e}", path.display());
            process::exit(EXIT_PROGRAM_FAILURE);
        });
        let issues = lint_source(&source);
        if issues.is_empty() {
            println!("lint ok: {}", path.display());
        } else {
            for issue in issues {
                println!("{}: {}", path.display(), issue);
            }
            process::exit(EXIT_PROGRAM_FAILURE);
        }
        return;
    }
    if args.len() == 3 && args[1] == "fmt" {
        let path = Path::new(&args[2]);
        let source = read_limited_text(path, "source read").unwrap_or_else(|e| {
            eprintln!("cannot read {}: {e}", path.display());
            process::exit(EXIT_PROGRAM_FAILURE);
        });
        write_limited_text(path, &format_source(&source), "format write").unwrap_or_else(|e| {
            eprintln!("cannot write {}: {e}", path.display());
            process::exit(EXIT_PROGRAM_FAILURE);
        });
        return;
    }
    if args.len() != 2 {
        eprintln!("{CLI_HELP}");
        process::exit(EXIT_USAGE_ERROR);
    }
    let source_path = Path::new(&args[1]);
    if !source_path.exists() && source_path.extension().is_none() {
        eprintln!("{CLI_HELP}");
        process::exit(EXIT_USAGE_ERROR);
    }
    let source = read_limited_text(source_path, "source read").unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(EXIT_PROGRAM_FAILURE);
    });
    let base = Path::new(&args[1])
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    if let Err(e) = run_checked(&source, base) {
        eprintln!("Zap error: {e}");
        process::exit(EXIT_PROGRAM_FAILURE);
    }
}

#[cfg(test)]
mod tests {
    use super::CLI_HELP;

    #[test]
    fn canonical_help_lists_supported_commands() {
        for command in [
            "zap run",
            "zap fmt",
            "zap lint",
            "zap check",
            "zap test",
            "zap lock",
            "zap lock-migrate",
            "zap add",
            "zap install",
            "zap update",
            "zap registry serve",
            "zap build",
            "zap init",
            "zap lsp",
            "zap async-check",
            "zap --version",
            "zap --help",
        ] {
            assert!(CLI_HELP.contains(command), "missing help entry: {command}");
        }
        assert!(!CLI_HELP.contains("\\n"));
    }
}
