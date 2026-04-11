# ADR-001: SDK extraction from monorepo

Status: accepted
Date: 2026-04-11

Context: Monorepo (38 crates, 103K LOC) causes agents to burn tokens, make errors, and CI to compile everything on every PR. Agents need isolated, small repos.

Decision:
- Extract shared types, telemetry, security, db into convergio-sdk
- SDK = what EVERY crate needs. If only some use it, it's a domain crate.
- Domain crates will be extracted into separate repos, each importing SDK via git tag
- Daemon becomes a thin compositor importing all crates

Consequences:
- Agent context drops from ~103K LOC to 1-5K LOC per task
- CI per crate: ~15-20s instead of ~1m14s
- Breaking SDK changes require coordinated update across all downstream repos
- Dependabot handles SDK version propagation automatically
