# convergio-sdk

[![CI](https://github.com/Roberdan/convergio-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/Roberdan/convergio-sdk/actions/workflows/ci.yml)
[![License: Convergio Community](https://img.shields.io/badge/license-Convergio%20Community-blue)](https://github.com/Roberdan/convergio-sdk/blob/main/LICENSE)

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

<!-- Copyright (c) 2026 Roberto D'Angelo. CC-BY-4.0. -->

## The Agentic Manifesto

*Human purpose. AI momentum.*
Milano — 23 June 2025

**What we believe**
1. **Intent is human, momentum is agent.**
2. **Impact must reach every mind and body.**
3. **Trust grows from transparent provenance.**
4. **Progress is judged by outcomes, not output.**

**How we act**
1. **Humans stay accountable for decisions and effects.**
2. **Agents amplify capability, never identity.**
3. **We design from the edge first: disability, language, connectivity.**
4. **Safety rails precede scale.**
5. **Learn in small loops, ship value early.**
6. **Bias is a bug—we detect, test, and fix continuously.**

*Signed in Milano, 23 June 2025 — Roberto D'Angelo · Claude · ChatGPT*

*Made with ❤️ for Mario in Milano, Italy, Europe.*
