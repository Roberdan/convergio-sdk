//! Integration tests for convergio-types.
//! Tests the public API as an external consumer would use it.

use convergio_types::api_error::ApiError;
use convergio_types::events::{make_event, EventContext, EventKind};
use convergio_types::extension::{Health, Metric};
use convergio_types::manifest::{Capability, Manifest, ModuleKind};

#[test]
fn manifest_roundtrip() {
    let m = Manifest {
        id: "test-ext".into(),
        description: "A test extension".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        kind: ModuleKind::Extension,
        provides: vec![Capability {
            name: "http-routes".into(),
            version: "1".into(),
            description: "serves routes".into(),
        }],
        requires: vec![],
        agent_tools: vec![],
        required_roles: vec![],
    };

    let json = serde_json::to_string(&m).unwrap();
    let parsed: Manifest = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.id, "test-ext");
    assert!(!parsed.provides.is_empty());
}

#[test]
fn api_error_to_json() {
    let err = ApiError::not_found("widget 42 not found");
    let json = serde_json::to_value(&err).unwrap();
    assert_eq!(json["error_kind"], "NotFound");
}

#[test]
fn domain_event_creation() {
    let event = make_event(
        "test-agent",
        EventKind::PlanCreated {
            plan_id: 1,
            name: "test".into(),
        },
        EventContext::default(),
    );
    assert_eq!(event.actor.name, "test-agent");

    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains("PlanCreated"));
}

#[test]
fn health_variants_exist() {
    let _ok = Health::Ok;
    let _degraded = Health::Degraded {
        reason: "slow".into(),
    };
    let _down = Health::Down {
        reason: "crashed".into(),
    };
}

#[test]
fn metric_serialization() {
    let m = Metric {
        name: "requests".into(),
        value: 42.0,
        labels: vec![("method".into(), "GET".into())],
    };
    let json = serde_json::to_string(&m).unwrap();
    assert!(json.contains("requests"));
}

// --- config.rs ---

#[test]
fn node_role_as_str_and_display() {
    use convergio_types::config::NodeRole;
    assert_eq!(NodeRole::All.as_str(), "all");
    assert_eq!(NodeRole::Orchestrator.as_str(), "orchestrator");
    assert_eq!(NodeRole::Kernel.as_str(), "kernel");
    assert_eq!(NodeRole::Voice.as_str(), "voice");
    assert_eq!(NodeRole::Worker.as_str(), "worker");
    assert_eq!(NodeRole::NightAgent.as_str(), "nightagent");
    assert_eq!(format!("{}", NodeRole::All), "all");
}

#[test]
fn config_defaults() {
    use convergio_types::config::*;
    let cfg = ConvergioConfig::default();
    assert_eq!(cfg.daemon.port, 8420);
    assert!(cfg.daemon.auto_update);
    assert_eq!(cfg.node.role, NodeRole::All);
    assert_eq!(cfg.inference.default_model, "claude-sonnet-4-6");
    assert_eq!(cfg.inference.fallback.max_attempts, 3);
    assert!(!cfg.inference.fallback.t1.is_empty());
    assert_eq!(cfg.mesh.transport, "lan");
    assert_eq!(cfg.mesh.discovery, "mdns");
    assert!(!cfg.night.night_mode);
    assert_eq!(cfg.night.night_hours, "23:00-07:00");
    assert_eq!(cfg.kernel.max_tokens, 2048);
    assert!(!cfg.telegram.enabled);
}

#[test]
fn config_deserialize_toml() {
    use convergio_types::config::*;
    let toml_str = r#"
[node]
name = "test-node"
role = "worker"

[daemon]
port = 9000
"#;
    let cfg: ConvergioConfig = toml::from_str(toml_str).unwrap();
    assert_eq!(cfg.node.name, "test-node");
    assert_eq!(cfg.node.role, NodeRole::Worker);
    assert_eq!(cfg.daemon.port, 9000);
}

#[test]
fn node_role_serde_roundtrip() {
    use convergio_types::config::NodeRole;
    let roles = [
        NodeRole::All,
        NodeRole::Orchestrator,
        NodeRole::Kernel,
        NodeRole::Voice,
        NodeRole::Worker,
        NodeRole::NightAgent,
    ];
    for role in roles {
        let json = serde_json::to_string(&role).unwrap();
        let parsed: NodeRole = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, role);
    }
}

#[test]
fn inference_fallback_config_default_and_eq() {
    use convergio_types::config::InferenceFallbackConfig;
    let a = InferenceFallbackConfig::default();
    let b = InferenceFallbackConfig::default();
    assert_eq!(a, b);
}

// --- extension.rs ---

#[test]
fn app_context_insert_and_get() {
    use convergio_types::extension::AppContext;
    let mut ctx = AppContext::new();
    ctx.insert::<String>("hello".to_string());
    ctx.insert::<u32>(42u32);
    assert_eq!(ctx.get::<String>(), Some(&"hello".to_string()));
    assert_eq!(ctx.get::<u32>(), Some(&42));
    assert_eq!(ctx.get::<bool>(), None);
}

