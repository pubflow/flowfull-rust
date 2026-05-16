use std::{sync::Arc, time::Duration};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use url::Url;

use crate::{FlowfullError, error::Result, storage::Storage};

pub type SessionProvider = Arc<
    dyn Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Option<String>>> + Send>>
        + Send
        + Sync,
>;

#[derive(Clone)]
pub struct ClientConfig {
    pub base_url: Url,
    pub session_id: Option<String>,
    pub session_provider: Option<SessionProvider>,
    pub include_session: bool,
    pub session_header: HeaderName,
    pub session_cookie: String,
    pub timeout: Duration,
    pub headers: HeaderMap,
    pub retry: RetryConfig,
    pub storage: Option<Arc<dyn Storage>>,
}

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub attempts: usize,
    pub delay: Duration,
    pub exponential: bool,
    pub max_delay: Option<Duration>,
    pub retry_statuses: Vec<u16>,
    pub retry_non_idempotent: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            attempts: 1,
            delay: Duration::from_secs(1),
            exponential: false,
            max_delay: Some(Duration::from_secs(30)),
            retry_statuses: vec![408, 429, 500, 502, 503, 504],
            retry_non_idempotent: false,
        }
    }
}

impl RetryConfig {
    pub fn exponential(attempts: usize, delay: Duration) -> Self {
        Self {
            attempts: attempts.max(1),
            delay,
            exponential: true,
            ..Self::default()
        }
    }
}

pub struct ClientConfigBuilder {
    base_url: Url,
    session_id: Option<String>,
    session_provider: Option<SessionProvider>,
    include_session: bool,
    session_header: HeaderName,
    session_cookie: String,
    timeout: Duration,
    headers: HeaderMap,
    retry: RetryConfig,
    storage: Option<Arc<dyn Storage>>,
}

impl ClientConfigBuilder {
    pub fn new(base_url: impl AsRef<str>) -> Result<Self> {
        let mut base_url = Url::parse(base_url.as_ref())?;
        if !base_url.path().ends_with('/') {
            let path = format!("{}/", base_url.path());
            base_url.set_path(&path);
        }

        Ok(Self {
            base_url,
            session_id: None,
            session_provider: None,
            include_session: false,
            session_header: HeaderName::from_static("x-session-id"),
            session_cookie: "session_id".to_string(),
            timeout: Duration::from_secs(30),
            headers: HeaderMap::new(),
            retry: RetryConfig::default(),
            storage: None,
        })
    }

    pub fn session_id(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    pub fn session_provider<F, Fut>(mut self, provider: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<Option<String>>> + Send + 'static,
    {
        self.session_provider = Some(Arc::new(move || Box::pin(provider())));
        self
    }

    pub fn include_session(mut self, include_session: bool) -> Self {
        self.include_session = include_session;
        self
    }

    pub fn session_header(mut self, header: impl AsRef<str>) -> Result<Self> {
        self.session_header = HeaderName::from_bytes(header.as_ref().as_bytes())
            .map_err(|err| FlowfullError::Config(format!("invalid session header: {err}")))?;
        Ok(self)
    }

    pub fn session_cookie(mut self, cookie: impl Into<String>) -> Self {
        self.session_cookie = cookie.into();
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn header(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Result<Self> {
        let key = HeaderName::from_bytes(key.as_ref().as_bytes())
            .map_err(|err| FlowfullError::Config(format!("invalid header name: {err}")))?;
        let value = HeaderValue::from_str(value.as_ref())
            .map_err(|err| FlowfullError::Config(format!("invalid header value: {err}")))?;
        self.headers.insert(key, value);
        Ok(self)
    }

    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    }

    pub fn retry(mut self, retry: RetryConfig) -> Self {
        self.retry = retry;
        self
    }

    pub fn storage<S>(mut self, storage: S) -> Self
    where
        S: Storage + 'static,
    {
        self.storage = Some(Arc::new(storage));
        self
    }

    pub fn shared_storage(mut self, storage: Arc<dyn Storage>) -> Self {
        self.storage = Some(storage);
        self
    }

    pub fn build(self) -> Result<ClientConfig> {
        Ok(ClientConfig {
            base_url: self.base_url,
            session_id: self.session_id,
            session_provider: self.session_provider,
            include_session: self.include_session,
            session_header: self.session_header,
            session_cookie: self.session_cookie,
            timeout: self.timeout,
            headers: self.headers,
            retry: self.retry,
            storage: self.storage,
        })
    }
}
