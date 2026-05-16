pub mod password;
pub mod social;
pub mod token;
pub mod types;

use serde::{Serialize, de::DeserializeOwned};

use crate::{Result, client::FlowfullClient, request::RequestOptions};

pub use password::PasswordResetClient;
pub use social::SocialAuthClient;
pub use token::TokenAuthClient;
pub use types::*;

#[derive(Clone)]
pub struct AuthClient {
    client: FlowfullClient,
}

impl AuthClient {
    pub(crate) fn new(client: FlowfullClient) -> Self {
        Self { client }
    }

    pub fn password(&self) -> PasswordResetClient {
        PasswordResetClient::new(self.client.clone())
    }

    pub fn token(&self) -> TokenAuthClient {
        TokenAuthClient::new(self.client.clone())
    }

    pub fn social(&self) -> SocialAuthClient {
        SocialAuthClient::new(self.client.clone())
    }

    pub async fn login(&self, credentials: LoginCredentials) -> Result<LoginResult> {
        let result: LoginResult = self.client.post("/auth/login", &credentials).await?;
        self.persist_login_result(&result).await?;
        Ok(result)
    }

    pub async fn register(&self, data: RegisterData) -> Result<LoginResult> {
        let result: LoginResult = self.client.post("/auth/register/public", &data).await?;
        self.persist_login_result(&result).await?;
        Ok(result)
    }

    pub async fn logout(&self) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .post("/auth/logout", &serde_json::json!({}))
            .await?;
        self.client.session_manager().clear_all().await
    }

    pub async fn me(&self) -> Result<User> {
        let user: User = self.get_top_level("/auth/me", "user").await?;
        let _ = self.client.session_manager().set_user_data(&user).await;
        Ok(user)
    }

    pub async fn validate_session(&self) -> Result<SessionValidationResult> {
        self.client.get("/auth/validation").await
    }

    pub async fn bridge_validate(
        &self,
        session_id: impl Into<String>,
    ) -> Result<SessionValidationResult> {
        #[derive(Serialize)]
        struct BridgeValidateRequest {
            session_id: String,
        }

        self.client
            .post(
                "/auth/bridge/validate",
                &BridgeValidateRequest {
                    session_id: session_id.into(),
                },
            )
            .await
    }

    pub async fn update_profile(&self, data: ProfileUpdate) -> Result<User> {
        let user: User = self.put_top_level("/auth/profile", &data, "user").await?;
        let _ = self.client.session_manager().set_user_data(&user).await;
        Ok(user)
    }

    pub async fn change_password(&self, data: PasswordChange) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .post("/auth/password-change/self", &data)
            .await?;
        Ok(())
    }

    pub async fn resend_verification(&self, data: ResendVerification) -> Result<()> {
        let _: serde_json::Value = self.client.post("/auth/resend-verification", &data).await?;
        Ok(())
    }

    pub async fn upload_picture(&self, file: crate::upload::UploadFile) -> Result<User> {
        let result: serde_json::Value = self
            .client
            .upload_file("/auth/upload/picture", file)
            .field("picture")
            .send()
            .await?;
        let user = extract_field(result, "user")?;
        let _ = self.client.session_manager().set_user_data(&user).await;
        Ok(user)
    }

    pub async fn delete_picture(&self) -> Result<User> {
        self.client.delete("/auth/upload/picture").await
    }

    pub(crate) async fn persist_login_result(&self, result: &LoginResult) -> Result<()> {
        if !result.session_id.is_empty() {
            self.client
                .session_manager()
                .set_session_id(result.session_id.clone())
                .await?;
        }
        let _ = self
            .client
            .session_manager()
            .set_user_data(&result.user)
            .await;
        Ok(())
    }

    async fn get_top_level<T>(&self, endpoint: &str, field: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let value: serde_json::Value = self.client.get(endpoint).await?;
        extract_field(value, field)
    }

    async fn put_top_level<T, B>(&self, endpoint: &str, body: &B, field: &str) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let value: serde_json::Value = self
            .client
            .request_json(
                reqwest::Method::PUT,
                endpoint,
                Some(body),
                RequestOptions::default(),
            )
            .await?;
        extract_field(value, field)
    }
}

pub(crate) fn extract_field<T>(value: serde_json::Value, field: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    if let Some(data) = value.get(field) {
        return Ok(serde_json::from_value(data.clone())?);
    }
    Ok(serde_json::from_value(value)?)
}
