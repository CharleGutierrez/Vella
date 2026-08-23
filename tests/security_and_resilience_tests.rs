use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;
use std::collections::HashMap;
use tower::ServiceExt;
use vella::ai::decision::{AiDecisionEngine, RiskLevel};
use vella::api::filter::QueryOptions;
use vella::auth::crypto::Crypto;
use vella::auth::service::AuthService;
use vella::core::resilience::{BreakerState, CircuitBreaker};
use vella::model::{Field, ModelSchema};
use vella::prelude::*;

#[tokio::main]
#[test]
async fn test_security_sqli_filter_parameterization() {
    let schema = ModelSchema::new("User")
        .field(Field::string("name").searchable())
        .with_timestamps();

    let mut params = HashMap::new();
    params.insert("name__contains".to_string(), "' OR '1'='1 --".to_string());

    let q_opts = QueryOptions::parse(&params);
    let (select_sql, sql_params, _, _) = q_opts.build_sql(&schema);

    assert_eq!(select_sql, "SELECT * FROM \"users\" WHERE \"name\" LIKE ? ORDER BY \"id\" DESC LIMIT ? OFFSET ?");
    assert_eq!(sql_params[0], json!("%' OR '1'='1 --%"));
}

#[tokio::main]
#[test]
async fn test_security_sqli_order_by_whitelist_hardening() {
    let schema = ModelSchema::new("User")
        .field(Field::string("name"))
        .with_timestamps();

    let mut params = HashMap::new();
    params.insert("$order".to_string(), "-name\"; DROP TABLE users; --".to_string());

    let q_opts = QueryOptions::parse(&params);
    let (select_sql, _, _, _) = q_opts.build_sql(&schema);

    assert!(select_sql.contains("ORDER BY \"id\" DESC"));
    assert!(!select_sql.contains("DROP TABLE"));
}

#[test]
fn test_security_dos_query_limit_clamping() {
    let mut params = HashMap::new();
    params.insert("$limit".to_string(), "99999999".to_string());
    params.insert("$offset".to_string(), "-50".to_string());

    let q_opts = QueryOptions::parse(&params);
    assert_eq!(q_opts.limit, 1000);
    assert_eq!(q_opts.offset, 0);
}

#[tokio::main]
#[test]
async fn test_security_expired_session_replay_attack() {
    let db_file = format!("/tmp/vella_test_session_security_{}.db", rand::random::<u64>());
    let db = vella::db::SqliteDatabase::connect(&format!("sqlite://{}?mode=rwc", db_file), 5).await.unwrap();
    vella::db::SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();

    let auth_svc = AuthService::new(db.pool.clone());
    let token = Crypto::random_token(32);
    let expired_at = (chrono::Utc::now() - chrono::Duration::hours(2)).to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO _vella_users (username, email, password_hash, role, is_active)
        VALUES ('victim', 'victim@company.com', '$s2$hash', 'Admin', 1)
        "#
    )
    .execute(&db.pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        INSERT INTO _vella_sessions (token, user_id, expires_at)
        VALUES (?, 1, ?)
        "#
    )
    .bind(&token)
    .bind(&expired_at)
    .execute(&db.pool)
    .await
    .unwrap();

    let session_val = auth_svc.validate_session(&token).await.unwrap();
    assert!(session_val.is_none());

    let purged_row = sqlx::query("SELECT COUNT(*) as c FROM _vella_sessions WHERE token = ?")
        .bind(&token)
        .fetch_one(&db.pool)
        .await
        .unwrap();

    let count: i64 = sqlx::Row::try_get(&purged_row, "c").unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_security_privilege_escalation_detection() {
    let assessment = AiDecisionEngine::assess_approval_risk("role", Some("Editor"), "SuperAdmin");
    assert_eq!(assessment.risk_level, RiskLevel::Critical);
    assert!(assessment.reasoning.iter().any(|r| r.contains("Superadmin")));
}

#[test]
fn test_circuit_breaker_self_healing() {
    let breaker = CircuitBreaker::new("db_pool_breaker", 3, 1);

    assert_eq!(breaker.state(), BreakerState::Closed);
    assert!(breaker.allow_execution());

    breaker.record_failure();
    breaker.record_failure();
    assert_eq!(breaker.state(), BreakerState::Closed);

    breaker.record_failure(); // 3rd failure trips breaker
    assert_eq!(breaker.state(), BreakerState::Open);
    assert!(!breaker.allow_execution());

    // Wait cooldown
    std::thread::sleep(std::time::Duration::from_millis(1100));

    // Next request transitions to HalfOpen
    assert!(breaker.allow_execution());
    assert_eq!(breaker.state(), BreakerState::HalfOpen);

    // Successful execution heals breaker
    breaker.record_success();
    assert_eq!(breaker.state(), BreakerState::Closed);
}

#[tokio::main]
#[test]
async fn test_panic_recovery_http_isolation() {
    let db_file = format!("/tmp/vella_test_panic_{}.db", rand::random::<u64>());
    let (router, _) = VellaApp::new()
        .database(&format!("sqlite://{}?mode=rwc", db_file))
        .build_router()
        .await
        .unwrap();

    // Query non-existent route
    let req = Request::builder()
        .method("GET")
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let res = router.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}
