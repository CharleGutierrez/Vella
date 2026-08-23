use vella::auth::{AuthService, OAuthService};

#[tokio::main]
#[test]
async fn test_oauth_flow_and_magic_links() {
    let db_file = format!("/tmp/vella_test_oauth_{}.db", rand::random::<u64>());
    let db = vella::db::SqliteDatabase::connect(&format!("sqlite://{}?mode=rwc", db_file), 5).await.unwrap();

    vella::db::SchemaMigrator::migrate_system_tables(&db.pool).await.unwrap();
    let auth_svc = AuthService::new(db.pool.clone());
    let oauth_svc = OAuthService::new(db.pool.clone());

    // 1. Check OAuth URL Generation
    let google_url = OAuthService::get_google_auth_url("my-google-client-id", "http://localhost:8080/callback");
    assert!(google_url.contains("accounts.google.com"));
    assert!(google_url.contains("client_id=my-google-client-id"));

    let github_url = OAuthService::get_github_auth_url("my-github-client-id", "http://localhost:8080/callback");
    assert!(github_url.contains("github.com/login/oauth"));

    // 2. OAuth Callback Login & Auto User Provisioning
    let session = oauth_svc
        .handle_oauth_login(
            &auth_svc,
            "google",
            "google_sub_12345",
            "developer@company.com",
            "developer",
            None,
            None,
        )
        .await
        .unwrap();

    assert_eq!(session.username, "developer");
    assert!(!session.token.is_empty());

    // Validate created session
    let validated = auth_svc.validate_session(&session.token).await.unwrap();
    assert!(validated.is_some());
    let user = validated.unwrap();
    assert_eq!(user.email, "developer@company.com");
    assert_eq!(user.oauth_provider.as_deref(), Some("google"));

    // 3. Magic Link Flow
    let magic_token = oauth_svc.request_magic_link("magic_user@company.com").await.unwrap();
    assert!(!magic_token.is_empty());

    // Verify token
    let magic_session = oauth_svc
        .verify_magic_link(&auth_svc, &magic_token, None, None)
        .await
        .unwrap();

    assert!(magic_session.is_some());
    let s = magic_session.unwrap();
    assert_eq!(s.username, "magic_user");

    // Second verification should fail (already used)
    let re_verify = oauth_svc
        .verify_magic_link(&auth_svc, &magic_token, None, None)
        .await
        .unwrap();
    assert!(re_verify.is_none());
}
