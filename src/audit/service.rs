use crate::core::error::VellaError;
use crate::model::ModelSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Pool, Row, Sqlite};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub id: i64,
    pub model_name: String,
    pub record_id: i64,
    pub action: String, // CREATE, UPDATE, DELETE, ROLLBACK_RESTORE, ROLLBACK_UPDATE
    pub username: Option<String>,
    pub changes: Value,
    pub snapshot: Value,
    pub created_at: String,
}

pub struct AuditService {
    pool: Pool<Sqlite>,
}

impl AuditService {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    /// Record a model mutation in the audit log
    pub async fn log_action(
        &self,
        model_name: &str,
        record_id: i64,
        action: &str,
        user_id: Option<i64>,
        username: Option<&str>,
        changes: &Value,
        snapshot: &Value,
        ip: Option<&str>,
    ) -> Result<i64, VellaError> {
        let changes_str = serde_json::to_string(changes)?;
        let snapshot_str = serde_json::to_string(snapshot)?;

        let id = sqlx::query(
            r#"
            INSERT INTO _vella_audit_logs (
                model_name, record_id, action, user_id, username,
                changes_json, snapshot_json, ip_address
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(model_name)
        .bind(record_id)
        .bind(action)
        .bind(user_id)
        .bind(username)
        .bind(changes_str)
        .bind(snapshot_str)
        .bind(ip)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(id)
    }

    /// Retrieve paginated audit logs
    pub async fn list_logs(&self, limit: i64, offset: i64) -> Result<Vec<AuditLogEntry>, VellaError> {
        let rows = sqlx::query(
            r#"
            SELECT id, model_name, record_id, action, username, changes_json, snapshot_json, created_at
            FROM _vella_audit_logs
            ORDER BY id DESC
            LIMIT ? OFFSET ?
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let mut list = Vec::new();
        for r in rows {
            let changes_str: String = r.try_get("changes_json")?;
            let snapshot_str: String = r.try_get("snapshot_json")?;
            list.push(AuditLogEntry {
                id: r.try_get("id")?,
                model_name: r.try_get("model_name")?,
                record_id: r.try_get("record_id")?,
                action: r.try_get("action")?,
                username: r.try_get("username")?,
                changes: serde_json::from_str(&changes_str).unwrap_or(Value::Null),
                snapshot: serde_json::from_str(&snapshot_str).unwrap_or(Value::Null),
                created_at: r.try_get("created_at")?,
            });
        }
        Ok(list)
    }

    /// Time-Travel Rollback: Restore a record to its snapshot state
    pub async fn rollback(
        &self,
        log_id: i64,
        schema: &ModelSchema,
        user_id: Option<i64>,
        username: Option<&str>,
    ) -> Result<bool, VellaError> {
        let row_opt = sqlx::query(
            "SELECT model_name, record_id, action, snapshot_json FROM _vella_audit_logs WHERE id = ? LIMIT 1"
        )
        .bind(log_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row_opt {
            let record_id: i64 = row.try_get("record_id")?;
            let action: String = row.try_get("action")?;
            let snapshot_str: String = row.try_get("snapshot_json")?;
            let snapshot: Value = serde_json::from_str(&snapshot_str)?;

            if let Value::Object(obj) = snapshot {
                if action == "DELETE" {
                    // Re-insert deleted record from snapshot
                    let mut cols = Vec::new();
                    let mut placeholders = Vec::new();
                    let mut vals = Vec::new();

                    for (k, v) in &obj {
                        cols.push(format!("\"{}\"", k));
                        placeholders.push("?");
                        vals.push(v.clone());
                    }

                    let sql = format!(
                        "INSERT INTO \"{}\" ({}) VALUES ({})",
                        schema.table_name,
                        cols.join(", "),
                        placeholders.join(", ")
                    );

                    let mut query = sqlx::query(&sql);
                    for val in vals {
                        query = crate::db::SqliteDatabase::bind_json_value(query, &val);
                    }
                    query.execute(&self.pool).await?;

                    self.log_action(
                        &schema.name,
                        record_id,
                        "ROLLBACK_RESTORE",
                        user_id,
                        username,
                        &serde_json::json!({ "reverted_log_id": log_id }),
                        &Value::Object(obj),
                        None,
                    ).await?;

                    info!("✨ [Vella Time-Travel] Restored deleted record #{} in '{}'", record_id, schema.name);
                    return Ok(true);
                } else if action == "UPDATE" {
                    // Revert modified fields back to snapshot values
                    let mut set_clauses = Vec::new();
                    let mut vals = Vec::new();

                    for (k, v) in &obj {
                        if k == "id" || k == "created_at" {
                            continue;
                        }
                        set_clauses.push(format!("\"{}\" = ?", k));
                        vals.push(v.clone());
                    }

                    if !set_clauses.is_empty() {
                        let sql = format!(
                            "UPDATE \"{}\" SET {} WHERE id = ?",
                            schema.table_name,
                            set_clauses.join(", ")
                        );

                        let mut query = sqlx::query(&sql);
                        for val in vals {
                            query = crate::db::SqliteDatabase::bind_json_value(query, &val);
                        }
                        query = query.bind(record_id);
                        query.execute(&self.pool).await?;

                        self.log_action(
                            &schema.name,
                            record_id,
                            "ROLLBACK_UPDATE",
                            user_id,
                            username,
                            &serde_json::json!({ "reverted_log_id": log_id }),
                            &Value::Object(obj),
                            None,
                        ).await?;

                        info!("✨ [Vella Time-Travel] Reverted record #{} in '{}'", record_id, schema.name);
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }
}
