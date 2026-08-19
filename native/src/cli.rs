use super::project::TestOptions;
use super::*;

pub const EXIT_PROGRAM_FAILURE: i32 = 1;
pub const EXIT_USAGE_ERROR: i32 = 2;

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
        println!("zap 0.9.2 (native)");
        return;
    }
    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        println!("Zap native runtime\n\nUsage:\n  zap <file.zp>       Run a Zap source file\n  zap run <file.zp>   Run a Zap source file explicitly\n  zap fmt <file.zp>   Format a Zap source file\n  zap check [dir]      Validate zap.toml and the project entry file\n  zap test [dir]       Run *_test.zp files in a tests directory
  zap lint <file.zp>   Check formatting and style warnings
  zap check --json     Validate a project with JSON diagnostics\n  zap build [dir]      Validate and prepare a Zap project\n  zap init <dir>       Create a new Zap project\n  zap --version        Show the version\n  zap --help           Show this help");
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
        eprintln!("Usage: zap <file.zp>\n       zap run <file.zp>\n       zap fmt <file.zp>\n       zap lint <file.zp>\n       zap check [dir]\n       zap check --json [dir]\n       zap test [dir]\n       zap build [dir]\n       zap init <dir>\n       zap --version");
        process::exit(EXIT_USAGE_ERROR);
    }
    let source = read_limited_text(Path::new(&args[1]), "source read").unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(EXIT_PROGRAM_FAILURE);
    });
    let base = Path::new(&args[1]).parent().unwrap_or(Path::new("."));
    if let Err(e) = run_checked(&source, base) {
        eprintln!("Zap error: {e}");
        process::exit(EXIT_PROGRAM_FAILURE);
    }
}
