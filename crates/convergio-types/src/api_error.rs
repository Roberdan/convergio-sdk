//! Standardized API error type with proper HTTP status mapping.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

/// Unified error type for all daemon HTTP endpoints.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "error_kind")]
pub enum ApiError {
    /// 400 — malformed request, missing fields, invalid input.
    BadRequest { message: String },
    /// 404 — resource not found.
    NotFound { message: String },
    /// 401 — missing or invalid credentials.
    Unauthorized,
    /// 500 — unexpected internal failure.
    InternalError { message: String },
    /// 422 — a gate (evidence, test, PR, Thor) blocked the operation.
    GateBlocked { gate: String, reason: String },
}

impl ApiError {
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest {
            message: msg.into(),
        }
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound {
            message: msg.into(),
        }
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::InternalError {
            message: msg.into(),
        }
    }

    pub fn gate_blocked(gate: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::GateBlocked {
            gate: gate.into(),
            reason: reason.into(),
        }
    }

    /// HTTP status code for this error variant.
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest { .. } => StatusCode::BAD_REQUEST,
            Self::NotFound { .. } => StatusCode::NOT_FOUND,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Self::GateBlocked { .. } => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadRequest { message } => write!(f, "bad request: {message}"),
            Self::NotFound { message } => write!(f, "not found: {message}"),
            Self::Unauthorized => write!(f, "unauthorized"),
            Self::InternalError { message } => write!(f, "internal error: {message}"),
            Self::GateBlocked { gate, reason } => {
                write!(f, "gate blocked: {gate} — {reason}")
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = serde_json::json!({
            "error": self.to_string(),
            "status": status.as_u16(),
        });
        (status, axum::Json(body)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bad_request_status() {
        let err = ApiError::bad_request("missing field");
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn not_found_status() {
        let err = ApiError::not_found("plan 99");
        assert_eq!(err.status_code(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn unauthorized_status() {
        assert_eq!(
            ApiError::Unauthorized.status_code(),
            StatusCode::UNAUTHORIZED
        );
    }

    #[test]
    fn internal_error_status() {
        let err = ApiError::internal("db crash");
        assert_eq!(err.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn gate_blocked_status() {
        let err = ApiError::gate_blocked("EvidenceGate", "no test_pass");
        assert_eq!(err.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[test]
    fn display_formats_correctly() {
        let err = ApiError::gate_blocked("Thor", "not validated");
        let s = err.to_string();
        assert!(s.contains("Thor"));
        assert!(s.contains("not validated"));
    }

    #[test]
    fn serializes_to_json() {
        let err = ApiError::bad_request("oops");
        let json = serde_json::to_value(&err).expect("serialize");
        assert_eq!(json["error_kind"], "BadRequest");
        assert_eq!(json["message"], "oops");
    }
}
