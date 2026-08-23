use crate::auth::crypto::Crypto;
use crate::auth::rbac::{AuthUser, Role, Session};
use crate::core::error::VellaError;
use sqlx::{Pool, Row, Sqlite};
use tracing::info;

pub struct AuthService {
    pool: Pool<Sqlite>,
}

impl AuthService {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    /// Ensure default superadmin account exists
    pub async fn ensure_admin_user(&self) -> Result<(), VellaError> {
        let count_row = sqlx::query("SELECT COUNT(*) as count FROM _vella_users")
            .fetch_one(&self.pool)
            .await?;
        let count: i64 = count_row.try_get("count")?;

        if count == 0 {
            let pass_hash = Crypto::hash_password("admin");
            sqlx::query(
                r#"
                INSERT INTO _vella_users (username, email, password_hash, role, is_active)
                VALUES ('admin', 'admin@vella.dev', ?, 'Admin', 1)
                "#
            )
            .bind(pass_hash)
            .execute(&self.pool)
            .await?;

            info!("✨ [Vella] Auto-generated Superadmin => Username: 'admin', Password: 'admin'");
        }
        Ok(())
    }

    /// Create session for a user ID
    pub async fn create_session_for_user(
        &self,
        user_id: i64,
        username: &str,
        role: Role,
        ip: Option<String>,
        ua: Option<String>,
    ) -> Result<Session, VellaError> {
        let token = Crypto::random_token(32);
        let expires_at = (chrono::Utc::now() + chrono::Duration::days(7)).to_rfc3339();

        let session_id = sqlx::query(
            r#"
            INSERT INTO _vella_sessions (token, user_id, ip_address, user_agent, expires_at)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(&token)
        .bind(user_id)
        .bind(ip)
        .bind(ua)
        .bind(&expires_at)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        let _ = sqlx::query("UPDATE _vella_users SET last_login = datetime('now') WHERE id = ?")
            .bind(user_id)
            .execute(&self.pool)
            .await;

        Ok(Session {
            id: session_id,
            token,
            user_id,
            username: username.to_string(),
            role,
            expires_at,
        })
    }

    /// Authenticate a user and create a 7-day session
    pub async fn login(
        &self,
        username_or_email: &str,
        password: &str,
        ip: Option<String>,
        ua: Option<String>,
    ) -> Result<Option<Session>, VellaError> {
        let query_str = username_or_email.to_lowercase();
        let row_opt = sqlx::query(
            "SELECT id, username, email, password_hash, role, is_active FROM _vella_users WHERE username = ? OR email = ? LIMIT 1"
        )
        .bind(&query_str)
        .bind(&query_str)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row_opt {
            let pass_hash: String = row.try_get("password_hash")?;
            let is_active: i64 = row.try_get("is_active")?;

            if is_active == 0 || !Crypto::verify_password(password, &pass_hash) {
                return Ok(None);
            }

            let user_id: i64 = row.try_get("id")?;
            let username: String = row.try_get("username")?;
            let role_str: String = row.try_get("role")?;
            let role = Role::from_str(&role_str);

            let session = self.create_session_for_user(user_id, &username, role, ip, ua).await?;
            return Ok(Some(session));
        }

        Ok(None)
    }

    /// Validate session token from Cookie or Bearer header with expiration enforcement
    pub async fn validate_session(&self, token: &str) -> Result<Option<AuthUser>, VellaError> {
        let row_opt = sqlx::query(
            r#"
            SELECT u.id, u.username, u.email, u.role, u.is_active, u.oauth_provider, s.expires_at
            FROM _vella_sessions s
            JOIN _vella_users u ON s.user_id = u.id
            WHERE s.token = ? AND u.is_active = 1
            LIMIT 1
            "#
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row_opt {
            let expires_at_str: String = row.try_get("expires_at")?;
            if let Ok(exp) = chrono::DateTime::parse_from_rfc3339(&expires_at_str) {
                if exp < chrono::Utc::now() {
                    // Session expired! Purge expired session
                    let _ = sqlx::query("DELETE FROM _vella_sessions WHERE token = ?")
                        .bind(token)
                        .execute(&self.pool)
                        .await;
                    return Ok(None);
                }
            }

            let role_str: String = row.try_get("role")?;
            let oauth_prov: Option<String> = row.try_get("oauth_provider").ok();
            return Ok(Some(AuthUser {
                id: row.try_get("id")?,
                username: row.try_get("username")?,
                email: row.try_get("email")?,
                role: Role::from_str(&role_str),
                is_active: true,
                oauth_provider: oauth_prov,
            }));
        }

        Ok(None)
    }

    /// Invalidate session
    pub async fn logout(&self, token: &str) -> Result<(), VellaError> {
        sqlx::query("DELETE FROM _vella_sessions WHERE token = ?")
            .bind(token)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
