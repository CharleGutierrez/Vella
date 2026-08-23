use serde::{Deserialize, Serialize};

/// Role-Based Access Control (RBAC) user roles in Vella
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Manager,
    Editor,
    Viewer,
}

impl Role {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "admin" => Role::Admin,
            "manager" => Role::Manager,
            "editor" => Role::Editor,
            _ => Role::Viewer,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Admin => "Admin",
            Role::Manager => "Manager",
            Role::Editor => "Editor",
            Role::Viewer => "Viewer",
        }
    }

    pub fn can_read(&self) -> bool {
        true
    }

    pub fn can_create(&self) -> bool {
        matches!(self, Role::Admin | Role::Manager | Role::Editor)
    }

    pub fn can_update(&self) -> bool {
        matches!(self, Role::Admin | Role::Manager | Role::Editor)
    }

    pub fn can_delete(&self) -> bool {
        matches!(self, Role::Admin | Role::Manager)
    }

    pub fn can_approve(&self) -> bool {
        matches!(self, Role::Admin | Role::Manager)
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, Role::Admin)
    }
}

/// An authenticated user in Vella
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub role: Role,
    pub is_active: bool,
    pub oauth_provider: Option<String>,
}

/// An active session record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: i64,
    pub token: String,
    pub user_id: i64,
    pub username: String,
    pub role: Role,
    pub expires_at: String,
}
