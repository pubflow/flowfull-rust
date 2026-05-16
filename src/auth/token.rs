use crate::{Result, auth::types::*, client::FlowfullClient};

#[derive(Clone)]
pub struct TokenAuthClient {
    client: FlowfullClient,
}

impl TokenAuthClient {
    pub(crate) fn new(client: FlowfullClient) -> Self {
        Self { client }
    }

    pub async fn create(&self, data: TokenCreate) -> Result<TokenCreateResult> {
        self.client.post("/auth/token/create", &data).await
    }

    pub async fn validate(&self, data: TokenValidation) -> Result<ValidationResult> {
        self.client.post("/auth/token/validate", &data).await
    }

    pub async fn validate_get(&self, token: impl AsRef<str>) -> Result<ValidationResult> {
        let endpoint = format!("/auth/token/validate?token={}", token.as_ref());
        self.client.get(&endpoint).await
    }

    pub async fn login(&self, token: impl Into<String>) -> Result<LoginResult> {
        let result: LoginResult = self
            .client
            .post(
                "/auth/token/login",
                &TokenValidation {
                    token: token.into(),
                },
            )
            .await?;
        crate::auth::AuthClient::new(self.client.clone())
            .persist_login_result(&result)
            .await?;
        Ok(result)
    }
}
