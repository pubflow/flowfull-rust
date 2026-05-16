use flowfull::{FileStorage, FlowfullClient, auth::LoginCredentials};

#[tokio::main]
async fn main() -> flowfull::Result<()> {
    let client = FlowfullClient::builder("https://api.example.com")
        .storage(FileStorage::new("./storage"))
        .include_session(true)
        .build_client()?;

    let _result = client
        .auth()
        .login(LoginCredentials {
            email: Some("user@example.com".to_string()),
            user_name: None,
            password: "password123".to_string(),
        })
        .await?;

    Ok(())
}
