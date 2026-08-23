use crate::db::database_type::DatabaseType;
use crate::model::field::{Field, FieldType};
use crate::model::schema::ModelSchema;

/// Generates database-specific SQL dialects for SQLite, PostgreSQL (with pgvector), and MySQL
pub struct SqlDialect;

impl SqlDialect {
    /// Quote an identifier (table name or column name)
    pub fn quote_identifier(db_type: DatabaseType, name: &str) -> String {
        match db_type {
            DatabaseType::MySql => format!("`{}`", name.replace('`', "``")),
            _ => format!("\"{}\"", name.replace('"', "\"\"")),
        }
    }

    /// Parameter placeholder ($1, $2 for Postgres, ? for SQLite/MySQL)
    pub fn placeholder(db_type: DatabaseType, param_index: usize) -> String {
        match db_type {
            DatabaseType::Postgres => format!("${}", param_index),
            _ => "?".to_string(),
        }
    }

    /// Default current timestamp expression
    pub fn now_expr(db_type: DatabaseType) -> &'static str {
        match db_type {
            DatabaseType::Sqlite => "datetime('now')",
            DatabaseType::Postgres | DatabaseType::MySql => "NOW()",
        }
    }

    /// Column SQL type mapping across database engines (including vector embeddings)
    pub fn column_sql_type(db_type: DatabaseType, field: &Field) -> String {
        if field.name == "id" {
            return match db_type {
                DatabaseType::Sqlite => "INTEGER PRIMARY KEY AUTOINCREMENT".to_string(),
                DatabaseType::Postgres => "BIGSERIAL PRIMARY KEY".to_string(),
                DatabaseType::MySql => "BIGINT AUTO_INCREMENT PRIMARY KEY".to_string(),
            };
        }

        match &field.field_type {
            FieldType::Integer | FieldType::ForeignKey { .. } => match db_type {
                DatabaseType::Sqlite => "INTEGER".to_string(),
                DatabaseType::Postgres => "BIGINT".to_string(),
                DatabaseType::MySql => "BIGINT".to_string(),
            },
            FieldType::Float | FieldType::Money { .. } | FieldType::ProgressBar { .. } => match db_type {
                DatabaseType::Sqlite => "REAL".to_string(),
                DatabaseType::Postgres => "DOUBLE PRECISION".to_string(),
                DatabaseType::MySql => "DOUBLE".to_string(),
            },
            FieldType::Boolean => match db_type {
                DatabaseType::Sqlite => "INTEGER".to_string(),
                DatabaseType::Postgres => "BOOLEAN".to_string(),
                DatabaseType::MySql => "TINYINT(1)".to_string(),
            },
            FieldType::DateTime => match db_type {
                DatabaseType::Sqlite => "TEXT".to_string(),
                DatabaseType::Postgres => "TIMESTAMPTZ".to_string(),
                DatabaseType::MySql => "DATETIME".to_string(),
            },
            FieldType::Json => match db_type {
                DatabaseType::Sqlite => "TEXT".to_string(),
                DatabaseType::Postgres => "JSONB".to_string(),
                DatabaseType::MySql => "JSON".to_string(),
            },
            FieldType::Vector { dimensions } => match db_type {
                DatabaseType::Postgres => format!("vector({})", dimensions),
                DatabaseType::MySql => "JSON".to_string(),
                DatabaseType::Sqlite => "TEXT".to_string(),
            },
            _ => match db_type {
                DatabaseType::Sqlite => "TEXT".to_string(),
                DatabaseType::Postgres => "TEXT".to_string(),
                DatabaseType::MySql => "TEXT".to_string(),
            },
        }
    }

    /// Generate CREATE TABLE DDL for a model schema
    pub fn create_table_ddl(db_type: DatabaseType, schema: &ModelSchema) -> String {
        let mut statements = Vec::new();

        if db_type == DatabaseType::Postgres && schema.has_vectors() {
            statements.push("CREATE EXTENSION IF NOT EXISTS vector;".to_string());
        }

        let mut col_defs = Vec::new();

        for field in &schema.fields {
            if field.name == "id" {
                col_defs.push(format!(
                    "{} {}",
                    Self::quote_identifier(db_type, "id"),
                    Self::column_sql_type(db_type, field)
                ));
                continue;
            }

            let type_str = Self::column_sql_type(db_type, field);
            let mut def = format!("{} {}", Self::quote_identifier(db_type, &field.name), type_str);

            if field.required {
                def.push_str(" NOT NULL");
            }
            if field.unique {
                def.push_str(" UNIQUE");
            }

            if let Some(ref default) = field.default_value {
                match default {
                    serde_json::Value::Bool(b) => {
                        let val_str = match db_type {
                            DatabaseType::Postgres => if *b { "TRUE" } else { "FALSE" },
                            _ => if *b { "1" } else { "0" },
                        };
                        def.push_str(&format!(" DEFAULT {}", val_str));
                    }
                    serde_json::Value::Number(n) => def.push_str(&format!(" DEFAULT {}", n)),
                    serde_json::Value::String(s) => def.push_str(&format!(" DEFAULT '{}'", s.replace('\'', "''"))),
                    _ => {}
                }
            } else if field.name == "created_at" || field.name == "updated_at" {
                def.push_str(&format!(" DEFAULT ({})", Self::now_expr(db_type)));
            }

            col_defs.push(def);
        }

        let table_ddl = format!(
            "CREATE TABLE IF NOT EXISTS {} (\n    {}\n);",
            Self::quote_identifier(db_type, &schema.table_name),
            col_defs.join(",\n    ")
        );
        statements.push(table_ddl);

        // Vector indexes for PostgreSQL (HNSW index)
        if db_type == DatabaseType::Postgres {
            for (v_field, _) in schema.vector_fields() {
                statements.push(format!(
                    "CREATE INDEX IF NOT EXISTS idx_{}_{}_hnsw ON {} USING hnsw (\"{}\" vector_cosine_ops);",
                    schema.table_name, v_field.name, Self::quote_identifier(db_type, &schema.table_name), v_field.name
                ));
            }
        }

        statements.join("\n\n")
    }
}
