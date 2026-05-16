use crate::{Result, auth::types::*, client::FlowfullClient};

#[derive(Clone)]
pub struct SocialAuthClient {
    client: FlowfullClient,
}

impl SocialAuthClient {
    pub(crate) fn new(client: FlowfullClient) -> Self {
        Self { client }
    }

    pub async fn providers(&self) -> Result<SocialProvidersResult> {
        self.client.get("/auth/social/providers").await
    }

    pub async fn start_oauth(&self, provider: SocialProvider) -> Result<String> {
        let endpoint = format!("/auth/social/{provider}/login");
        let value: serde_json::Value = self.client.get(&endpoint).await?;
        if let Some(url) = value.get("url").and_then(|url| url.as_str()) {
            return Ok(url.to_string());
        }
        if let Some(url) = value.get("login_url").and_then(|url| url.as_str()) {
            return Ok(url.to_string());
        }
        Ok(serde_json::from_value(value)?)
    }

    pub async fn login_api(
        &self,
        provider: SocialProvider,
        data: SocialLoginData,
    ) -> Result<LoginResult> {
        let endpoint = format!("/auth/social/{provider}/login-api");
        let result: LoginResult = self.client.post(&endpoint, &data).await?;
        crate::auth::AuthClient::new(self.client.clone())
            .persist_login_result(&result)
            .await?;
        Ok(result)
    }

    pub async fn accounts(&self) -> Result<SocialAccountsResult> {
        self.client.get("/auth/social/accounts").await
    }

    pub async fn unlink(&self, provider: SocialProvider) -> Result<()> {
        let endpoint = format!("/auth/social/accounts/{provider}");
        let _: serde_json::Value = self.client.delete(&endpoint).await?;
        Ok(())
    }
}
