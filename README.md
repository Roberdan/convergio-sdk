# convergio-sdk

[![CI](https://github.com/Roberdan/convergio-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/Roberdan/convergio-sdk/actions/workflows/ci.yml)
[![Security](https://github.com/Roberdan/convergio-sdk/actions/workflows/ci.yml/badge.svg?event=push)](https://github.com/Roberdan/convergio-sdk/actions/workflows/ci.yml)
[![License: Convergio Community](https://img.shields.io/badge/license-Convergio%20Community-blue)](https://github.com/Roberdan/convergio-sdk/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange)](https://www.rust-lang.org/)
[![Zero Warnings](https://img.shields.io/badge/warnings-0-brightgreen)](#)
[![Adversarial Tests](https://img.shields.io/badge/adversarial_tests-17-brightgreen)](#)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/Roberdan/convergio-sdk/badge)](https://scorecard.dev/viewer/?uri=github.com/Roberdan/convergio-sdk)

Core SDK for the [Convergio](https://github.com/Roberdan/convergio) ecosystem.

## Architecture

```mermaid
graph TD
    D[convergio-daemon<br/>compositor] --> B[convergio-billing]
    D --> M[convergio-mesh]
    D --> V[convergio-voice]
    D --> O[convergio-orchestrator]
    D --> MORE[...15+ domain crates]

    B --> SDK
    M --> SDK
    V --> SDK
    O --> SDK
    MORE --> SDK

    subgraph SDK[convergio-sdk]
        T[convergio-types<br/>Extension, Manifest, DomainEvent, ApiError]
        TEL[convergio-telemetry<br/>tracing, health, metrics]
        DB[convergio-db<br/>r2d2 pool, migrations]
        SEC[convergio-security<br/>JWT, AEAD, RBAC, trust, SSRF]

        TEL --> T
        DB --> T
        SEC --> T
        SEC --> DB
    end

    style SDK fill:#1a1a2e,stroke:#e94560,color:#fff
    style T fill:#0f3460,stroke:#e94560,color:#fff
    style TEL fill:#0f3460,stroke:#e94560,color:#fff
    style DB fill:#0f3460,stroke:#e94560,color:#fff
    style SEC fill:#0f3460,stroke:#e94560,color:#fff
```

## Crates

| Crate | Description | LOC | Tests |
|-------|-------------|-----|-------|
| [`convergio-types`](crates/convergio-types/) | Extension trait, Manifest, DomainEvent, ApiError | ~1200 | 14 |
| [`convergio-telemetry`](crates/convergio-telemetry/) | Tracing, metrics, health aggregation | ~370 | 9 |
| [`convergio-db`](crates/convergio-db/) | r2d2 + SQLite pool, migration runner, schema registry | ~480 | 14 |
| [`convergio-security`](crates/convergio-security/) | JWT, AEAD, RBAC, audit, trust, sandbox, SSRF | ~1550 | 51 |

**Total: ~3600 LOC, 88 tests (unit + integration + adversarial)**

## Quality gates

| Gate | Enforced by | Status |
|------|------------|--------|
| Zero warnings | `RUSTFLAGS="-Dwarnings"` | CI blocks merge |
| All tests pass | `cargo test --locked` | CI blocks merge |
| Adversarial security tests | `tests/adversarial_*.rs` | CI blocks merge |
| Dependency audit (CVE) | `cargo audit` | CI blocks merge |
| License + dependency policy | `cargo deny check` | CI blocks merge |
| Format | `cargo fmt --check` | CI blocks merge |
| Coverage ≥80% | `cargo tarpaulin --fail-under 80` | CI blocks merge |
| Semver compatibility | `cargo semver-checks` | CI blocks merge (PR) |
| Unused dependencies | `cargo udeps` | CI blocks merge |
| Conventional commits | PR title lint | CI blocks merge (PR) |
| Mutation testing | `cargo mutants` | Informational |
| OpenSSF Scorecard | `ossf/scorecard-action` | Weekly scan |
| SBOM (CycloneDX) | `cargo cyclonedx` | On release |
| Auto-release | release-please + PAT | Fully automatic |

## Usage

```toml
[dependencies]
convergio-types = { git = "https://github.com/Roberdan/convergio-sdk", tag = "v0.1.4" }
convergio-telemetry = { git = "https://github.com/Roberdan/convergio-sdk", tag = "v0.1.4" }
convergio-db = { git = "https://github.com/Roberdan/convergio-sdk", tag = "v0.1.4" }
convergio-security = { git = "https://github.com/Roberdan/convergio-sdk", tag = "v0.1.4" }
```

## Development

```bash
cargo fmt --all -- --check
RUSTFLAGS="-Dwarnings" cargo clippy --workspace --all-targets --locked
cargo test --workspace --locked
cargo deny check
```

## License

Convergio Community License v1.3 — see [LICENSE](LICENSE).

---

## Agentic Manifesto

See the full [Agentic Manifesto](https://github.com/Roberdan/convergio/blob/main/AgenticManifesto.md) — the guiding philosophy behind Convergio.

---

© 2025-present Roberto D'Angelo. All rights reserved.
