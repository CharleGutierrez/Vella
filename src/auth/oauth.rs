use crate::auth::crypto::Crypto;
use crate::auth::rbac::{Role, Session};
use crate::auth::service::AuthService;
use crate::core::error::VellaError;
use sqlx::{Pool, Row, Sqlite};
use tracing::info;

pub struct OAuthService {
    pool: Pool<Sqlite>,
}

impl OAuthService {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    /// Generate Google OAuth Authorization URL
    pub fn get_google_auth_url(client_id: &str, redirect_uri: &str) -> String {
        let state = Crypto::random_token(16);
        format!(
            "https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id={}&redirect_uri={}&scope=openid%20email%20profile&state={}",
            client_id, urlencoding_simple(redirect_uri), state
        )
    }

    /// Generate GitHub OAuth Authorization URL
    pub fn get_github_auth_url(client_id: &str, redirect_uri: &str) -> String {
        let state = Crypto::random_token(16);
        format!(
            "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=user:email&state={}",
            client_id, urlencoding_simple(redirect_uri), state
        )
    }

    /// Handle Third-party OAuth profile resolution and auto-register/login user
    pub async fn handle_oauth_login(
        &self,
        auth_service: &AuthService,
        provider: &str,
        oauth_id: &str,
        email: &str,
        username: &str,
        ip: Option<String>,
        ua: Option<String>,
    ) -> Result<Session, VellaError> {
        let email_clean = email.to_lowercase();

        // Check if user exists by email or oauth provider
        let user_row_opt = sqlx::query(
            "SELECT id, username, role, is_active FROM _vella_users WHERE email = ? OR (oauth_provider = ? AND oauth_id = ?) LIMIT 1"
        )
        .bind(&email_clean)
        .bind(provider)
        .bind(oauth_id)
        .fetch_optional(&self.pool)
        .await?;

        let (user_id, final_username, role) = match user_row_opt {
            Some(row) => {
                let id: i64 = row.try_get("id")?;
                let uname: String = row.try_get("username")?;
                let role_str: String = row.try_get("role")?;
                // Update oauth linking if missing
                let _ = sqlx::query("UPDATE _vella_users SET oauth_provider = ?, oauth_id = ? WHERE id = ?")
                    .bind(provider)
                    .bind(oauth_id)
                    .bind(id)
                    .execute(&self.pool)
                    .await;
                (id, uname, Role::from_str(&role_str))
            }
            None => {
                // Auto-create user
                let pass_hash = Crypto::hash_password(&Crypto::random_token(16));
                let id = sqlx::query(
                    r#"
                    INSERT INTO _vella_users (username, email, password_hash, role, is_active, oauth_provider, oauth_id)
                    VALUES (?, ?, ?, 'Editor', 1, ?, ?)
                    "#
                )
                .bind(username)
                .bind(&email_clean)
                .bind(pass_hash)
                .bind(provider)
                .bind(oauth_id)
                .execute(&self.pool)
                .await?
                .last_insert_rowid();

                info!("✨ [Vella OAuth] Auto-created user '{}' via {} OAuth", username, provider);
                (id, username.to_string(), Role::Editor)
            }
        };

        auth_service.create_session_for_user(user_id, &final_username, role, ip, ua).await
    }

    /// Request a Magic Link login token
    pub async fn request_magic_link(&self, email: &str) -> Result<String, VellaError> {
        let token = Crypto::random_token(32);
        let expires_at = (chrono::Utc::now() + chrono::Duration::minutes(15)).to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO _vella_magic_links (email, token, expires_at)
            VALUES (?, ?, ?)
            "#
        )
        .bind(email.to_lowercase())
        .bind(&token)
        .bind(&expires_at)
        .execute(&self.pool)
        .await?;

        info!("📧 [Vella Magic Link] Generated login token for '{}'", email);
        Ok(token)
    }

    /// Verify a Magic Link login token and create a session
    pub async fn verify_magic_link(
        &self,
        auth_service: &AuthService,
        token: &str,
        ip: Option<String>,
        ua: Option<String>,
    ) -> Result<Option<Session>, VellaError> {
        let row_opt = sqlx::query(
            "SELECT id, email, expires_at FROM _vella_magic_links WHERE token = ? AND used = 0 LIMIT 1"
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row_opt {
            let magic_id: i64 = row.try_get("id")?;
            let email: String = row.try_get("email")?;
            let expires_at_str: String = row.try_get("expires_at")?;

            if let Ok(exp) = chrono::DateTime::parse_from_rfc3339(&expires_at_str) {
                if exp < chrono::Utc::now() {
                    return Ok(None);
                }
            }

            // Mark token as used
            let _ = sqlx::query("UPDATE _vella_magic_links SET used = 1 WHERE id = ?")
                .bind(magic_id)
                .execute(&self.pool)
                .await;

            // Find or create user
            let session = self.handle_oauth_login(
                auth_service,
                "magic_link",
                &email,
                &email,
                &email.split('@').next().unwrap_or("user"),
                ip,
                ua,
            ).await?;

            return Ok(Some(session));
        }

        Ok(None)
    }
}

fn urlencoding_simple(s: &str) -> String {
    s.replace(':', "%3A").replace('/', "%2F").replace('?', "%3F").replace('&', "%26").replace('=', "%3D")
}