#[test]
fn app_context_get_arc() {
    use convergio_types::extension::AppContext;
    let mut ctx = AppContext::new();
    ctx.insert::<String>("world".to_string());
    let arc = ctx.get_arc::<String>();
    assert!(arc.is_some());
    assert_eq!(*arc.unwrap(), "world");
}

#[test]
fn app_context_overwrite() {
    use convergio_types::extension::AppContext;
    let mut ctx = AppContext::new();
    ctx.insert::<u32>(1);
    ctx.insert::<u32>(2);
    assert_eq!(ctx.get::<u32>(), Some(&2));
}

#[test]
fn migration_and_scheduled_task_fields() {
    use convergio_types::extension::{Migration, ScheduledTask};
    let m = Migration {
        version: 1,
        description: "init",
        up: "CREATE TABLE t(id INT)",
    };
    assert_eq!(m.version, 1);
    let s = ScheduledTask {
        name: "cleanup",
        cron: "0 2 * * *",
    };
    assert_eq!(s.name, "cleanup");
}

#[test]
fn mcp_tool_def_serde() {
    use convergio_types::extension::McpToolDef;
    let def = McpToolDef {
        name: "get_plan".into(),
        description: "Get a plan".into(),
        method: "GET".into(),
        path: "/api/plan/:id".into(),
        input_schema: serde_json::json!({"type": "object"}),
        min_ring: "community".into(),
        path_params: vec!["id".into()],
    };
    let json = serde_json::to_string(&def).unwrap();
    let parsed: McpToolDef = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.name, "get_plan");
    assert_eq!(parsed.path_params, vec!["id"]);
}

// --- resilience.rs ---

#[test]
fn retry_config_defaults() {
    use convergio_types::resilience::{CircuitBreakerConfig, CircuitState, RetryConfig};
    let r = RetryConfig::default();
    assert_eq!(r.max_retries, 3);
    assert!(r.jitter);
    assert_eq!(r.backoff_factor, 2.0);

    let cb = CircuitBreakerConfig::default();
    assert_eq!(cb.failure_threshold, 5);
    assert_eq!(cb.success_threshold, 2);

    assert_eq!(CircuitState::Closed, CircuitState::Closed);
    assert_ne!(CircuitState::Open, CircuitState::Closed);
}

// --- message_error.rs ---

#[test]
fn message_error_from_impls() {
    use convergio_types::message_error::MessageError;
    let e1: MessageError = "hello".into();
    assert_eq!(e1.to_string(), "hello");

    let e2: MessageError = String::from("world").into();
    assert_eq!(e2.to_string(), "world");

    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
    let e3: MessageError = io_err.into();
    assert!(e3.to_string().contains("not found"));
}

// --- dev_auth.rs ---

#[test]
fn dev_auth_header_format() {
    use convergio_types::dev_auth::dev_auth_header;
    let header = dev_auth_header();
    assert!(header.starts_with("Bearer "));
}

// --- platform_paths.rs ---

#[test]
fn validate_path_components_rejects_traversal() {
    use convergio_types::platform_paths::validate_path_components;
    use std::path::Path;
    assert!(validate_path_components(Path::new("a/b/c")).is_ok());
    assert!(validate_path_components(Path::new("a/../b")).is_err());
    assert!(validate_path_components(Path::new(".hidden/file")).is_ok());
}

#[test]
fn sanitize_path_rejects_traversal() {
    use convergio_types::platform_paths::sanitize_path;
    // Create a temp dir for testing
    let tmp = std::env::temp_dir().join("convergio_test_sanitize");
    let _ = std::fs::create_dir_all(&tmp);
    let sub = tmp.join("sub");
    let _ = std::fs::create_dir_all(&sub);
    // Valid path within base
    assert!(sanitize_path(&sub, &tmp).is_ok());
    // Invalid: path outside base
    let outside = std::env::temp_dir();
    assert!(sanitize_path(&outside, &sub).is_err());
    let _ = std::fs::remove_dir_all(&tmp);
}

// --- platform_restart.rs ---

#[test]
fn restart_result_fields() {
    use convergio_types::platform_restart::RestartResult;
    let r = RestartResult {
        success: true,
        method: "test",
        message: "ok".into(),
    };
    assert!(r.success);
    let json = serde_json::to_string(&r).unwrap();
    assert!(json.contains("test"));
}

#[test]
fn service_config_path_returns_something() {
    use convergio_types::platform_restart::service_config_path;
    // On macOS and Linux this should return Some; on other platforms may be None
    let path = service_config_path();
    if let Some(p) = path {
        assert!(!p.to_string_lossy().is_empty());
    }
}

#[test]
fn stop_daemon_returns_result() {
    use convergio_types::platform_restart::stop_daemon;
    // Will likely fail (no daemon running) but should return a RestartResult, not panic
    let result = stop_daemon();
    assert!(!result.method.is_empty());
}

#[test]
fn restart_daemon_returns_result() {
    use convergio_types::platform_restart::restart_daemon;
    let result = restart_daemon();
    assert!(!result.method.is_empty());
}
