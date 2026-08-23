use super::project::{
    install_dependencies, migrate_lockfile, update_dependencies, validate_project_locked,
    write_lockfile, TestOptions,
};
use super::*;
use crate::database::{
    apply_migrations, database_plan, plan_to_json, plan_to_text, validate_project_database,
};
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
  zap new <dir>                         Create a Zap-first Web project
  zap dev [dir]                         Run the Zap-native Web server entrypoint
  zap web check [dir]                   Validate a Zap Web project
  zap db check [dir]                    Validate Web migration layout and SQL plan
  zap db plan [dir] [--json]            Show the read-only SQLite migration plan
  zap db inspect [dir] [--json]         Inspect SQLite adapter and migration status
  zap db migrate [dir] [--dry-run]      Apply SQLite migrations transactionally
  zap db migrate [dir] [--check]       Verify migrations are up to date without applying
  zap init <dir>                        Create a generic Zap project
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

fn scaffold_package_name(dir: &Path) -> String {
    let raw = dir
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("zap-app");
    let name: String = raw
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
                character
            } else {
                '-'
            }
        })
        .collect();
    if name.is_empty() {
        "zap-app".to_string()
    } else {
        name
    }
}

fn write_scaffold_file(dir: &Path, relative: &str, content: &str) -> Result<(), String> {
    let path = dir.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("cannot create {}: {error}", parent.display()))?;
    }
    fs::write(&path, content).map_err(|error| format!("cannot write {}: {error}", path.display()))
}

fn manifest_has_section(dir: &Path, section: &str) -> Result<bool, String> {
    let text = read_limited_text(&dir.join("zap.toml"), "manifest read")?;
    Ok(text
        .lines()
        .any(|line| line.trim() == format!("[{section}]")))
}

fn web_manifest_path(dir: &Path, key: &str) -> Result<PathBuf, String> {
    let text = read_limited_text(&dir.join("zap.toml"), "manifest read")?;
    if !manifest_has_section(dir, "web")? {
        return Err(format!("{} is not a Zap Web project", dir.display()));
    }
    let value = manifest_value(&text, key)
        .ok_or_else(|| format!("zap.toml: Web field `{key}` is missing"))?;
    let path = Path::new(&value);
    if path.is_absolute()
        || path
            .components()
            .any(|component| component == std::path::Component::ParentDir)
    {
        return Err(format!(
            "zap.toml: Web field `{key}` must be a safe relative path"
        ));
    }
    Ok(dir.join(path))
}

fn validate_web_command(dir: &Path) -> Result<String, String> {
    if !manifest_has_section(dir, "web")? {
        return Err(format!("{} is not a Zap Web project", dir.display()));
    }
    validate_project(dir).map(|info| format!("valid Zap Web project: {info}"))
}

fn validate_migration_layout(dir: &Path) -> Result<String, String> {
    validate_project_database(dir)
}

fn print_database_plan(dir: &Path, json_output: bool) {
    match database_plan(dir, true) {
        Ok(plan) if json_output => println!("{}", plan_to_json(&plan)),
        Ok(plan) => print!("{}", plan_to_text(&plan)),
        Err(error) => {
            eprintln!("Zap DB plan error: {error}");
            process::exit(EXIT_PROGRAM_FAILURE);
        }
    }
}

fn print_database_inspect(dir: &Path, json_output: bool) {
    match database_plan(dir, true) {
        Ok(plan) if json_output => {
            let mut output = plan_to_json(&plan);
            if let Some(object) = output.as_object_mut() {
                object.insert("mode".into(), serde_json::Value::String("inspect".into()));
                object.insert("read_only".into(), serde_json::Value::Bool(true));
                object.insert(
                    "ledger".into(),
                    serde_json::Value::String("__zap_migrations".into()),
                );
            }
            println!("{output}");
        }
        Ok(plan) => println!(
            "SQLite database inspection: driver={}, url={}, database={}, applied={}, pending={}, ledger=__zap_migrations",
            plan.config.driver,
            plan.config.url,
            plan.database_path,
            plan.applied_count,
            plan.pending.len()
        ),
        Err(error) => {
            eprintln!("Zap DB inspect error: {error}");
            process::exit(EXIT_PROGRAM_FAILURE);
        }
    }
}

