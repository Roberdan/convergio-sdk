//! Integration tests for convergio-security.
//! Tests crypto, auth, and trust as an external consumer.

use convergio_security::aead;
use convergio_security::jwt;
use convergio_security::rbac;
use convergio_security::sandbox;
use convergio_security::ssrf;
use convergio_security::trust::{self, TrustLevel};
use std::net::IpAddr;
use std::sync::Arc;

// Helper: create an in-memory DB connection with all security tables.
fn setup_conn() -> rusqlite::Connection {
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    conn.execute_batch(convergio_security::schema::ALL_TABLES)
        .unwrap();
    conn
}

// Helper: create an in-memory pool with all security tables applied.
fn setup_pool() -> convergio_db::pool::ConnPool {
    let pool = convergio_db::pool::create_memory_pool().unwrap();
    {
        let conn = pool.get().unwrap();
        conn.execute_batch(convergio_security::schema::ALL_TABLES)
            .unwrap();
    }
    pool
}

#[test]
fn aead_encrypt_decrypt_roundtrip() {
    aead::init_master_key(None);
    let org = "test-org";
    let plaintext = "sensitive data";

    let encrypted = aead::encrypt(org, plaintext).expect("encrypt should work");
    assert_ne!(encrypted, plaintext);

    let decrypted = aead::decrypt(org, &encrypted).expect("decrypt should work");
    assert_eq!(decrypted, plaintext);
}

#[test]
fn jwt_issue_and_validate_roundtrip() {
    jwt::init_jwt_secret(None);

    let token = jwt::issue_token("agent-007", jwt::AgentRole::Executor, vec![], 3600)
        .expect("issue should work");
    assert!(!token.is_empty());

    let claims = jwt::validate_token(&token).expect("validate should work");
    assert_eq!(claims.sub, "agent-007");
}

#[test]
fn ssrf_blocks_private_ips() {
    assert!(ssrf::is_private_ip("127.0.0.1".parse::<IpAddr>().unwrap()));
    assert!(ssrf::is_private_ip("10.0.0.1".parse::<IpAddr>().unwrap()));
    assert!(ssrf::is_private_ip(
        "192.168.1.1".parse::<IpAddr>().unwrap()
    ));
    assert!(!ssrf::is_private_ip("8.8.8.8".parse::<IpAddr>().unwrap()));
}

#[test]
fn ssrf_validates_urls() {
    assert!(ssrf::validate_outbound_url("https://example.com/api").is_ok());
    assert!(ssrf::validate_outbound_url("").is_err());
}

#[test]
fn trust_level_ordering() {
    assert_eq!(TrustLevel::from_i64(0), TrustLevel::Untrusted);
    assert_eq!(TrustLevel::from_i64(1), TrustLevel::Basic);
    assert_eq!(TrustLevel::from_i64(2), TrustLevel::Standard);
}

// --- trust.rs coverage ---

#[test]
fn trust_level_from_i64_all_variants() {
    assert_eq!(TrustLevel::from_i64(0), TrustLevel::Untrusted);
    assert_eq!(TrustLevel::from_i64(1), TrustLevel::Basic);
    assert_eq!(TrustLevel::from_i64(2), TrustLevel::Standard);
    assert_eq!(TrustLevel::from_i64(3), TrustLevel::Elevated);
    assert_eq!(TrustLevel::from_i64(4), TrustLevel::Owner);
    // Unknown values fall back to Untrusted
    assert_eq!(TrustLevel::from_i64(99), TrustLevel::Untrusted);
    assert_eq!(TrustLevel::from_i64(-1), TrustLevel::Untrusted);
}

#[test]
fn trust_set_and_get() {
    let conn = setup_conn();
    trust::set_trust(&conn, "peer-alpha", TrustLevel::Elevated, "admin", "verified").unwrap();
    assert_eq!(trust::get_trust(&conn, "peer-alpha"), TrustLevel::Elevated);
}

#[test]
fn trust_get_unknown_peer_returns_untrusted() {
    let conn = setup_conn();
    assert_eq!(trust::get_trust(&conn, "ghost-peer"), TrustLevel::Untrusted);
}

