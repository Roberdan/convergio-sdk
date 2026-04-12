# ADR-004: Security Audit Fixes

**Status:** Accepted  
**Date:** 2025-07-25  
**Author:** Security Audit (automated)

## Context

The convergio-sdk (5149 LOC) is the foundation for all Convergio crates. A comprehensive security audit identified multiple vulnerabilities across SQL injection, SSRF, path traversal, authentication, authorization, and cryptographic integrity domains.

## Findings & Fixes

### CRITICAL

| # | Category | File | Issue | Fix |
|---|----------|------|-------|-----|
| 1 | SQL Injection | `convergio-db/helpers.rs` | `PRAGMA table_info('{table}')` uses string interpolation | Added `is_safe_identifier()` validation; errors propagated instead of swallowed via `filter_map` |
| 2 | SSRF Bypass | `convergio-security/ssrf.rs` | Naive URL parsing; bypassed with `@`, IPv6 brackets, non-HTTP schemes, localhost DNS | Replaced with `url::Url` parser; block non-HTTP schemes, credentials, localhost hostnames, bracketed IPv6 |
| 3 | Auth Bypass | `convergio-security/jwt.rs` | `get_secret()` returns empty `b""` if init not called — tokens signed with empty key are valid | Panic on uninitialized secret; `saturating_add` for TTL overflow |
| 4 | Integrity | `convergio-security/audit.rs` | `verify()` only checked prev_hash links, not recomputed entry hashes | Full hash recomputation on every entry during verification |
| 5 | AuthZ Bypass | `convergio-security/rbac.rs` | Path-prefix matching allowed `/api/buildx` to match `/api/build` | Added `path_prefix()` helper with segment-boundary enforcement |
| 6 | Path Traversal | `convergio-types/platform_paths.rs` | `project_output_dir()` passed unsanitized input to path join; `validate_path_components()` allowed absolute paths | `project_output_dir` now validates input; absolute paths rejected |

### HIGH

| # | Category | File | Issue | Fix |
|---|----------|------|-------|-----|
| 7 | Secret Exposure | `convergio-types/config.rs` | `TailscaleConfig.auth_key` and `TelegramConfig.token_keychain` derive `Debug` | Custom `Debug` impls that redact secrets |
| 8 | Permissive Fallback | `convergio-security/sandbox.rs` | Invalid JSON in DB falls back to empty allow/block lists | Added `tracing::warn` on corrupt data (preserves fallback behavior with observability) |
| 9 | DoS | `convergio-security/rate_limiter.rs` | Unbounded HashMap growth from unique IPs | Added `max_buckets` cap with auto-eviction of stale entries |

## Breaking Changes

- `project_output_dir()` now returns `Result<PathBuf, String>` instead of `PathBuf`
- `get_secret()` panics instead of returning empty bytes (fail-secure)
- `validate_path_components()` rejects absolute paths

## Decision

All fixes applied. The SDK must fail-secure: panicking on missing crypto initialization is preferable to silently accepting unsigned tokens.

## Dependencies Added

- `url = "2"` (for proper SSRF URL parsing)
