# ADR-003: Migrate convergio-telemetry to OpenTelemetry

**Status:** Accepted
**Date:** 2026-04-11
**Context:** convergio-telemetry uses custom traits (`MetricsCollector`, `MetricSource`, `HealthRegistry`) for metrics and health. This works but locks us into a proprietary API that no external tool understands.
**Decision:**
- Replace `MetricsCollector` / `MetricSource` with OpenTelemetry SDK (`opentelemetry`, `opentelemetry-rust`)
- Use `tracing` + `tracing-opentelemetry` for distributed traces
- Use `opentelemetry-prometheus` exporter for metrics (Prometheus-compatible)
- Keep `HealthRegistry` as a thin wrapper over OTel health checks
- Migration is additive: old trait methods get default impls that delegate to OTel, then deprecate over 2 minor versions
- Target: SDK v0.3.0

**Consequences:**
- Every crate gets Jaeger/Prometheus/Grafana/Datadog export for free
- Distributed tracing across crate boundaries works out of the box
- `tracing` is already a dependency -- minimal new deps
- Breaking change for crates implementing `MetricSource` directly (rare -- most use the default impl)
- Need to coordinate SDK bump with downstream crates
