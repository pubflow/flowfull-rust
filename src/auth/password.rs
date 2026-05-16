use crate::{Result, auth::types::*, client::FlowfullClient};

#[derive(Clone)]
pub struct PasswordResetClient {
    client: FlowfullClient,
}

impl PasswordResetClient {
    pub(crate) fn new(client: FlowfullClient) -> Self {
        Self { client }
    }

    pub async fn request(&self, data: PasswordResetRequest) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .post("/auth/password-reset/request", &data)
            .await?;
        Ok(())
    }

    pub async fn validate(&self, token: impl AsRef<str>) -> Result<ValidationResult> {
        let endpoint = format!("/auth/password-reset/validate?token={}", token.as_ref());
        self.client.get(&endpoint).await
    }

    pub async fn complete(&self, data: PasswordResetComplete) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .post("/auth/password-reset/complete", &data)
            .await?;
        Ok(())
    }
}
