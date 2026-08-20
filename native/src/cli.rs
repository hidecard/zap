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
  zap update [dir]     Regenerate zap.lock from zap.toml
  zap registry gc [--dry-run] [dir]     Remove unreferenced registry cache files
  zap build [dir]                       Validate and prepare a project
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
                    println!("{action}: {}", path.display());
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
        match crate::registry::cache_package_source(&args[4], &cache, &package) {
            Ok(path) => println!("cached package: {}", path.display()),
            Err(error) => {
                eprintln!("Zap registry error: {error}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 8 && args[1] == "registry" && args[2] == "publish" {
        let package = crate::registry::RegistryPackage {
            name: args[5].clone(),
            version: args[6].clone(),
            source: args[4].clone(),
            checksum: args[7].to_ascii_lowercase(),
            dependencies: std::collections::BTreeMap::new(),
        };
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
        let base = Path::new(&args[2]).parent().unwrap_or(Path::new("."));
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
    let base = Path::new(&args[1]).parent().unwrap_or(Path::new("."));
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
