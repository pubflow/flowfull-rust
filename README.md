# Flowfull Rust Client

Async Rust client for Flowfull and Flowless-compatible backends.

## Features

- Async HTTP client powered by `reqwest` and Tokio.
- Backend-safe default: `include_session = false`.
- Session storage using the standard Pubflow keys:
  - `pubflow_session_id`
  - `pubflow_user_data`
- Authentication helpers for login, register, token auth, password reset, social auth, profile, and Bridge validation.
- Universal bracket query syntax: `age[gte]=18`.
- Multipart file uploads.
- Bridge validator helper for backend integrations.
- Payments helper for `/bridge-payment/*` endpoints.
- Optional middleware modules for Rust web frameworks.

## Quick Start

```rust
use flowfull::FlowfullClient;

#[tokio::main]
async fn main() -> flowfull::Result<()> {
    let client = FlowfullClient::new("https://api.example.com")?;

    let users: serde_json::Value = client.get("/users").await?;
    println!("{users:#?}");

    Ok(())
}
```

## CLI or Worker With Session Storage

```rust
use flowfull::{FileStorage, FlowfullClient};

#[tokio::main]
async fn main() -> flowfull::Result<()> {
    let client = FlowfullClient::builder("https://api.example.com")
        .storage(FileStorage::new("./storage"))
        .include_session(true)
        .build_client()?;

    Ok(())
}
```

## Backend API Forwarding User Sessions

For multi-user backends, keep `include_session = false` and pass the incoming user's session per request.

```rust
use flowfull::{FlowfullClient, RequestOptions};

async fn load_profile(client: &FlowfullClient, session_id: String) -> flowfull::Result<serde_json::Value> {
    client
        .get_with_options(
            "/profile",
            RequestOptions::new().session_id(session_id),
        )
        .await
}
```

## Query Builder

```rust
use flowfull::{gte, in_op, FlowfullClient, SortDirection};

async fn query_users(client: FlowfullClient) -> flowfull::Result<serde_json::Value> {
    client
        .query("/users")
        .where_("age", gte(18))
        .where_("status", in_op(["active", "verified"]))
        .sort("created_at", SortDirection::Desc)
        .page(1)
        .limit(20)
        .get()
        .await
}
```

## Status

This is the initial Rust implementation. See [`to-do/01-IMPLEMENTATION-PLAN.md`](./to-do/01-IMPLEMENTATION-PLAN.md) for the full implementation plan and review checklist.

