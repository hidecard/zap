use rusqlite::{params, Connection, OpenFlags, Row};
use serde_json::{json, Value as JsonValue};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::ast::{parse_program, Expr, Literal, Stmt};
use crate::{read_limited_text, Value};

const DEFAULT_DATABASE_URL: &str = "data/zap.sqlite3";
const MIGRATION_TABLE: &str = "__zap_migrations";
const MAX_MIGRATIONS: usize = 1024;
const MAX_OPERATIONS_PER_MIGRATION: usize = 256;
const MAX_IDENTIFIER_BYTES: usize = 64;
const MAX_SQL_BYTES: usize = 1024 * 1024;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DatabaseConfig {
    pub(crate) driver: String,
    pub(crate) url: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct UserRecord {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) email: String,
}

#[allow(dead_code)]
pub(crate) trait DatabaseAdapter {
    fn driver(&self) -> &str;
    fn database_path(&self) -> &str;
    fn find_user(&self, id: i64) -> Result<Option<UserRecord>, String>;
    fn insert_user(&mut self, name: &str, email: &str) -> Result<UserRecord, String>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum MigrationOperation {
    CreateTable {
        table: String,
        columns: BTreeMap<String, String>,
    },
    AddColumn {
        table: String,
        column: String,
        definition: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MigrationSpec {
    pub(crate) id: String,
    pub(crate) depends_on: Vec<String>,
    pub(crate) operations: Vec<MigrationOperation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct PlannedMigration {
    pub(crate) id: String,
    pub(crate) depends_on: Vec<String>,
    pub(crate) sql: Vec<String>,
    pub(crate) checksum: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DatabasePlan {
    pub(crate) config: DatabaseConfig,
    pub(crate) database_path: String,
    pub(crate) applied_count: usize,
    pub(crate) pending: Vec<PlannedMigration>,
}

fn parse_quoted_value(value: &str, context: &str) -> Result<String, String> {
    if value.len() < 2 || !value.starts_with('"') || !value.ends_with('"') {
        return Err(format!("{context} must be a quoted string"));
    }
    let inner = &value[1..value.len() - 1];
    if inner.contains(['\n', '\r']) {
        return Err(format!("{context} must not contain newlines"));
    }
    Ok(inner.to_string())
}

fn safe_relative_path(value: &str, context: &str) -> Result<(), String> {
    let path = Path::new(value);
    if value.is_empty()
        || path.is_absolute()
        || path
            .components()
            .any(|component| component == std::path::Component::ParentDir)
    {
        return Err(format!("{context} must be a safe relative path"));
    }
    Ok(())
}

fn parse_database_fields(manifest: &str) -> Result<Option<DatabaseConfig>, String> {
    let mut in_database = false;
    let mut saw_database = false;
    let mut fields = BTreeMap::new();
    for raw_line in manifest.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') {
            in_database = line == "[database]";
            saw_database |= in_database;
            continue;
        }
        if !in_database {
            continue;
        }
        let Some((key, raw_value)) = line.split_once('=') else {
            return Err(format!("zap.toml: invalid database entry `{line}`"));
        };
        let key = key.trim();
        if !matches!(key, "driver" | "url") {
            return Err(format!("zap.toml: unknown Database field `{key}`"));
        }
        if fields
            .insert(
                key.to_string(),
                parse_quoted_value(
                    raw_value.trim(),
                    &format!("zap.toml: Database field `{key}`"),
                )?,
            )
            .is_some()
        {
            return Err(format!("zap.toml: duplicate Database field `{key}`"));
        }
    }
    if !saw_database {
        return Ok(None);
    }
    let driver = fields
        .remove("driver")
        .ok_or("zap.toml: Database field `driver` is required".to_string())?;
    let url = fields
        .remove("url")
        .ok_or("zap.toml: Database field `url` is required".to_string())?;
    if driver != "sqlite" {
        return Err(format!(
            "zap.toml: unsupported database driver `{driver}`; only `sqlite` is implemented"
        ));
    }
    if url != ":memory:" {
        let path = url.strip_prefix("sqlite://").unwrap_or(&url);
        safe_relative_path(path, "zap.toml: Database url")?;
    }
    Ok(Some(DatabaseConfig { driver, url }))
}

pub(crate) fn validate_database_manifest(manifest: &str) -> Result<(), String> {
    let _ = parse_database_fields(manifest)?;
    Ok(())
}

pub(crate) fn database_config(dir: &Path) -> Result<DatabaseConfig, String> {
    let manifest = read_limited_text(&dir.join("zap.toml"), "manifest read")?;
    Ok(
        parse_database_fields(&manifest)?.unwrap_or_else(|| DatabaseConfig {
            driver: "sqlite".into(),
            url: DEFAULT_DATABASE_URL.into(),
        }),
    )
}

fn resolved_database_url(
    dir: &Path,
    config: &DatabaseConfig,
) -> Result<(String, Option<PathBuf>), String> {
    let from_environment = env::var("ZAP_DATABASE_URL").ok();
    let raw = from_environment
        .as_deref()
        .unwrap_or(config.url.as_str())
        .trim();
    if raw.is_empty() {
        return Err("database URL must not be empty".into());
    }
    if raw == ":memory:" {
        return Ok((raw.to_string(), None));
    }
    if raw.contains("://") && !raw.starts_with("sqlite://") {
        return Err("database URL must use `sqlite://`, a relative path, or `:memory:`".into());
    }
    let path_text = raw.strip_prefix("sqlite://").unwrap_or(raw);
    let path = PathBuf::from(path_text);
    if from_environment.is_none() {
        safe_relative_path(path_text, "zap.toml: Database url")?;
    }
    if path_text.is_empty()
        || path
            .components()
            .any(|component| component == std::path::Component::ParentDir)
    {
        return Err("database URL must not contain parent-directory traversal".into());
    }
    let resolved = if path.is_absolute() {
        path
    } else {
        dir.join(path)
    };
    Ok((resolved.display().to_string(), Some(resolved)))
}

fn is_safe_migration_id(value: &str) -> bool {
    value.len() <= 128
        && !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
}

fn is_safe_identifier(value: &str) -> bool {
    value.len() <= MAX_IDENTIFIER_BYTES
        && !value.is_empty()
        && value.bytes().enumerate().all(|(index, byte)| {
            if index == 0 {
                byte.is_ascii_alphabetic() || byte == b'_'
            } else {
                byte.is_ascii_alphanumeric() || byte == b'_'
            }
        })
}

fn quote_identifier(value: &str, context: &str) -> Result<String, String> {
    if !is_safe_identifier(value) {
        return Err(format!("{context} is not a safe SQL identifier"));
    }
    Ok(format!("\"{value}\""))
}

fn normalize_definition(value: &str, context: &str) -> Result<String, String> {
    let tokens = value.split_whitespace().collect::<Vec<_>>();
    if tokens.is_empty() || tokens.len() > 8 {
        return Err(format!(
            "{context} must contain a bounded SQLite column definition"
        ));
    }
    let allowed_types = ["integer", "real", "text", "blob", "boolean"];
    if !allowed_types.contains(&tokens[0].to_ascii_lowercase().as_str()) {
        return Err(format!("{context} has an unsupported SQLite type"));
    }
    let allowed_modifiers = ["primary", "key", "not", "null", "unique", "autoincrement"];
    if tokens[1..]
        .iter()
        .any(|token| !allowed_modifiers.contains(&token.to_ascii_lowercase().as_str()))
    {
        return Err(format!("{context} contains an unsupported column modifier"));
    }
    Ok(tokens
        .into_iter()
        .map(|token| token.to_ascii_uppercase())
        .collect::<Vec<_>>()
        .join(" "))
}

fn map_text<'a>(
    map: &'a HashMap<String, Value>,
    key: &str,
    context: &str,
) -> Result<&'a str, String> {
    match map.get(key) {
        Some(Value::Text(value)) if !value.is_empty() => Ok(value),
        _ => Err(format!("{context} requires text field `{key}`")),
    }
}

fn literal_expr_to_value(expression: &Expr) -> Result<Value, String> {
    match expression {
        Expr::Literal(Literal::Number(value)) => Ok(Value::Number(*value)),
        Expr::Literal(Literal::Text(value)) => Ok(Value::Text(value.clone())),
        Expr::Literal(Literal::Bool(value)) => Ok(Value::Bool(*value)),
        Expr::Literal(Literal::None) => Ok(Value::None),
        Expr::List(items) => Ok(Value::List(
            items
                .iter()
                .map(|item| literal_expr_to_value(&item.node))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        Expr::Map(entries) => {
            let mut map = HashMap::new();
            for (key, value) in entries {
                let Value::Text(key) = literal_expr_to_value(&key.node)? else {
                    return Err("migration map keys must be text literals".into());
                };
                map.insert(key, literal_expr_to_value(&value.node)?);
            }
            Ok(Value::Map(map))
        }
        _ => Err("migration() must return only literal maps and lists; calls, names, and expressions are not allowed".into()),
    }
}

fn parse_operation(
    value: &Value,
    migration_id: &str,
    index: usize,
) -> Result<MigrationOperation, String> {
    let Value::Map(map) = value else {
        return Err(format!(
            "migration `{migration_id}` operation {index} must be a map"
        ));
    };
    let kind = map_text(map, "kind", "migration operation")?;
    match kind {
        "create_table" => {
            let table = map_text(map, "table", "create_table operation")?;
            quote_identifier(table, "create_table table")?;
            let Value::Map(columns) = map
                .get("columns")
                .ok_or_else(|| "create_table operation requires map `columns`".to_string())?
            else {
                return Err("create_table operation requires map `columns`".into());
            };
            if columns.is_empty() || columns.len() > 128 {
                return Err("create_table operation requires between 1 and 128 columns".into());
            }
            let mut normalized = BTreeMap::new();
            for (column, definition) in columns {
                quote_identifier(column, "create_table column")?;
                let Value::Text(definition) = definition else {
                    return Err("create_table column definitions must be text".into());
                };
                normalized.insert(
                    column.clone(),
                    normalize_definition(definition, "create_table column definition")?,
                );
            }
            Ok(MigrationOperation::CreateTable {
                table: table.to_string(),
                columns: normalized,
            })
        }
        "add_column" => {
            let table = map_text(map, "table", "add_column operation")?;
            let column = map_text(map, "column", "add_column operation")?;
            let definition = map_text(map, "definition", "add_column operation")?;
            quote_identifier(table, "add_column table")?;
            quote_identifier(column, "add_column column")?;
            Ok(MigrationOperation::AddColumn {
                table: table.to_string(),
                column: column.to_string(),
                definition: normalize_definition(definition, "add_column definition")?,
            })
        }
        _ => Err(format!(
            "migration `{migration_id}` operation {index} has unsupported kind `{kind}`"
        )),
    }
}

fn parse_migration_value(value: Value, source: &Path) -> Result<MigrationSpec, String> {
    let Value::Map(map) = value else {
        return Err(format!(
            "{}: migration() must return a map",
            source.display()
        ));
    };
    let id = map_text(&map, "id", "migration")?.to_string();
    if !is_safe_migration_id(&id) {
        return Err(format!("{}: migration id is not safe", source.display()));
    }
    let depends_on = match map.get("depends_on") {
        Some(Value::List(items)) => items
            .iter()
            .map(|item| match item {
                Value::Text(value) if is_safe_migration_id(value) => Ok(value.clone()),
                _ => Err(format!(
                    "{}: depends_on values must be safe text ids",
                    source.display()
                )),
            })
            .collect::<Result<Vec<_>, _>>()?,
        None => Vec::new(),
        _ => return Err(format!("{}: depends_on must be a list", source.display())),
    };
    let Value::List(operations) = map
        .get("operations")
        .ok_or_else(|| format!("{}: migration requires operations", source.display()))?
    else {
        return Err(format!("{}: operations must be a list", source.display()));
    };
    if operations.is_empty() || operations.len() > MAX_OPERATIONS_PER_MIGRATION {
        return Err(format!(
            "{}: operations must contain between 1 and {} entries",
            source.display(),
            MAX_OPERATIONS_PER_MIGRATION
        ));
    }
    let operations = operations
        .iter()
        .enumerate()
        .map(|(index, operation)| parse_operation(operation, &id, index))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(MigrationSpec {
        id,
        depends_on,
        operations,
    })
}

fn load_migration(path: &Path) -> Result<MigrationSpec, String> {
    let source = read_limited_text(path, "migration read")?;
    let program = parse_program(&source).map_err(|error| format!("{}: {error}", path.display()))?;
    let mut migration_body = None;
    for statement in program.statements {
        match statement.node {
            Stmt::Function {
                name,
                params,
                body,
                exported: true,
                ..
            } if name == "migration" => {
                if !params.is_empty() || migration_body.replace(body).is_some() {
                    return Err(format!(
                        "{}: migration() must be exported once with no parameters",
                        path.display()
                    ));
                }
            }
            _ => {
                return Err(format!(
                    "{}: migration files may contain only `export fn migration()`",
                    path.display()
                ));
            }
        }
    }
    let body = migration_body
        .ok_or_else(|| format!("{}: migration() export not found", path.display()))?;
    if body.statements.len() != 1 {
        return Err(format!(
            "{}: migration() must contain one return statement",
            path.display()
        ));
    }
    let Stmt::Return(Some(expression)) = &body.statements[0].node else {
        return Err(format!(
            "{}: migration() must return a literal map",
            path.display()
        ));
    };
    parse_migration_value(literal_expr_to_value(&expression.node)?, path)
}

fn migration_directory(dir: &Path) -> Result<PathBuf, String> {
    let manifest = read_limited_text(&dir.join("zap.toml"), "manifest read")?;
    let mut in_web = false;
    let mut migrations = None;
    for raw_line in manifest.lines() {
        let line = raw_line.trim();
        if line.starts_with('[') {
            in_web = line == "[web]";
            continue;
        }
        if !in_web {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            if key.trim() == "migrations" {
                migrations = Some(parse_quoted_value(
                    value.trim(),
                    "zap.toml: Web migrations",
                )?);
            }
        }
    }
    let value = migrations.ok_or("zap.toml: Web field `migrations` is required".to_string())?;
    safe_relative_path(&value, "zap.toml: Web migrations")?;
    Ok(dir.join(value))
}

fn discover_migrations(dir: &Path) -> Result<Vec<MigrationSpec>, String> {
    let path = migration_directory(dir)?;
    if !path.is_dir() {
        return Err(format!("migration directory not found: {}", path.display()));
    }
    let mut files = fs::read_dir(&path)
        .map_err(|error| format!("cannot read migrations: {error}"))?
        .map(|entry| {
            entry
                .map(|item| item.path())
                .map_err(|error| error.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    files.sort();
    let files = files
        .into_iter()
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("zp"))
        .collect::<Vec<_>>();
    if files.is_empty() {
        return Err(format!("no .zp migrations found in {}", path.display()));
    }
    if files.len() > MAX_MIGRATIONS {
        return Err(format!(
            "migration directory contains more than {MAX_MIGRATIONS} files"
        ));
    }
    files.iter().map(|path| load_migration(path)).collect()
}

fn operation_sql(operation: &MigrationOperation) -> Result<String, String> {
    match operation {
        MigrationOperation::CreateTable { table, columns } => {
            let table = quote_identifier(table, "create_table table")?;
            let columns = columns
                .iter()
                .map(|(column, definition)| {
                    Ok(format!(
                        "{} {}",
                        quote_identifier(column, "create_table column")?,
                        definition
                    ))
                })
                .collect::<Result<Vec<_>, String>>()?;
            Ok(format!("CREATE TABLE {table} ({});", columns.join(", ")))
        }
        MigrationOperation::AddColumn {
            table,
            column,
            definition,
        } => Ok(format!(
            "ALTER TABLE {} ADD COLUMN {} {};",
            quote_identifier(table, "add_column table")?,
            quote_identifier(column, "add_column column")?,
            definition
        )),
    }
}

fn migration_sql(migration: &MigrationSpec) -> Result<Vec<String>, String> {
    let sql = migration
        .operations
        .iter()
        .map(operation_sql)
        .collect::<Result<Vec<_>, _>>()?;
    let total = sql.iter().map(String::len).sum::<usize>();
    if total > MAX_SQL_BYTES {
        return Err(format!(
            "migration `{}` SQL exceeds the {MAX_SQL_BYTES} byte limit",
            migration.id
        ));
    }
    Ok(sql)
}

fn migration_json(migration: &MigrationSpec) -> JsonValue {
    let operations = migration
        .operations
        .iter()
        .map(|operation| match operation {
            MigrationOperation::CreateTable { table, columns } => json!({
                "kind": "create_table",
                "table": table,
                "columns": columns,
            }),
            MigrationOperation::AddColumn {
                table,
                column,
                definition,
            } => json!({
                "kind": "add_column",
                "table": table,
                "column": column,
                "definition": definition,
            }),
        })
        .collect::<Vec<_>>();
    json!({
        "id": migration.id,
        "depends_on": migration.depends_on,
        "operations": operations,
    })
}

fn migration_checksum(migration: &MigrationSpec) -> String {
    let bytes = serde_json::to_vec(&migration_json(migration)).unwrap_or_default();
    let digest = Sha256::digest(bytes);
    let mut checksum = String::with_capacity(digest.len() * 2);
    for byte in digest {
        let _ = write!(&mut checksum, "{byte:02x}");
    }
    checksum
}

fn visit_migration(
    id: &str,
    by_id: &HashMap<String, MigrationSpec>,
    applied: &HashSet<String>,
    states: &mut HashMap<String, u8>,
    ordered: &mut Vec<MigrationSpec>,
) -> Result<(), String> {
    match states.get(id).copied() {
        Some(2) => return Ok(()),
        Some(1) => return Err(format!("circular migration dependency involving `{id}`")),
        _ => {}
    }
    states.insert(id.to_string(), 1);
    let migration = by_id
        .get(id)
        .ok_or_else(|| format!("migration dependency `{id}` is not present or already applied"))?;
    for dependency in &migration.depends_on {
        if by_id.contains_key(dependency) {
            visit_migration(dependency, by_id, applied, states, ordered)?;
        } else if !applied.contains(dependency) {
            return Err(format!(
                "migration `{}` depends on missing migration `{dependency}`",
                migration.id
            ));
        }
    }
    states.insert(id.to_string(), 2);
    if !applied.contains(id) {
        ordered.push(migration.clone());
    }
    Ok(())
}

fn ordered_pending(
    migrations: &[MigrationSpec],
    applied: &HashSet<String>,
) -> Result<Vec<MigrationSpec>, String> {
    let mut by_id = HashMap::new();
    for migration in migrations {
        if by_id
            .insert(migration.id.clone(), migration.clone())
            .is_some()
        {
            return Err(format!("duplicate migration id `{}`", migration.id));
        }
    }
    let mut states = HashMap::new();
    let mut ordered = Vec::new();
    let ids = migrations
        .iter()
        .map(|migration| migration.id.clone())
        .collect::<Vec<_>>();
    for id in ids {
        visit_migration(&id, &by_id, applied, &mut states, &mut ordered)?;
    }
    Ok(ordered)
}

fn open_connection(
    dir: &Path,
    config: &DatabaseConfig,
    read_only: bool,
) -> Result<(Connection, String), String> {
    let (database_path, path) = resolved_database_url(dir, config)?;
    let connection = match path {
        None => Connection::open_in_memory(),
        Some(path) if read_only && !path.exists() => Connection::open_in_memory(),
        Some(path) if read_only => Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI,
        ),
        Some(path) => {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|error| format!("cannot create database directory: {error}"))?;
            }
            Connection::open(path)
        }
    }
    .map_err(|error| format!("SQLite connection failed: {error}"))?;
    connection
        .busy_timeout(std::time::Duration::from_secs(5))
        .map_err(|error| format!("SQLite busy timeout failed: {error}"))?;
    connection
        .execute_batch("PRAGMA foreign_keys = ON;")
        .map_err(|error| format!("SQLite foreign-key policy failed: {error}"))?;
    Ok((connection, database_path))
}

#[allow(dead_code)]
fn normalize_user_input(name: &str, email: &str) -> Result<(String, String), String> {
    let name = name.trim().to_string();
    let email = email.trim().to_lowercase();
    if name.is_empty() || name.chars().count() > 120 {
        return Err("user name length is outside the allowed range".into());
    }
    if email.is_empty() || email.chars().count() > 254 || !email.contains('@') {
        return Err("user email format is not accepted".into());
    }
    Ok((name, email))
}

#[allow(dead_code)]
fn user_record_from_row(row: &Row<'_>) -> rusqlite::Result<UserRecord> {
    Ok(UserRecord {
        id: row.get(0)?,
        name: row.get(1)?,
        email: row.get(2)?,
    })
}

#[allow(dead_code)]
pub(crate) struct SqliteDatabaseAdapter {
    connection: Connection,
    database_path: String,
}

pub(crate) struct SqliteTransaction<'connection> {
    transaction: Option<rusqlite::Transaction<'connection>>,
}

impl SqliteDatabaseAdapter {
    #[allow(dead_code)]
    pub(crate) fn open(dir: &Path) -> Result<Self, String> {
        let config = database_config(dir)?;
        let (connection, database_path) = open_connection(dir, &config, false)?;
        Ok(Self {
            connection,
            database_path,
        })
    }

