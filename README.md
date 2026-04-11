# convergio-sdk

[![CI](https://github.com/Roberdan/convergio-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/Roberdan/convergio-sdk/actions/workflows/ci.yml)
[![License: Convergio Community](https://img.shields.io/badge/license-Convergio%20Community-blue)](https://github.com/Roberdan/convergio-sdk/blob/main/LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/Roberdan/convergio-sdk/badge)](https://scorecard.dev/viewer/?uri=github.com/Roberdan/convergio-sdk)

Core SDK for the [Convergio](https://github.com/Roberdan/convergio) ecosystem — shared types, traits, telemetry, security, and database primitives.

Every Convergio crate depends on this SDK. It provides the foundation that all
domain crates build on: the `Extension` trait, type contracts, observability,
authentication, encryption, and database pooling.

## Crates

| Crate | Description | LOC | Tests |
|-------|-------------|-----|-------|
| [`convergio-types`](crates/convergio-types/) | Extension trait, Manifest, DomainEvent, ApiError, Config | ~1200 | 9 |
| [`convergio-telemetry`](crates/convergio-telemetry/) | Tracing, metrics collection, health aggregation | ~370 | 6 |
| [`convergio-db`](crates/convergio-db/) | Database pool (r2d2 + SQLite), migration runner, schema registry | ~480 | 12 |
| [`convergio-security`](crates/convergio-security/) | JWT, AEAD encryption, RBAC, audit chain, trust, sandbox, SSRF | ~1550 | 29 |

**Total: ~3600 LOC, 56 tests**

## Usage

```toml
[dependencies]
convergio-types = { git = "https://github.com/Roberdan/convergio-sdk", tag = "v0.2.0" }
convergio-telemetry = { git = "https://github.com/Roberdan/convergio-sdk", tag = "v0.2.0" }
convergio-db = { git = "https://github.com/Roberdan/convergio-sdk", tag = "v0.2.0" }
convergio-security = { git = "https://github.com/Roberdan/convergio-sdk", tag = "v0.2.0" }
```

## Development

```bash
cargo fmt --all -- --check
RUSTFLAGS="-Dwarnings" cargo clippy --workspace --all-targets --locked
cargo test --workspace --locked
```

## Part of Convergio

This SDK is the foundation of the [Convergio](https://github.com/Roberdan/convergio) ecosystem.
All domain crates (orchestrator, billing, mesh, voice, etc.) depend on this SDK
to share common types, traits, and contracts.

## License

Convergio Community License v1.3 — see [LICENSE](LICENSE).

---

## Agentic Manifesto

See the full [Agentic Manifesto](https://github.com/Roberdan/convergio/blob/main/AgenticManifesto.md) — the guiding philosophy behind Convergio.

---

© 2025-present Roberto D'Angelo. All rights reserved.
