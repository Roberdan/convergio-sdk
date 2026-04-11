# Convergio — Agent Rules

> Universal rules for any AI agent working on this repository.
> Tool-agnostic: applies to Claude Code, Cursor, Copilot, Windsurf, or any other AI tool.

## What is Convergio

Convergio: modular Rust daemon for autonomous AI organizations.
Every module implements the `Extension` trait from [convergio-sdk](https://github.com/Roberdan/convergio-sdk).

This crate is part of the Convergio ecosystem. It depends on the SDK for shared types,
telemetry, security, and database primitives.

## Code rules

- Code and docs in **English**
- Max 300 lines per file
- Conventional commits: `feat:`, `fix:`, `docs:`, `chore:`, `refactor:`
- Every extension owns its DB tables via `migrations()`
- Breaking changes require a version bump

## Before "done" checklist

1. `cargo fmt --all -- --check`
2. `RUSTFLAGS="-Dwarnings" cargo clippy --workspace --all-targets --locked`
3. `cargo test --workspace --locked`
4. Commit with conventional message
5. Push and create PR — do NOT merge without user approval

## Test rules (NON-NEGOTIABLE)

- Never hardcode system counts in tests (use `>=`, not `==`)
- Never hardcode versions (use `env!("CARGO_PKG_VERSION")`)
- Never use fixed baselines as CI gates
- Every public function should have at least one test

## Model routing

| Role | Model | When |
|------|-------|------|
| Architecture, security, planning, review | **Opus** | Design decisions, complex reasoning |
| Mechanical execution (CRUD, tests, file ops) | **Sonnet** | Clear instructions with precise file/function specs |
| Classification, routing | **Haiku** | Tool routing, simple validation |

## What NOT to do

- NEVER ship without tests
- NEVER declare done without evidence (commit hash, test result)
- NEVER bypass hooks, tests, or CI gates
- NEVER merge PRs without explicit user approval
- NEVER modify SDK types here — changes to shared types go in the SDK repo
- NEVER add dependencies without checking version alignment with the SDK

## SDK dependency

This crate depends on [convergio-sdk](https://github.com/Roberdan/convergio-sdk).
The SDK provides: types, telemetry, security, db.
Do NOT duplicate SDK functionality — import it.
