//! Optional web-framework middleware.
//!
//! Enable `middleware-axum` or `middleware-actix` to use framework-specific
//! helpers. The core client has no required web framework dependency.

#[cfg(feature = "middleware-actix")]
pub mod actix;

#[cfg(feature = "middleware-axum")]
pub mod axum;
