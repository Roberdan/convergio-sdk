# convergio-sdk

[![CI](https://github.com/Roberdan/convergio-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/Roberdan/convergio-sdk/actions/workflows/ci.yml)
[![License: Convergio Community](https://img.shields.io/badge/license-Convergio%20Community-blue)](https://github.com/Roberdan/convergio-sdk/blob/main/LICENSE)

Core SDK for the [Convergio](https://github.com/Roberdan/convergio) ecosystem — shared types, traits, and contracts.

Every Convergio crate depends on this SDK. It defines the `Extension` trait,
`Manifest`, `DomainEvent`, `ApiError`, configuration, and platform utilities.

## Crates

| Crate | Description | LOC |
|-------|-------------|-----|
| [`convergio-types`](crates/convergio-types/) | Extension trait, Manifest, DomainEvent, ApiError, Config | ~1200 |

## Usage

```toml
[dependencies]
convergio-types = { git = "https://github.com/Roberdan/convergio-sdk", tag = "v0.1.0" }
```

## Development

```bash
# Format check
cargo fmt --all -- --check

# Lint
RUSTFLAGS="-Dwarnings" cargo clippy --workspace

# Test
cargo test --workspace
```

## Part of Convergio

This SDK is the foundation of the [Convergio](https://github.com/Roberdan/convergio) ecosystem.
All domain crates (orchestrator, billing, mesh, voice, etc.) depend on this SDK
to share common types, traits, and contracts.

## License

Convergio Community License v1.3 — see [LICENSE](LICENSE).