#[test]
fn trust_set_overwrites_existing_level() {
    let conn = setup_conn();
    trust::set_trust(&conn, "peer-b", TrustLevel::Basic, "admin", "initial").unwrap();
    trust::set_trust(&conn, "peer-b", TrustLevel::Owner, "admin", "promoted").unwrap();
    assert_eq!(trust::get_trust(&conn, "peer-b"), TrustLevel::Owner);
}

#[test]
fn trust_list_empty_then_populated() {
    let conn = setup_conn();
    let empty = trust::list_trust(&conn);
    assert!(empty.is_empty());

    trust::set_trust(&conn, "node-1", TrustLevel::Standard, "ops", "ok").unwrap();
    trust::set_trust(&conn, "node-2", TrustLevel::Basic, "ops", "new").unwrap();
    let records = trust::list_trust(&conn);
    assert_eq!(records.len(), 2);
    // Results are ordered by peer_name
    assert_eq!(records[0].peer_name, "node-1");
    assert_eq!(records[1].peer_name, "node-2");
}

#[test]
fn trust_list_record_fields() {
    let conn = setup_conn();
    trust::set_trust(&conn, "verified-node", TrustLevel::Standard, "root", "prod").unwrap();
    let records = trust::list_trust(&conn);
    assert_eq!(records.len(), 1);
    let r = &records[0];
    assert_eq!(r.peer_name, "verified-node");
    assert_eq!(r.trust_level, TrustLevel::Standard);
    assert_eq!(r.granted_by, "root");
    assert_eq!(r.reason, "prod");
    assert!(!r.granted_at.is_empty());
    assert!(r.expires_at.is_none());
}

#[test]
fn secret_filter_register_and_list() {
    let conn = setup_conn();
    let empty = trust::list_secret_filters(&conn);
    assert!(empty.is_empty());

    trust::register_secret_filter(&conn, "PROD_SECRET", TrustLevel::Elevated, "production API key")
        .unwrap();
    trust::register_secret_filter(&conn, "LOG_LEVEL", TrustLevel::Untrusted, "safe to share")
        .unwrap();

    let filters = trust::list_secret_filters(&conn);
    assert_eq!(filters.len(), 2);
    // Ordered by env_var alphabetically: LOG_LEVEL before PROD_SECRET
    assert_eq!(filters[0].env_var, "LOG_LEVEL");
    assert_eq!(filters[0].min_trust_level, TrustLevel::Untrusted);
    assert_eq!(filters[1].env_var, "PROD_SECRET");
    assert_eq!(filters[1].min_trust_level, TrustLevel::Elevated);
}

#[test]
fn secret_filter_overwrite() {
    let conn = setup_conn();
    trust::register_secret_filter(&conn, "DB_URL", TrustLevel::Standard, "database").unwrap();
    // Overwrite with higher requirement
    trust::register_secret_filter(&conn, "DB_URL", TrustLevel::Owner, "prod db — owner only")
        .unwrap();
    let filters = trust::list_secret_filters(&conn);
    assert_eq!(filters.len(), 1);
    assert_eq!(filters[0].min_trust_level, TrustLevel::Owner);
    assert_eq!(filters[0].description, "prod db — owner only");
}

#[test]
fn can_access_secret_based_on_trust() {
    let conn = setup_conn();
    trust::set_trust(&conn, "low-peer", TrustLevel::Basic, "admin", "low").unwrap();
    trust::register_secret_filter(
        &conn,
        "PRIVILEGED_KEY",
        TrustLevel::Elevated,
        "elevated only",
    )
    .unwrap();
    trust::register_secret_filter(&conn, "PUBLIC_VAR", TrustLevel::Untrusted, "public").unwrap();

    // Basic peer cannot access Elevated-required secret
    assert!(!trust::can_access_secret(&conn, "low-peer", "PRIVILEGED_KEY"));
    // Basic peer can access Untrusted-required var
    assert!(trust::can_access_secret(&conn, "low-peer", "PUBLIC_VAR"));
    // Unknown var defaults to Standard requirement — Basic cannot access
    assert!(!trust::can_access_secret(&conn, "low-peer", "UNKNOWN_VAR"));
}

