use std::time::Duration;

use chrono::Utc;
use serde::Serialize;

use crate::{
    FlowfullClient, Result,
    auth::types::SessionValidationResult,
    bridge::{SessionData, ValidationOptions},
    config::{ClientConfigBuilder, RetryConfig},
};

#[derive(Clone)]
pub struct BridgeValidator {
    client: FlowfullClient,
    endpoint: String,
    include_secret_in_body: bool,
    bridge_secret: Option<String>,
}

pub struct BridgeValidatorBuilder {
    base_url: String,
    bridge_secret: Option<String>,
    endpoint: String,
    timeout: Duration,
    retry: RetryConfig,
    include_secret_in_body: bool,
}

impl BridgeValidator {
    pub fn builder(base_url: impl Into<String>) -> BridgeValidatorBuilder {
        BridgeValidatorBuilder {
            base_url: base_url.into(),
            bridge_secret: None,
            endpoint: "/auth/bridge/validate".to_string(),
            timeout: Duration::from_secs(5),
            retry: RetryConfig::exponential(3, Duration::from_millis(100)),
            include_secret_in_body: false,
        }
    }

    pub async fn validate_session(
        &self,
        session_id: impl Into<String>,
        options: ValidationOptions,
    ) -> Result<SessionData> {
        #[derive(Serialize)]
        struct RequestBody {
            session_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            bridge_secret: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ip: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            user_agent: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            device_id: Option<String>,
        }

        let result: SessionValidationResult = self
            .client
            .post(
                &self.endpoint,
                &RequestBody {
                    session_id: session_id.into(),
                    bridge_secret: self
                        .include_secret_in_body
                        .then(|| self.bridge_secret.clone())
                        .flatten(),
                    ip: options.ip,
                    user_agent: options.user_agent,
                    device_id: options.device_id,
                },
            )
            .await?;

        let user = result.user.clone().ok_or_else(|| {
            crate::FlowfullError::Session("bridge validation returned no user".to_string())
        })?;

        Ok(SessionData {
            user_id: user.id.clone(),
            email: user.email.clone(),
            name: Some(user.name.clone()),
            user_type: user.user_type.clone(),
            organization_id: None,
            permissions: Vec::new(),
            expires_at: result.session.and_then(|session| session.expires_at),
            validated_at: Utc::now(),
            user: Some(user),
        })
    }
}

impl BridgeValidatorBuilder {
    pub fn bridge_secret(mut self, secret: impl Into<String>) -> Self {
        self.bridge_secret = Some(secret.into());
        self
    }

    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = endpoint.into();
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn retry(mut self, retry: RetryConfig) -> Self {
        self.retry = retry;
        self
    }

    pub fn include_secret_in_body(mut self, include: bool) -> Self {
        self.include_secret_in_body = include;
        self
    }

    pub fn build(self) -> Result<BridgeValidator> {
        let mut builder = ClientConfigBuilder::new(&self.base_url)?
            .timeout(self.timeout)
            .retry(self.retry);

        if let Some(secret) = &self.bridge_secret {
            builder = builder.header("X-Bridge-Secret", secret)?;
        }

        Ok(BridgeValidator {
            client: builder.build_client()?,
            endpoint: self.endpoint,
            include_secret_in_body: self.include_secret_in_body,
            bridge_secret: self.bridge_secret,
        })
    }
}
