use crate::core::error::VellaError;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Row, Sqlite};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub id: i64,
    pub model_name: String,
    pub record_id: i64,
    pub field_name: String,
    pub old_value: Option<String>,
    pub new_value: String,
    pub status: String,
    pub requested_by_username: Option<String>,
    pub created_at: String,
}

pub struct ApprovalService {
    pool: Pool<Sqlite>,
}

impl ApprovalService {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    /// Submit a field change for manager/admin approval
    pub async fn create_approval(
        &self,
        model_name: &str,
        record_id: i64,
        field_name: &str,
        old_val: Option<&str>,
        new_val: &str,
        user_id: Option<i64>,
        username: Option<&str>,
    ) -> Result<i64, VellaError> {
        let id = sqlx::query(
            r#"
            INSERT INTO _vella_approvals (
                model_name, record_id, field_name, old_value, new_value,
                status, requested_by_id, requested_by_username
            ) VALUES (?, ?, ?, ?, ?, 'Pending', ?, ?)
            "#
        )
        .bind(model_name)
        .bind(record_id)
        .bind(field_name)
        .bind(old_val)
        .bind(new_val)
        .bind(user_id)
        .bind(username)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(id)
    }

    /// List all pending approval requests
    pub async fn list_pending(&self) -> Result<Vec<ApprovalRecord>, VellaError> {
        let rows = sqlx::query(
            r#"
            SELECT id, model_name, record_id, field_name, old_value, new_value, status,
                   requested_by_username, created_at
            FROM _vella_approvals
            WHERE status = 'Pending'
            ORDER BY id DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut list = Vec::new();
        for r in rows {
            list.push(ApprovalRecord {
                id: r.try_get("id")?,
                model_name: r.try_get("model_name")?,
                record_id: r.try_get("record_id")?,
                field_name: r.try_get("field_name")?,
                old_value: r.try_get("old_value")?,
                new_value: r.try_get("new_value")?,
                status: r.try_get("status")?,
                requested_by_username: r.try_get("requested_by_username")?,
                created_at: r.try_get("created_at")?,
            });
        }
        Ok(list)
    }

    /// Approve a pending change and apply it directly to the target table
    pub async fn approve(
        &self,
        approval_id: i64,
        reviewer_id: i64,
        reviewer_username: &str,
        table_name: &str,
    ) -> Result<bool, VellaError> {
        let row_opt = sqlx::query(
            "SELECT model_name, record_id, field_name, new_value, status FROM _vella_approvals WHERE id = ? LIMIT 1"
        )
        .bind(approval_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row_opt {
            let status: String = row.try_get("status")?;
            if status != "Pending" {
                return Ok(false);
            }

            let record_id: i64 = row.try_get("record_id")?;
            let field_name: String = row.try_get("field_name")?;
            let new_value: String = row.try_get("new_value")?;

            let update_sql = format!("UPDATE \"{}\" SET \"{}\" = ? WHERE id = ?", table_name, field_name);
            sqlx::query(&update_sql)
                .bind(&new_value)
                .bind(record_id)
                .execute(&self.pool)
                .await?;

            sqlx::query(
                r#"
                UPDATE _vella_approvals
                SET status = 'Approved', reviewed_by_id = ?, reviewed_by_username = ?, updated_at = datetime('now')
                WHERE id = ?
                "#
            )
            .bind(reviewer_id)
            .bind(reviewer_username)
            .bind(approval_id)
            .execute(&self.pool)
            .await?;

            return Ok(true);
        }

        Ok(false)
    }

    /// Reject a pending change request
    pub async fn reject(
        &self,
        approval_id: i64,
        reviewer_id: i64,
        reviewer_username: &str,
    ) -> Result<bool, VellaError> {
        let res = sqlx::query(
            r#"
            UPDATE _vella_approvals
            SET status = 'Rejected', reviewed_by_id = ?, reviewed_by_username = ?, updated_at = datetime('now')
            WHERE id = ? AND status = 'Pending'
            "#
        )
        .bind(reviewer_id)
        .bind(reviewer_username)
        .bind(approval_id)
        .execute(&self.pool)
        .await?;

        Ok(res.rows_affected() > 0)
    }
}