    #[allow(dead_code)]
    pub(crate) fn begin(&mut self) -> Result<SqliteTransaction<'_>, String> {
        let transaction = self
            .connection
            .transaction()
            .map_err(|error| format!("SQLite transaction begin failed: {error}"))?;
        Ok(SqliteTransaction {
            transaction: Some(transaction),
        })
    }
}

impl DatabaseAdapter for SqliteDatabaseAdapter {
    fn driver(&self) -> &str {
        "sqlite"
    }

    fn database_path(&self) -> &str {
        &self.database_path
    }

    fn find_user(&self, id: i64) -> Result<Option<UserRecord>, String> {
        if id <= 0 {
            return Err("user id must be positive".into());
        }
        match self.connection.query_row(
            "SELECT id, name, email FROM users WHERE id = ?1",
            params![id],
            user_record_from_row,
        ) {
            Ok(record) => Ok(Some(record)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(format!("SQLite user lookup failed: {error}")),
        }
    }

    fn insert_user(&mut self, name: &str, email: &str) -> Result<UserRecord, String> {
        let mut transaction = self.begin()?;
        let record = transaction.insert_user(name, email)?;
        transaction.commit()?;
        Ok(record)
    }
}

impl<'connection> SqliteTransaction<'connection> {
    #[allow(dead_code)]
    pub(crate) fn insert_user(&mut self, name: &str, email: &str) -> Result<UserRecord, String> {
        let (name, email) = normalize_user_input(name, email)?;
        let transaction = self
            .transaction
            .as_mut()
            .ok_or_else(|| "SQLite transaction is already closed".to_string())?;
        transaction
            .execute(
                "INSERT INTO users (name, email) VALUES (?1, ?2)",
                params![name, email],
            )
            .map_err(|error| format!("SQLite user insert failed: {error}"))?;
        let id = transaction.last_insert_rowid();
        transaction
            .query_row(
                "SELECT id, name, email FROM users WHERE id = ?1",
                params![id],
                user_record_from_row,
            )
            .map_err(|error| format!("SQLite inserted-user read failed: {error}"))
    }

