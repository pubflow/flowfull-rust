# Quickstart

## Install

```toml
[dependencies]
flowfull = { path = "../flowfull-rust" }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
serde_json = "1"
```

## Basic Request

```rust
let client = flowfull::FlowfullClient::new("https://api.example.com")?;
let data: serde_json::Value = client.get("/health").await?;
```

## Login and Store Session

```rust
let client = flowfull::FlowfullClient::builder("https://api.example.com")
    .storage(flowfull::MemoryStorage::new())
    .include_session(true)
    .build_client()?;

let result = client.auth().login(flowfull::auth::LoginCredentials {
    email: Some("user@example.com".to_string()),
    user_name: None,
    password: "password123".to_string(),
}).await?;
```

