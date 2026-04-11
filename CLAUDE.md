# CLAUDE.md — convergio-sdk

> Agent instructions for the Convergio SDK repository.

## What this is

The foundational SDK for all Convergio crates. Contains the shared contracts,
observability, security, and database primitives that every crate in the ecosystem uses.

## Structure

```
crates/
├── convergio-types/      — Extension trait, Manifest, DomainEvent, ApiError, Config (~1200 LOC)
├── convergio-telemetry/  — Tracing, metrics, health aggregation (~370 LOC)
├── convergio-db/         — r2d2 SQLite pool, migration runner, schema registry (~480 LOC)
└── convergio-security/   — JWT, AEAD, RBAC, audit, trust, sandbox, SSRF (~1550 LOC)
```

## Dependency graph

```
convergio-telemetry → convergio-types
convergio-db        → convergio-types
convergio-security  → convergio-types + convergio-db
```

## Rules

- Max 300 lines per file
- English only (code + docs). Conversation in Italian.
- Conventional commits: `feat:`, `fix:`, `docs:`, `chore:`, `refactor:`
- Every public type must have doc comments
- Breaking changes require a minor version bump (pre-1.0) or major bump (post-1.0)

## Build & Test

```bash
cargo fmt --all -- --check
RUSTFLAGS="-Dwarnings" cargo clippy --workspace --all-targets --locked
cargo test --workspace --locked
```

## This SDK is consumed by

All 36+ crates in the Convergio ecosystem via git dependency.
Changes here affect everything — be careful and deliberate.

## What belongs here vs. in a domain crate

SDK = what EVERY Convergio crate needs to function.
If only some crates use it, it's a domain crate with its own repo.

Examples of what does NOT belong here: IPC, mesh, inference, knowledge.