    #[allow(dead_code)]
    pub(crate) fn commit(mut self) -> Result<(), String> {
        let transaction = self
            .transaction
            .take()
            .ok_or_else(|| "SQLite transaction is already closed".to_string())?;
        transaction
            .commit()
            .map_err(|error| format!("SQLite transaction commit failed: {error}"))
    }

    #[allow(dead_code)]
    pub(crate) fn rollback(mut self) -> Result<(), String> {
        let transaction = self
            .transaction
            .take()
            .ok_or_else(|| "SQLite transaction is already closed".to_string())?;
        transaction
            .rollback()
            .map_err(|error| format!("SQLite transaction rollback failed: {error}"))
    }
}

fn read_applied(connection: &Connection) -> Result<BTreeMap<String, String>, String> {
    let exists: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
            params![MIGRATION_TABLE],
            |row| row.get(0),
        )
        .map_err(|error| format!("SQLite migration ledger check failed: {error}"))?;
    if exists == 0 {
        return Ok(BTreeMap::new());
    }
    let mut statement = connection
        .prepare(&format!(
            "SELECT id, checksum FROM \"{MIGRATION_TABLE}\" ORDER BY id"
        ))
        .map_err(|error| format!("SQLite migration ledger read failed: {error}"))?;
    let rows = statement
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|error| format!("SQLite migration ledger query failed: {error}"))?;
    rows.collect::<Result<BTreeMap<_, _>, _>>()
        .map_err(|error| format!("SQLite migration ledger row failed: {error}"))
}

