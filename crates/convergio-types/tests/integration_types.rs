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
