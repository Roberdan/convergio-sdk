//! Development auth helper — reads token from environment with fallback.
//!
//! Used by daemon-internal HTTP calls (doctor checks, plan executor, etc.)
//! to avoid hard-coding auth tokens in source code.

/// Build the `Authorization` header value for internal daemon calls.
///
/// Reads `CONVERGIO_AUTH_TOKEN` from the environment; falls back to
/// `"dev-local"` when the variable is unset (local development only).
pub fn dev_auth_header() -> String {
    let token = std::env::var("CONVERGIO_AUTH_TOKEN").unwrap_or_else(|_| "dev-local".into());
    format!("Bearer {token}")
}
