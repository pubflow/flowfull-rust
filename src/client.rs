use std::{sync::Arc, time::Duration};

use reqwest::{Method, Url};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use tokio::time::sleep;

use crate::{
    auth::AuthClient,
    bridge::BridgeClient,
    config::{ClientConfig, ClientConfigBuilder},
    error::{FlowfullError, Result},
    query::QueryBuilder,
    request::RequestOptions,
    response::{ApiResponse, RawResponse},
    session::SessionManager,
    upload::UploadBuilder,
};

#[derive(Clone)]
pub struct FlowfullClient {
    http: reqwest::Client,
    config: Arc<ClientConfig>,
    session_manager: SessionManager,
}

impl FlowfullClient {
    pub fn new(base_url: impl AsRef<str>) -> Result<Self> {
        Self::builder(base_url).build_client()
    }

    pub fn builder(base_url: impl AsRef<str>) -> ClientConfigBuilder {
        ClientConfigBuilder::new(base_url).expect("invalid base URL")
    }

    pub fn from_config(config: ClientConfig) -> Result<Self> {
        let http = reqwest::Client::builder().timeout(config.timeout).build()?;
        let config = Arc::new(config);
        let session_manager = SessionManager::new(config.clone());
        Ok(Self {
            http,
            config,
            session_manager,
        })
    }

    pub fn session_manager(&self) -> &SessionManager {
        &self.session_manager
    }

    pub fn auth(&self) -> AuthClient {
        AuthClient::new(self.clone())
    }

    pub fn bridge(&self) -> BridgeClient {
        BridgeClient::new(self.clone())
    }

    #[cfg(feature = "payments")]
    pub fn pay(&self) -> crate::payments::PaymentsClient {
        crate::payments::PaymentsClient::new(self.clone())
    }

    pub fn query(&self, endpoint: impl Into<String>) -> QueryBuilder {
        QueryBuilder::new(self.clone(), endpoint.into())
    }

    pub fn upload_file(
        &self,
        endpoint: impl Into<String>,
        file: crate::upload::UploadFile,
    ) -> UploadBuilder {
        UploadBuilder::new(self.clone(), endpoint.into(), file)
    }

