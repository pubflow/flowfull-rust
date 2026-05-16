use std::{future::Future, pin::Pin, sync::Arc};

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::{
    Result,
    bridge::{BridgeValidator, SessionData, ValidationMode},
};

#[derive(Clone)]
pub struct AxumAuthState {
    pub validator: Arc<BridgeValidator>,
    pub validation_mode: ValidationMode,
    pub session_header: String,
    pub allow_query_session: bool,
}

impl AxumAuthState {
    pub fn new(validator: BridgeValidator) -> Self {
        Self {
            validator: Arc::new(validator),
            validation_mode: ValidationMode::Standard,
            session_header: "X-Session-ID".to_string(),
            allow_query_session: false,
        }
    }
}

pub async fn require_auth(
    State(state): State<AxumAuthState>,
    mut request: Request,
    next: Next,
) -> std::result::Result<Response, StatusCode> {
    let session_id = extract_session_id(&request, &state).ok_or(StatusCode::UNAUTHORIZED)?;
    let options = state.validation_mode.build_options(
        request
            .headers()
            .get("x-forwarded-for")
            .and_then(|value| value.to_str().ok())
            .map(str::to_string),
        request
            .headers()
            .get("user-agent")
            .and_then(|value| value.to_str().ok())
            .map(str::to_string),
        request
            .headers()
            .get("x-device-id")
            .and_then(|value| value.to_str().ok())
            .map(str::to_string),
    );

    let session = state
        .validator
        .validate_session(session_id, options)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    request.extensions_mut().insert(session);
    Ok(next.run(request).await)
}

pub async fn optional_auth(
    State(state): State<AxumAuthState>,
    mut request: Request,
    next: Next,
) -> Response {
    if let Some(session_id) = extract_session_id(&request, &state) {
        let options = state.validation_mode.build_options(None, None, None);
        if let Ok(session) = state.validator.validate_session(session_id, options).await {
            request.extensions_mut().insert(session);
        }
    }
    next.run(request).await
}

pub fn require_user_type(
    allowed: Vec<String>,
) -> impl Fn(
    Request,
    Next,
) -> Pin<Box<dyn Future<Output = std::result::Result<Response, StatusCode>> + Send>>
+ Clone
+ Send
+ Sync
+ 'static {
    move |request: Request, next: Next| {
        let allowed = allowed.clone();
        Box::pin(async move {
            let session = request
                .extensions()
                .get::<SessionData>()
                .ok_or(StatusCode::UNAUTHORIZED)?;
            let user_type = session.user_type.as_deref().unwrap_or_default();
            if allowed.iter().any(|allowed| allowed == user_type) {
                Ok(next.run(request).await)
            } else {
                Err(StatusCode::FORBIDDEN)
            }
        })
    }
}

fn extract_session_id(request: &Request, state: &AxumAuthState) -> Option<String> {
    request
        .headers()
        .get(state.session_header.as_str())
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
        .or_else(|| {
            request
                .headers()
                .get("cookie")
                .and_then(|value| value.to_str().ok())
                .and_then(extract_session_cookie)
        })
        .or_else(|| {
            state.allow_query_session.then(|| {
                request.uri().query().and_then(|query| {
                    query.split('&').find_map(|part| {
                        let (key, value) = part.split_once('=')?;
                        (key == "session_id").then(|| value.to_string())
                    })
                })
            })?
        })
}

fn extract_session_cookie(cookie_header: &str) -> Option<String> {
    cookie_header.split(';').find_map(|cookie| {
        let (key, value) = cookie.trim().split_once('=')?;
        (key == "session_id").then(|| value.to_string())
    })
}

pub async fn validate_session(
    validator: &BridgeValidator,
    session_id: impl Into<String>,
) -> Result<SessionData> {
    validator
        .validate_session(session_id.into(), Default::default())
        .await
}
