pub mod types;
pub mod validator;

pub use types::*;
pub use validator::BridgeValidator;

use crate::{Result, auth::types::SessionValidationResult, client::FlowfullClient};

#[derive(Clone)]
pub struct BridgeClient {
    client: FlowfullClient,
}

impl BridgeClient {
    pub(crate) fn new(client: FlowfullClient) -> Self {
        Self { client }
    }

    pub async fn validate_session(
        &self,
        session_id: impl Into<String>,
    ) -> Result<SessionValidationResult> {
        self.client.auth().bridge_validate(session_id).await
    }
}