    pub async fn get<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_options(endpoint, RequestOptions::default())
            .await
    }

    pub async fn get_with_options<T>(&self, endpoint: &str, options: RequestOptions) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request_json(Method::GET, endpoint, Option::<&()>::None, options)
            .await
    }

    pub async fn post<T, B>(&self, endpoint: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        self.post_with_options(endpoint, body, RequestOptions::default())
            .await
    }

    pub async fn post_with_options<T, B>(
        &self,
        endpoint: &str,
        body: &B,
        options: RequestOptions,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        self.request_json(Method::POST, endpoint, Some(body), options)
            .await
    }

    pub async fn put<T, B>(&self, endpoint: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        self.request_json(Method::PUT, endpoint, Some(body), RequestOptions::default())
            .await
    }

    pub async fn patch<T, B>(&self, endpoint: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        self.request_json(
            Method::PATCH,
            endpoint,
            Some(body),
            RequestOptions::default(),
        )
        .await
    }

    pub async fn delete<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request_json(
            Method::DELETE,
            endpoint,
            Option::<&()>::None,
            RequestOptions::default(),
        )
        .await
    }

    pub async fn request_json<T, B>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<&B>,
        options: RequestOptions,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let raw = self
            .raw_json_request(method, endpoint, body, options)
            .await?;
        self.parse_json_response(raw)
    }

    pub async fn raw_json_request<B>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<&B>,
        options: RequestOptions,
    ) -> Result<RawResponse>
    where
        B: Serialize + ?Sized,
    {
        let url = self.build_url(endpoint, &options)?;
        let body = match body {
            Some(body) => Some(serde_json::to_value(body)?),
            None => None,
        };
        self.execute_with_retry(method, url, body, options).await
    }

    pub(crate) async fn execute_with_retry(
        &self,
        method: Method,
        url: Url,
        body: Option<Value>,
        options: RequestOptions,
    ) -> Result<RawResponse> {
        let attempts = self.config.retry.attempts.max(1);
        let mut last_error: Option<FlowfullError> = None;

        for attempt in 0..attempts {
            match self
                .execute_once(method.clone(), url.clone(), body.clone(), options.clone())
                .await
            {
                Ok(resp)
                    if self.should_retry_status(method.clone(), resp.status, attempt, attempts) =>
                {
                    last_error = Some(FlowfullError::Api {
                        status: resp.status,
                        message: format!("retryable status {}", resp.status),
                        body: Some(resp.text()),
                    });
                }
                Ok(resp) => return Ok(resp),
                Err(err) if self.should_retry_error(&method, attempt, attempts) => {
                    last_error = Some(err);
                }
                Err(err) => return Err(err),
            }

            if attempt + 1 < attempts {
                sleep(self.retry_delay(attempt)).await;
            }
        }

        Err(last_error.unwrap_or(FlowfullError::RequestFailed))
    }

    async fn execute_once(
        &self,
        method: Method,
        url: Url,
        body: Option<Value>,
        options: RequestOptions,
    ) -> Result<RawResponse> {
        let mut request = self.http.request(method, url);

        for (key, value) in self.config.headers.iter() {
            request = request.header(key, value);
        }
        for (key, value) in options.headers.iter() {
            request = request.header(key, value);
        }

        let include_session = options
            .include_session
            .unwrap_or(self.config.include_session);
        let session_id = match options.session_id {
            Some(session_id) => Some(session_id),
            None if include_session => self.session_manager.get_session_id().await?,
            None => None,
        };
        if let Some(session_id) = session_id {
            if !session_id.is_empty() {
                request = request.header(self.config.session_header.clone(), session_id);
            }
        }

        if let Some(timeout) = options.timeout {
            request = request.timeout(timeout);
        }

        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await?;
        let status = response.status().as_u16();
        let headers = response.headers().clone();
        let body = response.bytes().await?.to_vec();

        Ok(RawResponse {
            status,
            headers,
            body,
        })
    }

    pub(crate) fn parse_json_response<T>(&self, raw: RawResponse) -> Result<T>
    where
        T: DeserializeOwned,
    {
        if !(200..300).contains(&raw.status) {
            return Err(FlowfullError::Api {
                status: raw.status,
                message: raw.text(),
                body: Some(raw.text()),
            });
        }

        let value: Value = serde_json::from_slice(&raw.body)?;
        if value.get("success").is_some() {
            let envelope: ApiResponse<Value> = serde_json::from_value(value.clone())?;
            if !envelope.success {
                return Err(FlowfullError::Api {
                    status: raw.status,
                    message: envelope
                        .error
                        .or(envelope.message)
                        .unwrap_or_else(|| "request failed".to_string()),
                    body: Some(raw.text()),
                });
            }

            if let Some(data) = envelope.data {
                return Ok(serde_json::from_value(data)?);
            }
        }

        Ok(serde_json::from_value(value)?)
    }

    pub(crate) async fn raw_multipart_request(
        &self,
        method: Method,
        endpoint: &str,
        form: reqwest::multipart::Form,
        options: RequestOptions,
    ) -> Result<RawResponse> {
        let url = self.build_url(endpoint, &options)?;
        let mut request = self.http.request(method, url).multipart(form);

        for (key, value) in self.config.headers.iter() {
            request = request.header(key, value);
        }
        for (key, value) in options.headers.iter() {
            request = request.header(key, value);
        }

        let include_session = options
            .include_session
            .unwrap_or(self.config.include_session);
        let session_id = match options.session_id {
            Some(session_id) => Some(session_id),
            None if include_session => self.session_manager.get_session_id().await?,
            None => None,
        };
        if let Some(session_id) = session_id {
            if !session_id.is_empty() {
                request = request.header(self.config.session_header.clone(), session_id);
            }
        }

        if let Some(timeout) = options.timeout {
            request = request.timeout(timeout);
        }

        let response = request.send().await?;
        let status = response.status().as_u16();
        let headers = response.headers().clone();
        let body = response.bytes().await?.to_vec();
        Ok(RawResponse {
            status,
            headers,
            body,
        })
    }

    pub(crate) fn build_url(&self, endpoint: &str, options: &RequestOptions) -> Result<Url> {
        let mut url = if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
            Url::parse(endpoint)?
        } else {
            let endpoint = endpoint.trim_start_matches('/');
            self.config.base_url.join(endpoint)?
        };

        if !options.query.is_empty() {
            let mut pairs = url.query_pairs_mut();
            for (key, value) in &options.query {
                pairs.append_pair(key, value);
            }
        }

        Ok(url)
    }

    fn should_retry_error(&self, method: &Method, attempt: usize, attempts: usize) -> bool {
        attempt + 1 < attempts && (is_idempotent(method) || self.config.retry.retry_non_idempotent)
    }

    fn should_retry_status(
        &self,
        method: Method,
        status: u16,
        attempt: usize,
        attempts: usize,
    ) -> bool {
        attempt + 1 < attempts
            && self.config.retry.retry_statuses.contains(&status)
            && (is_idempotent(&method) || self.config.retry.retry_non_idempotent)
    }

    fn retry_delay(&self, attempt: usize) -> Duration {
        let mut delay = self.config.retry.delay;
        if self.config.retry.exponential {
            delay = delay.saturating_mul(2_u32.saturating_pow(attempt as u32));
        }
        if let Some(max_delay) = self.config.retry.max_delay {
            delay.min(max_delay)
        } else {
            delay
        }
    }
}

impl ClientConfigBuilder {
    pub fn build_client(self) -> Result<FlowfullClient> {
        FlowfullClient::from_config(self.build()?)
    }
}

fn is_idempotent(method: &Method) -> bool {
    matches!(
        *method,
        Method::GET | Method::HEAD | Method::PUT | Method::DELETE | Method::OPTIONS
    )
}
