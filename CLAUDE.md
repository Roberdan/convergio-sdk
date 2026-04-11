# CLAUDE.md — convergio-sdk

> Agent instructions for the Convergio SDK repository.

## What this is

The foundational SDK for all Convergio crates. Contains the shared contract
(Extension trait, types, events) that every crate in the ecosystem implements.

## Structure

```
crates/
└── convergio-types/   — Extension trait, Manifest, DomainEvent, ApiError, Config
```

## Rules

- Max 300 lines per file
- English only
- Conventional commits
- Every public type must have doc comments
- Breaking changes require a minor version bump (pre-1.0) or major bump (post-1.0)

## Build & Test

```bash
cargo fmt --all -- --check
RUSTFLAGS="-Dwarnings" cargo clippy --workspace
cargo test --workspace
```

## This SDK is consumed by

All 36+ crates in the Convergio ecosystem via git dependency.
Changes here affect everything — be careful and deliberate.