#[test]
fn filter_env_for_peer_filters_correctly() {
    let conn = setup_conn();
    trust::set_trust(&conn, "standard-peer", TrustLevel::Standard, "admin", "ok").unwrap();
    trust::register_secret_filter(&conn, "OWNER_SECRET", TrustLevel::Owner, "owner only").unwrap();
    trust::register_secret_filter(&conn, "SAFE_VAR", TrustLevel::Untrusted, "safe").unwrap();

    let env = vec![
        ("OWNER_SECRET".to_string(), "top-secret".to_string()),
        ("SAFE_VAR".to_string(), "visible".to_string()),
        // UNKNOWN_VAR defaults to Standard — Standard peer can access
        ("UNKNOWN_VAR".to_string(), "maybe".to_string()),
    ];

    let filtered = trust::filter_env_for_peer(&conn, "standard-peer", &env);
    // Standard peer: can access SAFE_VAR and UNKNOWN_VAR (defaults to Standard), not OWNER_SECRET
    assert_eq!(filtered.len(), 2);
    let keys: Vec<&str> = filtered.iter().map(|(k, _)| k.as_str()).collect();
    assert!(keys.contains(&"SAFE_VAR"));
    assert!(keys.contains(&"UNKNOWN_VAR"));
    assert!(!keys.contains(&"OWNER_SECRET"));
}

// --- sandbox.rs coverage ---

#[test]
fn sandbox_for_trust_untrusted_restrictions() {
    let policy = sandbox::sandbox_for_trust("rogue", TrustLevel::Untrusted);
    assert_eq!(policy.peer_name, "rogue");
    assert!(!policy.allow_network);
    assert!(!policy.allow_disk_write);
    assert_eq!(policy.max_cpu_seconds, 60);
    assert_eq!(policy.max_memory_mb, 512);
    assert!(policy.allowed_commands.is_empty());
    assert!(!policy.blocked_paths.is_empty());
}

#[test]
fn sandbox_for_trust_basic_allows_disk_write() {
    let policy = sandbox::sandbox_for_trust("basic-peer", TrustLevel::Basic);
    assert!(!policy.allow_network);
    assert!(policy.allow_disk_write);
    assert_eq!(policy.max_cpu_seconds, 300);
    assert_eq!(policy.max_memory_mb, 1024);
    assert!(!policy.allowed_commands.is_empty());
}

#[test]
fn sandbox_for_trust_standard_allows_network() {
    let policy = sandbox::sandbox_for_trust("std-peer", TrustLevel::Standard);
    assert!(policy.allow_network);
    assert!(policy.allow_disk_write);
    assert_eq!(policy.max_cpu_seconds, 3600);
    assert_eq!(policy.max_memory_mb, 4096);
    assert!(policy.blocked_paths.is_empty());
}

#[test]
fn sandbox_for_trust_elevated_unrestricted() {
    let policy = sandbox::sandbox_for_trust("elevated-peer", TrustLevel::Elevated);
    assert!(policy.allow_network);
    assert!(policy.allow_disk_write);
    assert_eq!(policy.max_cpu_seconds, 0);
    assert_eq!(policy.max_memory_mb, 0);
}

#[test]
fn sandbox_for_trust_owner_unrestricted() {
    let policy = sandbox::sandbox_for_trust("owner-peer", TrustLevel::Owner);
    assert!(policy.allow_network);
    assert!(policy.allow_disk_write);
    assert_eq!(policy.max_cpu_seconds, 0);
    assert_eq!(policy.max_memory_mb, 0);
}

#[test]
fn sandbox_resolve_uses_custom_override_when_present() {
    let conn = setup_conn();
    let custom = sandbox::SandboxPolicy {
        peer_name: "special-peer".into(),
        allow_network: false,
        allow_disk_write: false,
        max_cpu_seconds: 10,
        max_memory_mb: 128,
        allowed_commands: vec!["echo".into()],
        blocked_paths: vec!["/tmp".into()],
    };
    sandbox::set_custom_sandbox(&conn, &custom).unwrap();

    // Even though trust is Owner (normally unrestricted), the override applies
    let resolved = sandbox::resolve_sandbox(&conn, "special-peer", TrustLevel::Owner);
    assert_eq!(resolved.max_cpu_seconds, 10);
    assert!(!resolved.allow_network);
}