fn run_database_migrate(dir: &Path, json_output: bool, dry_run: bool, check_only: bool) {
    if dry_run || check_only {
        match database_plan(dir, true) {
            Ok(plan) => {
                let up_to_date = plan.pending.is_empty();
                if json_output {
                    let mut output = plan_to_json(&plan);
                    if check_only {
                        if let Some(object) = output.as_object_mut() {
                            object.insert("ok".into(), serde_json::Value::Bool(up_to_date));
                            object.insert(
                                "check".into(),
                                serde_json::Value::String("migrations_up_to_date".into()),
                            );
                        }
                    }
                    println!("{output}");
                } else if dry_run {
                    print!("{}", plan_to_text(&plan));
                } else if up_to_date {
                    println!(
                        "SQLite migration check passed: {} applied migration(s) at {}",
                        plan.applied_count, plan.database_path
                    );
                } else {
                    print!("{}", plan_to_text(&plan));
                }
                if check_only && !up_to_date {
                    eprintln!("Zap DB migrate check failed: pending migrations exist");
                    process::exit(EXIT_PROGRAM_FAILURE);
                }
            }
            Err(error) => {
                eprintln!("Zap DB migrate check error: {error}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    match apply_migrations(dir) {
        Ok(plan) if json_output => println!("{}", plan_to_json(&plan)),
        Ok(plan) => println!(
            "SQLite migrations applied: {} applied migration(s) at {}",
            plan.applied_count, plan.database_path
        ),
        Err(error) => {
            eprintln!("Zap DB migrate error: {error}");
            process::exit(EXIT_PROGRAM_FAILURE);
        }
    }
}

fn parse_database_command(args: &[String]) -> Result<(PathBuf, bool, bool, bool), String> {
    let mut dir = PathBuf::from(".");
    let mut json_output = false;
    let mut dry_run = false;
    let mut check_only = false;
    for argument in args.iter().skip(3) {
        match argument.as_str() {
            "--json" => json_output = true,
            "--dry-run" => dry_run = true,
            "--check" => check_only = true,
            value if value.starts_with('-') => {
                return Err(format!("unknown database option: {value}"))
            }
            _value if dir != Path::new(".") => {
                return Err("database command accepts only one directory".into())
            }
            value => dir = PathBuf::from(value),
        }
    }
    if dry_run && check_only {
        return Err("--dry-run and --check cannot be combined".into());
    }
    Ok((dir, json_output, dry_run, check_only))
}

fn create_web_project(dir: &Path) -> Result<(), String> {
    if dir.exists() {
        return Err(format!(
            "cannot initialize existing path: {}",
            dir.display()
        ));
    }
    fs::create_dir_all(dir).map_err(|error| format!("cannot create {}: {error}", dir.display()))?;
    for directory in [
        "models",
        "services",
        "views",
        "public",
        "migrations",
        "tests",
    ] {
        fs::create_dir_all(dir.join(directory))
            .map_err(|error| format!("cannot create {}: {error}", dir.join(directory).display()))?;
    }
    let name = scaffold_package_name(dir);
    let manifest = format!(
        "# Zap-first Web project\n[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[web]\nroutes = \"routes.zp\"\nmodels = \"models\"\nmiddleware = \"middleware.zp\"\nmigrations = \"migrations\"\nassets = \"public\"\nadmin = \"admin.zp\"\nserver = \"server.zp\"\nserialization = \"json-by-default\"\n\n[frontend]\nframework = \"plain\"\noutput = \"public\"\nspa_fallback = \"index.html\"\n\n[database]\ndriver = \"sqlite\"\nurl = \"data/zap.sqlite3\"\n"
    );
    write_scaffold_file(dir, "zap.toml", &manifest)?;
    write_scaffold_file(
        dir,
        "web.zp",
        "# Zap-native Web primitives for this project.\nexport fn web_app(name):\n    return {\"name\": name, \"serialization\": \"json\", \"error_mode\": \"centralized\"}\n\nexport fn web_route(method, path, handler, scope):\n    return {\"method\": method, \"path\": path, \"handler\": handler, \"scope\": scope}\n",
    )?;
    write_scaffold_file(
        dir,
        "public/index.html",
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Zap Web</title>
  <link rel="stylesheet" href="/assets/app.css">
</head>
<body>
  <main>
    <h1>Zap Web</h1>
    <p>A browser-native frontend running on a Zap JSON API.</p>
    <ul id="tasks" aria-live="polite"></ul>
  </main>
  <script type="module" src="/assets/app.js"></script>
</body>
</html>
"#,
    )?;
    write_scaffold_file(
        dir,
        "public/assets/app.css",
        r#"body { font-family: system-ui, sans-serif; margin: 3rem auto; max-width: 42rem; padding: 0 1rem; }
li { margin: .5rem 0; }
"#,
    )?;
    write_scaffold_file(
        dir,
        "public/assets/app.js",
        r##"const list = document.querySelector("#tasks");
const response = await fetch("/api/tasks", { headers: { Accept: "application/json" } });
if (!response.ok) throw new Error(`API returned ${response.status}`);
const payload = await response.json();
for (const task of payload.tasks) {
  const item = document.createElement("li");
  item.textContent = `${task.done ? "✓" : "○"} ${task.title}`;
  list.append(item);
}
"##,
    )?;
    write_scaffold_file(
        dir,
        "routes.zp",
        "export fn routes():\n    return [{\"method\": \"GET\", \"path\": \"/\", \"handler\": \"home\", \"scope\": \"\"}, {\"method\": \"GET\", \"path\": \"/health\", \"handler\": \"health\", \"scope\": \"\"}, {\"method\": \"GET\", \"path\": \"/assets/*path\", \"handler\": \"frontend_asset\", \"scope\": \"\"}, {\"method\": \"GET\", \"path\": \"/api/tasks\", \"handler\": \"frontend_tasks_api\", \"scope\": \"tasks:read\"}, {\"method\": \"GET\", \"path\": \"/users/:id\", \"handler\": \"get_user\", \"scope\": \"users:read\"}, {\"method\": \"POST\", \"path\": \"/users\", \"handler\": \"create_user\", \"scope\": \"users:write\"}, {\"method\": \"GET\", \"path\": \"/*path\", \"handler\": \"frontend_spa\", \"scope\": \"\"}]\n",
    )?;
    write_scaffold_file(
        dir,
        "models/user.zp",
        "export fn user_model():\n    return {\"name\": \"User\", \"table\": \"users\", \"fields\": {\"id\": \"number primary_key\", \"name\": \"text required\", \"email\": \"email unique\"}}\n",
    )?;
    write_scaffold_file(
        dir,
        "services/user_service.zp",
        "export fn home(request):\n    return web_static(\"index.html\", \"public\")\n\nexport fn health(request):\n    return {\"status\": 200, \"body\": json({\"status\": \"ok\", \"request_id\": request[\"request_id\"]})}\n\nexport fn frontend_asset(request):\n    return web_static(\"assets/\" + request[\"params\"][\"path\"], \"public\")\n\nexport fn frontend_spa(request):\n    return web_static_spa(request[\"params\"][\"path\"], \"public\", \"index.html\")\n\nexport fn frontend_tasks_api(request):\n    return {\"status\": 200, \"body\": json({\"tasks\": [{\"id\": 1, \"title\": \"Try the Zap API\", \"done\": false}], \"summary\": {\"total\": 1, \"completed\": 0, \"remaining\": 1}, \"request_id\": request[\"request_id\"]})}\n\nexport fn get_user(request):\n    return {\"status\": 200, \"body\": json({\"id\": request[\"params\"][\"id\"], \"request_id\": request[\"request_id\"]})}\n\nexport fn create_user(request):\n    return {\"status\": 201, \"body\": json({\"created\": true, \"body\": request[\"body\"], \"request_id\": request[\"request_id\"]})}\n",
    )?;
    write_scaffold_file(
        dir,
        "middleware.zp",
        "export fn middleware_stack():\n    return [{\"name\": \"request_id\", \"stage\": \"before\", \"order\": 10}, {\"name\": \"security_headers\", \"stage\": \"after\", \"order\": 90}, {\"name\": \"auth\", \"stage\": \"before_handler\", \"order\": 40}]\n",
    )?;
    write_scaffold_file(
        dir,
        "admin.zp",
        "export fn admin_registry():\n    return [{\"model\": \"User\", \"list\": [\"id\", \"name\", \"email\"], \"permissions\": [\"admin:read\", \"admin:write\"]}]\n",
    )?;
    write_scaffold_file(
        dir,
        "migrations/0001_initial.zp",
        "export fn migration():\n    return {\"id\": \"0001_initial\", \"depends_on\": [], \"operations\": [{\"kind\": \"create_table\", \"table\": \"users\", \"columns\": {\"id\": \"integer primary key\", \"name\": \"text not null\", \"email\": \"text not null unique\"}}]}\n",
    )?;
    write_scaffold_file(
        dir,
        "main.zp",
        "# Zap-first Web project entrypoint.\nimport \"web\"\nimport \"routes\"\nimport \"models/user\"\nimport \"services/user_service\"\nimport \"middleware\"\nimport \"admin\"\nlet app = web_app(\"APP_NAME\")\nlet route_table = routes()\nlet model = user_model()\nlet middleware_table = middleware_stack()\nlet admin_table = admin_registry()\nsay json({\"framework\": \"zap-web\", \"app\": app, \"routes\": route_table, \"model\": model, \"middleware\": middleware_table, \"admin\": admin_table})\n",
    )?;
    write_scaffold_file(
        dir,
        "server.zp",
        "# Zap-native Web development server.\nimport \"web\"\nimport \"routes\"\nimport \"services/user_service\"\nlet port = from_json(env_get(\"ZAP_WEB_PORT\", \"3000\"))\nlet result = web_serve(routes(), port, 0)\nsay json(result)\n",
    )?;
    write_scaffold_file(
        dir,
        "tests/web_test.zp",
        "import \"routes\"\n\nlet route_table = routes()\nassert(len(route_table) == 7, \"web scaffold must contain seven starter routes\")\nassert(route_table[0][\"path\"] == \"/\", \"root route must be present\")\nassert(route_table[2][\"path\"] == \"/assets/*path\", \"asset route must be present\")\nassert(route_table[3][\"path\"] == \"/api/tasks\", \"JSON API route must be present\")\nassert(route_table[5][\"scope\"] == \"users:write\", \"write scope must be explicit\")\nassert(route_table[6][\"handler\"] == \"frontend_spa\", \"SPA fallback route must be present\")\nsay \"Zap Web scaffold test passed\"\n",
    )?;
    Ok(())
}

fn run_dev_command(dir: &Path) {
    if let Err(error) = validate_web_command(dir) {
        eprintln!("Zap dev error: {error}");
        process::exit(EXIT_PROGRAM_FAILURE);
    }
    let server = match web_manifest_path(dir, "server") {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Zap dev error: {error}");
            process::exit(EXIT_PROGRAM_FAILURE);
        }
    };
    let source = read_limited_text(&server, "Web server source read").unwrap_or_else(|error| {
        eprintln!("Zap dev error: {error}");
        process::exit(EXIT_PROGRAM_FAILURE);
    });
    if let Err(error) = run_checked(&source, dir) {
        eprintln!("Zap dev error: {error}");
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
    if args.len() == 3 && args[1] == "new" {
        let dir = Path::new(&args[2]);
        if let Err(error) = create_web_project(dir) {
            eprintln!("Zap new error: {error}");
            process::exit(EXIT_PROGRAM_FAILURE);
        }
        println!("Created Zap Web project: {}", dir.display());
        return;
    }
    if args.len() == 3 && args[1] == "web" && args[2] == "check" {
        match validate_web_command(Path::new(".")) {
            Ok(info) => println!("{info}"),
            Err(error) => {
                eprintln!("Zap Web check error: {error}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 4 && args[1] == "web" && args[2] == "check" {
        match validate_web_command(Path::new(&args[3])) {
            Ok(info) => println!("{info}"),
            Err(error) => {
                eprintln!("Zap Web check error: {error}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() >= 3
        && args[1] == "db"
        && matches!(args[2].as_str(), "inspect" | "plan" | "migrate")
    {
        let (dir, json_output, dry_run, check_only) = match parse_database_command(args) {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Zap DB usage error: {error}");
                process::exit(EXIT_USAGE_ERROR);
            }
        };
        if args[2] == "inspect" {
            if dry_run || check_only {
                eprintln!("Zap DB usage error: `db inspect` accepts only `--json`");
                process::exit(EXIT_USAGE_ERROR);
            }
            print_database_inspect(&dir, json_output);
        } else if args[2] == "plan" {
            if dry_run || check_only {
                eprintln!("Zap DB usage error: `db plan` accepts only `--json`");
                process::exit(EXIT_USAGE_ERROR);
            }
            print_database_plan(&dir, json_output);
        } else {
            run_database_migrate(&dir, json_output, dry_run, check_only);
        }
        return;
    }
    if args.len() == 3 && args[1] == "db" && args[2] == "check" {
        match validate_migration_layout(Path::new(".")) {
            Ok(info) => println!("{info}"),
            Err(error) => {
                eprintln!("Zap DB check error: {error}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
        return;
    }
    if args.len() == 4 && args[1] == "db" && args[2] == "check" {
        match validate_migration_layout(Path::new(&args[3])) {
            Ok(info) => println!("{info}"),
            Err(error) => {
                eprintln!("Zap DB check error: {error}");
                process::exit(EXIT_PROGRAM_FAILURE);
            }
        }
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
        match validate_project_locked(Path::new(".")) {
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
        match validate_project_locked(dir) {
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
    if args.len() == 2 && args[1] == "dev" {
        run_dev_command(Path::new("."));
        return;
    }
    if args.len() == 3 && args[1] == "dev" {
        run_dev_command(Path::new(&args[2]));
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
            "zap new",
            "zap web check",
            "zap db check",
            "zap db plan",
            "zap db inspect",
            "zap db migrate",
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
