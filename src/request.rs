use std::time::Duration;

use reqwest::{Method, header::HeaderMap};
use serde::Serialize;
use serde_json::Value;
use url::Url;

#[derive(Debug, Clone, Default)]
pub struct RequestOptions {
    pub headers: HeaderMap,
    pub query: Vec<(String, String)>,
    pub include_session: Option<bool>,
    pub session_id: Option<String>,
    pub timeout: Option<Duration>,
}

impl RequestOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn header(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> crate::Result<Self> {
        let key = reqwest::header::HeaderName::from_bytes(key.as_ref().as_bytes())
            .map_err(|err| crate::FlowfullError::Config(format!("invalid header name: {err}")))?;
        let value = reqwest::header::HeaderValue::from_str(value.as_ref()).map_err(|err| {
            crate::FlowfullError::Config(format!("invalid header value for request option: {err}"))
        })?;
        self.headers.insert(key, value);
        Ok(self)
    }

    pub fn query(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.push((key.into(), value.into()));
        self
    }

    pub fn include_session(mut self, include_session: bool) -> Self {
        self.include_session = Some(include_session);
        self
    }

    pub fn session_id(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

#[derive(Debug, Clone)]
pub struct InternalRequest {
    pub method: Method,
    pub url: Url,
    pub body: Option<Value>,
    pub options: RequestOptions,
}

impl InternalRequest {
    pub fn json<B>(
        method: Method,
        url: Url,
        body: Option<&B>,
        options: RequestOptions,
    ) -> crate::Result<Self>
    where
        B: Serialize + ?Sized,
    {
        let body = match body {
            Some(body) => Some(serde_json::to_value(body)?),
            None => None,
        };
        Ok(Self {
            method,
            url,
            body,
            options,
        })
    }
}