#[test]
fn sandbox_resolve_falls_back_to_trust_default() {
    let conn = setup_conn();
    // No custom override for this peer
    let resolved = sandbox::resolve_sandbox(&conn, "no-override-peer", TrustLevel::Standard);
    assert!(resolved.allow_network);
    assert_eq!(resolved.max_cpu_seconds, 3600);
}

#[test]
fn sandbox_get_custom_returns_none_if_absent() {
    let conn = setup_conn();
    assert!(sandbox::get_custom_sandbox(&conn, "absent-peer").is_none());
}

// --- rbac.rs coverage ---

#[test]
fn rbac_coordinator_accesses_anything() {
    assert!(rbac::role_can_access(&jwt::AgentRole::Coordinator, "/api/security/trust"));
    assert!(rbac::role_can_access(&jwt::AgentRole::Coordinator, "/api/admin/secret"));
}

#[test]
fn rbac_executor_allowed_routes() {
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/plan-db/task/42"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/plan-db/agent/1"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/plan-db/checkpoint/5"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/plan-db/context/x"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/plan-db/json/foo"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/plan-db/list"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/plan-db/execution-tree/1"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/plan-db/readiness/1"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/plan-db/kb"
    ));
    assert!(rbac::role_can_access(&jwt::AgentRole::Executor, "/api/build"));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/build/foo"
    ));
    assert!(rbac::role_can_access(&jwt::AgentRole::Executor, "/api/test"));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/test/foo"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/health"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/tracking/x"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/workspace/x"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/memory/x"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/ipc/x"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/delegate/x"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/delegation/x"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/notify"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/notify/foo"
    ));
}

#[test]
fn rbac_executor_denied_routes() {
    assert!(!rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/security/trust"
    ));
    assert!(!rbac::role_can_access(
        &jwt::AgentRole::Executor,
        "/api/admin"
    ));
}

#[test]
fn rbac_kernel_allowed_routes() {
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Kernel,
        "/api/kernel/run"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Kernel,
        "/api/notify"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Kernel,
        "/api/notify/foo"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Kernel,
        "/api/health"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Kernel,
        "/api/health/deep"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Kernel,
        "/api/node/info"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Kernel,
        "/api/heartbeat"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Kernel,
        "/api/voice/x"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Kernel,
        "/api/memory/x"
    ));
}

#[test]
fn rbac_kernel_denied_routes() {
    assert!(!rbac::role_can_access(
        &jwt::AgentRole::Kernel,
        "/api/plan-db/task/1"
    ));
    assert!(!rbac::role_can_access(
        &jwt::AgentRole::Kernel,
        "/api/build"
    ));
}

#[test]
fn rbac_worker_allowed_routes() {
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Worker,
        "/api/plan-db/task/1"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Worker,
        "/api/plan-db/list"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Worker,
        "/api/plan-db/context/x"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Worker,
        "/api/delegate/status"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Worker,
        "/api/delegation/x"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Worker,
        "/api/health"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Worker,
        "/api/health/deep"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Worker,
        "/api/heartbeat"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Worker,
        "/api/ipc/call"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Worker,
        "/api/tracking/x"
    ));
}

#[test]
fn rbac_worker_denied_routes() {
    assert!(!rbac::role_can_access(
        &jwt::AgentRole::Worker,
        "/api/plan-db/agent/1"
    ));
    assert!(!rbac::role_can_access(
        &jwt::AgentRole::Worker,
        "/api/build"
    ));
    assert!(!rbac::role_can_access(
        &jwt::AgentRole::Worker,
        "/api/admin"
    ));
}

#[test]
fn rbac_dashboard_allowed_routes() {
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/api/health"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/api/health/deep"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/api/overview"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/api/agents"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/api/plans"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/api/plan-db/tasks"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/api/mesh/peers"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/api/tasks/1"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/api/notifications"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/api/projects"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/api/events"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/api/peers"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/api/ipc/x"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/api/kernel/x"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/api/node/x"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/api/memory"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/api/audit/x"
    ));
    assert!(rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/ws/live"
    ));
}

#[test]
fn rbac_dashboard_denied_routes() {
    assert!(!rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/api/build"
    ));
    assert!(!rbac::role_can_access(
        &jwt::AgentRole::Dashboard,
        "/api/security/trust"
    ));
}

