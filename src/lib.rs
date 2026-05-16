//! Async Rust client for Flowfull and Flowless-compatible backends.

pub mod auth;
pub mod bridge;
pub mod client;
pub mod config;
pub mod error;
pub mod middleware;
pub mod operators;
#[cfg(feature = "payments")]
pub mod payments;
pub mod query;
pub mod request;
pub mod response;
pub mod session;
pub mod storage;
pub mod upload;

pub use auth::AuthClient;
pub use bridge::{BridgeClient, BridgeValidator, SessionData, ValidationMode, ValidationOptions};
pub use client::FlowfullClient;
pub use config::{ClientConfig, ClientConfigBuilder, RetryConfig};
pub use error::{FlowfullError, Result};
pub use operators::*;
#[cfg(feature = "payments")]
pub use payments::PaymentsClient;
pub use query::{QueryBuilder, SortDirection};
pub use request::RequestOptions;
pub use response::{ApiResponse, PaginationMeta, RawResponse};
pub use session::{PUBFLOW_SESSION_ID, PUBFLOW_USER_DATA, SessionManager};
pub use storage::{FileStorage, MemoryStorage, Storage};
pub use upload::{UploadBuilder, UploadFile};
