# ADR-002: Quality framework — zero tolerance

Status: accepted
Date: 2026-04-11

Context: SDK is the template for all repos. Defects propagate everywhere. Need standardized quality gates before extracting any crate.

Decision:
- Zero warnings (`-Dwarnings`), zero skipped tests, zero TODO without issue
- Max 250 lines per file
- Coverage minimum: 80% SDK, 70% domain crates
- Test levels: unit + integration (always), adversarial (if security), responsible AI (if LLM)
- cargo-deny for license/dependency policy
- ADR for every non-obvious architectural decision
- All docs agent-first: structured, token-efficient, actionable
- Comments: only `//!` module doc and `// WHY:` for non-obvious choices
- Engineering fundamentals from Microsoft ISE playbook

Consequences:
- Higher upfront effort per crate
- Slower initial extraction but much faster iteration once quality is baselined
- Every repo inherits the standard via template