// --- trust_routes.rs coverage (via HTTP handlers through tower) ---

use axum::body::Body;
use axum::http::{Request, StatusCode};
use convergio_security::trust_routes::SecurityState;
use tower::ServiceExt;

fn make_security_app() -> axum::Router {
    let pool = setup_pool();
    let state = Arc::new(SecurityState { pool });
    convergio_security::trust_routes::security_routes(state)
}

#[tokio::test]
async fn route_set_trust_returns_ok() {
    let app = make_security_app();
    let body = serde_json::json!({
        "peer": "test-node",
        "level": 2,
        "granted_by": "admin",
        "reason": "integration test"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/security/trust")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["status"], "ok");
    assert_eq!(json["peer"], "test-node");
    assert_eq!(json["trust_level"], 2);
}

#[tokio::test]
async fn route_get_trust_unknown_peer_returns_untrusted() {
    let app = make_security_app();
    let req = Request::builder()
        .method("GET")
        .uri("/api/security/trust/unknown-peer")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["peer"], "unknown-peer");
    assert_eq!(json["trust_level"], 0);
}

#[tokio::test]
async fn route_list_trust_empty() {
    let app = make_security_app();
    let req = Request::builder()
        .method("GET")
        .uri("/api/security/trust")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert!(json.as_array().unwrap().is_empty());
}

#[tokio::test]
async fn route_set_and_list_trust() {
    let pool = setup_pool();
    let state = Arc::new(SecurityState { pool });
    let app = convergio_security::trust_routes::security_routes(state);

    // Set trust for two peers
    for (peer, level) in [("alpha", 1i64), ("beta", 3i64)] {
        let body = serde_json::json!({
            "peer": peer,
            "level": level,
            "granted_by": "tester",
            "reason": "test"
        });
        let req = Request::builder()
            .method("POST")
            .uri("/api/security/trust")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&body).unwrap()))
            .unwrap();
        let resp = app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    // List trust
    let req = Request::builder()
        .method("GET")
        .uri("/api/security/trust")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let records: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(records.as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn route_register_and_list_filters() {
    let pool = setup_pool();
    let state = Arc::new(SecurityState { pool });
    let app = convergio_security::trust_routes::security_routes(state);

    let body = serde_json::json!({
        "env_var": "MY_SECRET",
        "min_trust_level": 3,
        "description": "sensitive key"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/security/secrets/filter")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["status"], "ok");
    assert_eq!(json["env_var"], "MY_SECRET");

    // List filters
    let req = Request::builder()
        .method("GET")
        .uri("/api/security/secrets/filter")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let filters: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(filters.as_array().unwrap().len(), 1);
    assert_eq!(filters[0]["env_var"], "MY_SECRET");
}

#[tokio::test]
async fn route_get_sandbox_untrusted_peer() {
    let app = make_security_app();
    let req = Request::builder()
        .method("GET")
        .uri("/api/security/sandbox/new-peer")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let policy: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(policy["peer_name"], "new-peer");
    assert_eq!(policy["allow_network"], false);
    assert_eq!(policy["max_cpu_seconds"], 60);
}

#[tokio::test]
async fn route_set_and_get_sandbox_override() {
    let pool = setup_pool();
    let state = Arc::new(SecurityState { pool });
    let app = convergio_security::trust_routes::security_routes(state);

    let body = serde_json::json!({
        "peer_name": "override-peer",
        "allow_network": true,
        "allow_disk_write": false,
        "max_cpu_seconds": 30,
        "max_memory_mb": 256,
        "allowed_commands": ["echo"],
        "blocked_paths": ["/tmp"]
    });

    // POST to set sandbox override
    let req = Request::builder()
        .method("POST")
        .uri("/api/security/sandbox/override-peer")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["status"], "ok");
    assert_eq!(json["peer"], "override-peer");

    // GET should now return the override
    let req = Request::builder()
        .method("GET")
        .uri("/api/security/sandbox/override-peer")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let policy: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(policy["max_cpu_seconds"], 30);
    assert_eq!(policy["allow_network"], true);
    assert_eq!(policy["allow_disk_write"], false);
}
