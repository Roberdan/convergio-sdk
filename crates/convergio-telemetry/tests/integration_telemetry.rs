//! Integration tests for convergio-telemetry.
//! Tests health registry and metrics as an external consumer.

use convergio_telemetry::health::{ComponentHealth, HealthCheck, HealthRegistry};
use convergio_telemetry::metrics::MetricsCollector;
use convergio_types::extension::Health;
use std::sync::Arc;

struct AlwaysHealthy;
impl HealthCheck for AlwaysHealthy {
    fn name(&self) -> &str {
        "test-healthy"
    }
    fn check(&self) -> ComponentHealth {
        ComponentHealth {
            name: "test-healthy".into(),
            status: Health::Ok,
            message: None,
        }
    }
}

struct AlwaysDegraded;
impl HealthCheck for AlwaysDegraded {
    fn name(&self) -> &str {
        "test-degraded"
    }
    fn check(&self) -> ComponentHealth {
        ComponentHealth {
            name: "test-degraded".into(),
            status: Health::Degraded {
                reason: "slow".into(),
            },
            message: Some("running slow".into()),
        }
    }
}

#[test]
fn health_registry_all_healthy() {
    let registry = HealthRegistry::new();
    registry.register(Arc::new(AlwaysHealthy));

    let results = registry.check_all();
    assert_eq!(results.len(), 1);
    assert!(matches!(results[0].status, Health::Ok));
}

#[test]
fn health_registry_aggregate_degraded() {
    let registry = HealthRegistry::new();
    registry.register(Arc::new(AlwaysHealthy));
    registry.register(Arc::new(AlwaysDegraded));

    let overall = registry.aggregate_status();
    assert!(matches!(overall, Health::Degraded { .. }));
}

#[test]
fn metrics_collector_empty() {
    let collector = MetricsCollector::new();
    let metrics = collector.collect_all();
    assert!(metrics.is_empty());
}