fn validate_applied_migrations(
    migrations: &[MigrationSpec],
    applied: &BTreeMap<String, String>,
) -> Result<(), String> {
    let known = migrations
        .iter()
        .map(|migration| (migration.id.as_str(), migration))
        .collect::<HashMap<_, _>>();
    for (id, checksum) in applied {
        let Some(migration) = known.get(id.as_str()) else {
            return Err(format!(
                "applied migration `{id}` is missing from the migration directory"
            ));
        };
        if checksum != &migration_checksum(migration) {
            return Err(format!(
                "migration `{id}` changed after it was applied; create a new migration instead"
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_project_database(dir: &Path) -> Result<String, String> {
    let config = database_config(dir)?;
    let migrations = discover_migrations(dir)?;
    let applied = HashSet::new();
    let pending = ordered_pending(&migrations, &applied)?;
    for migration in &pending {
        let _ = migration_sql(migration)?;
    }
    Ok(format!(
        "valid Zap DB migration layout: SQLite adapter `{}`; {} migration file(s); {} pending plan step(s)",
        config.url,
        migrations.len(),
        pending.len()
    ))
}

pub(crate) fn database_plan(dir: &Path, read_only: bool) -> Result<DatabasePlan, String> {
    let config = database_config(dir)?;
    let (connection, database_path) = open_connection(dir, &config, read_only)?;
    let applied_map = read_applied(&connection)?;
    let applied = applied_map.keys().cloned().collect::<HashSet<_>>();
    let migrations = discover_migrations(dir)?;
    validate_applied_migrations(&migrations, &applied_map)?;
    let pending = ordered_pending(&migrations, &applied)?
        .into_iter()
        .map(|migration| {
            let sql = migration_sql(&migration)?;
            Ok(PlannedMigration {
                id: migration.id.clone(),
                depends_on: migration.depends_on.clone(),
                sql,
                checksum: migration_checksum(&migration),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(DatabasePlan {
        config,
        database_path,
        applied_count: applied_map.len(),
        pending,
    })
}

pub(crate) fn apply_migrations(dir: &Path) -> Result<DatabasePlan, String> {
    let config = database_config(dir)?;
    let (mut connection, database_path) = open_connection(dir, &config, false)?;
    connection
        .execute_batch(&format!(
            "CREATE TABLE IF NOT EXISTS \"{MIGRATION_TABLE}\" (id TEXT PRIMARY KEY NOT NULL, applied_at TEXT NOT NULL, checksum TEXT NOT NULL);"
        ))
        .map_err(|error| format!("SQLite migration ledger initialization failed: {error}"))?;
    let applied_map = read_applied(&connection)?;
    let applied = applied_map.keys().cloned().collect::<HashSet<_>>();
    let migrations = discover_migrations(dir)?;
    validate_applied_migrations(&migrations, &applied_map)?;
    let pending = ordered_pending(&migrations, &applied)?;
    let transaction = connection
        .transaction()
        .map_err(|error| format!("SQLite migration transaction failed: {error}"))?;
    for migration in &pending {
        for sql in migration_sql(migration)? {
            transaction
                .execute_batch(&sql)
                .map_err(|error| format!("migration `{}` failed: {error}", migration.id))?;
        }
        let applied_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| format!("migration clock failed: {error}"))?
            .as_secs()
            .to_string();
        transaction
            .execute(
                &format!("INSERT INTO \"{MIGRATION_TABLE}\" (id, applied_at, checksum) VALUES (?1, ?2, ?3)"),
                params![migration.id, applied_at, migration_checksum(migration)],
            )
            .map_err(|error| format!("migration `{}` ledger write failed: {error}", migration.id))?;
    }
    transaction
        .commit()
        .map_err(|error| format!("SQLite migration commit failed: {error}"))?;
    Ok(DatabasePlan {
        config,
        database_path,
        applied_count: applied_map.len() + pending.len(),
        pending: Vec::new(),
    })
}

pub(crate) fn plan_to_json(plan: &DatabasePlan) -> JsonValue {
    json!({
        "driver": plan.config.driver,
        "url": plan.config.url,
        "database": plan.database_path,
        "applied": plan.applied_count,
        "pending": plan.pending.iter().map(|migration| json!({
            "id": migration.id,
            "depends_on": migration.depends_on,
            "checksum": migration.checksum,
            "sql": migration.sql,
        })).collect::<Vec<_>>(),
    })
}

pub(crate) fn plan_to_text(plan: &DatabasePlan) -> String {
    if plan.pending.is_empty() {
        return format!(
            "SQLite migration plan is up to date: {} applied migration(s) at {}",
            plan.applied_count, plan.database_path
        );
    }
    let mut output = format!(
        "SQLite migration plan: {} pending migration(s), {} applied, database={}\n",
        plan.pending.len(),
        plan.applied_count,
        plan.database_path
    );
    for migration in &plan.pending {
        output.push_str(&format!("- {}", migration.id));
        if !migration.depends_on.is_empty() {
            output.push_str(&format!(" depends_on={}", migration.depends_on.join(",")));
        }
        output.push('\n');
        for sql in &migration.sql {
            output.push_str("  ");
            output.push_str(sql);
            output.push('\n');
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::OptionalExtension;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_COUNTER: AtomicU64 = AtomicU64::new(1);

    struct TempProject {
        path: PathBuf,
    }

    impl TempProject {
        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempProject {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn write_project(files: &[(&str, &str)]) -> TempProject {
        let path = env::temp_dir().join(format!(
            "zap-db-test-{}-{}",
            std::process::id(),
            TEST_COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("temporary project should be created");
        let directory = path;
        fs::write(
            directory.join("zap.toml"),
            "[package]\nname = \"db-test\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[web]\nroutes = \"routes.zp\"\nmodels = \"models\"\nmiddleware = \"middleware.zp\"\nmigrations = \"migrations\"\nadmin = \"admin.zp\"\nserver = \"server.zp\"\nserialization = \"json-by-default\"\n[database]\ndriver = \"sqlite\"\nurl = \"data/test.sqlite3\"\n",
        )
        .expect("manifest should be written");
        fs::create_dir_all(directory.join("migrations")).expect("migration directory");
        for (path, content) in files {
            let target = directory.join(path);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent).expect("fixture parent should exist");
            }
            fs::write(target, content).expect("fixture should be written");
        }
        TempProject { path: directory }
    }

    #[test]
    fn parses_literal_migration_and_generates_deterministic_sql() {
        let project = write_project(&[(
            "migrations/0001_initial.zp",
            "export fn migration():\n    return {\"id\": \"0001_initial\", \"depends_on\": [], \"operations\": [{\"kind\": \"create_table\", \"table\": \"users\", \"columns\": {\"email\": \"text not null unique\", \"id\": \"integer primary key\"}}]}\n",
        )]);
        let plan = database_plan(project.path(), true).expect("migration plan should validate");
        assert_eq!(plan.pending.len(), 1);
        assert_eq!(
            plan.pending[0].sql[0],
            "CREATE TABLE \"users\" (\"email\" TEXT NOT NULL UNIQUE, \"id\" INTEGER PRIMARY KEY);"
        );
    }

    #[test]
    fn rejects_non_declarative_migration_source() {
        let project = write_project(&[(
            "migrations/0001_bad.zp",
            "export fn migration():\n    return {\"id\": env_get(\"BAD\", \"x\"), \"depends_on\": [], \"operations\": []}\n",
        )]);
        let error = database_plan(project.path(), true).expect_err("calls must be rejected");
        assert!(error.contains("only literal maps and lists"));
    }

    #[test]
    fn applies_migrations_atomically_and_is_idempotent() {
        let project = write_project(&[(
            "migrations/0001_initial.zp",
            "export fn migration():\n    return {\"id\": \"0001_initial\", \"depends_on\": [], \"operations\": [{\"kind\": \"create_table\", \"table\": \"users\", \"columns\": {\"id\": \"integer primary key\"}}]}\n",
        )]);
        let first = apply_migrations(project.path()).expect("first migration should apply");
        assert_eq!(first.applied_count, 1);
        let second =
            apply_migrations(project.path()).expect("second migration should be idempotent");
        assert_eq!(second.applied_count, 1);
        let connection = Connection::open(project.path().join("data/test.sqlite3"))
            .expect("database should open");
        let table: Option<String> = connection
            .query_row(
                "SELECT name FROM sqlite_master WHERE type = 'table' AND name = 'users'",
                [],
                |row| row.get(0),
            )
            .optional()
            .expect("table query should succeed");
        assert_eq!(table.as_deref(), Some("users"));
    }

    #[test]
    fn detects_dependency_cycles() {
        let project = write_project(&[
            (
                "migrations/0001_a.zp",
                "export fn migration():\n    return {\"id\": \"0001_a\", \"depends_on\": [\"0002_b\"], \"operations\": [{\"kind\": \"create_table\", \"table\": \"a\", \"columns\": {\"id\": \"integer primary key\"}}]}\n",
            ),
            (
                "migrations/0002_b.zp",
                "export fn migration():\n    return {\"id\": \"0002_b\", \"depends_on\": [\"0001_a\"], \"operations\": [{\"kind\": \"create_table\", \"table\": \"b\", \"columns\": {\"id\": \"integer primary key\"}}]}\n",
            ),
        ]);
        let error = database_plan(project.path(), true).expect_err("cycle must be rejected");
        assert!(error.contains("circular migration dependency"));
    }

    #[test]
    fn sqlite_adapter_maps_rows_and_controls_transactions() {
        let project = write_project(&[(
            "migrations/0001_initial.zp",
            "export fn migration():\n    return {\"id\": \"0001_initial\", \"depends_on\": [], \"operations\": [{\"kind\": \"create_table\", \"table\": \"users\", \"columns\": {\"id\": \"integer primary key\", \"name\": \"text not null\", \"email\": \"text not null unique\"}}]}\n",
        )]);
        apply_migrations(project.path()).expect("schema should apply");
        let mut adapter = SqliteDatabaseAdapter::open(project.path()).expect("adapter should open");
        assert_eq!(adapter.driver(), "sqlite");
        assert!(
            adapter.database_path().ends_with("data/test.sqlite3"),
            "unexpected database path: {}",
            adapter.database_path()
        );

        let inserted = adapter
            .insert_user("  Ada  ", " ADA@EXAMPLE.COM ")
            .expect("user should insert");
        assert_eq!(inserted.name, "Ada");
        assert_eq!(inserted.email, "ada@example.com");
        assert_eq!(
            adapter.find_user(inserted.id).unwrap(),
            Some(inserted.clone())
        );
        assert_eq!(adapter.find_user(99_999).unwrap(), None);
        assert!(adapter.insert_user("", "bad").is_err());
        assert!(adapter
            .insert_user("Other", "ADA@example.com")
            .expect_err("duplicate email must fail")
            .contains("user insert failed"));

        let rolled_back_id = {
            let mut transaction = adapter.begin().expect("transaction should begin");
            let record = transaction
                .insert_user("Rollback", "rollback@example.com")
                .expect("transaction insert should succeed");
            let id = record.id;
            transaction.rollback().expect("rollback should succeed");
            id
        };
        assert_eq!(adapter.find_user(rolled_back_id).unwrap(), None);
    }

    #[test]
    fn rejects_changed_applied_migrations() {
        let project = write_project(&[(
            "migrations/0001_initial.zp",
            "export fn migration():\n    return {\"id\": \"0001_initial\", \"depends_on\": [], \"operations\": [{\"kind\": \"create_table\", \"table\": \"users\", \"columns\": {\"id\": \"integer primary key\"}}]}\n",
        )]);
        apply_migrations(project.path()).expect("migration should apply");
        fs::write(
            project.path().join("migrations/0001_initial.zp"),
            "export fn migration():\n    return {\"id\": \"0001_initial\", \"depends_on\": [], \"operations\": [{\"kind\": \"create_table\", \"table\": \"users\", \"columns\": {\"id\": \"integer primary key\", \"name\": \"text\"}}]}\n",
        )
        .expect("migration should be edited");
        let error = database_plan(project.path(), true).expect_err("drift must be rejected");
        assert!(error.contains("changed after it was applied"));
    }

    #[test]
    fn rejects_unsafe_identifiers_and_definitions() {
        let project = write_project(&[(
            "migrations/0001_bad.zp",
            "export fn migration():\n    return {\"id\": \"0001_bad\", \"depends_on\": [], \"operations\": [{\"kind\": \"create_table\", \"table\": \"users;drop\", \"columns\": {\"id\": \"integer primary key\"}}]}\n",
        )]);
        let error =
            database_plan(project.path(), true).expect_err("unsafe identifier must be rejected");
        assert!(error.contains("safe SQL identifier"));
    }
}
