//! convergio-types — The contract everything builds on.
//!
//! Defines the Extension trait, Manifest, DomainEvent, and shared types.
//! Every crate in the workspace depends on this.

pub mod api_error;
pub mod config;
pub mod dev_auth;
pub mod errors;
pub mod events;
pub mod extension;
pub mod manifest;
pub mod message_error;
pub mod platform_paths;
pub mod platform_restart;
pub mod resilience;

pub use api_error::ApiError;
pub use dev_auth::dev_auth_header;
