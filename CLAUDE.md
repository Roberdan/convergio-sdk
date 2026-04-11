# CLAUDE.md — convergio-sdk

Read `AGENTS.md` first. This file adds Claude Code-specific behavior.

Conversation: **Italian**. Code + docs: **English**.
Co-Authored-By: your model name (e.g. `Claude Opus 4.6`)
PRs: auto-merged when CI green. Branch auto-deleted.

## Crate layout

```
crates/
├── convergio-types/      — Extension, Manifest, DomainEvent, ApiError
├── convergio-telemetry/  — tracing, health registry, metrics
├── convergio-db/         — r2d2 SQLite pool, migration runner
└── convergio-security/   — JWT, AEAD, RBAC, audit, trust, SSRF
```

Deps: telemetry → types, db → types, security → types + db

## Workflow

1. Read AGENTS.md for build/test/rules
2. Work in worktree: `git worktree add .worktrees/fix-name -b fix/name`
3. Commit conventional, push, create PR with 5 sections
4. Never merge — auto-merge handles it after CI green

## SDK boundary

SDK = what EVERY crate needs. If only some use it → domain crate, own repo.
Not SDK: IPC, mesh, inference, knowledge.
