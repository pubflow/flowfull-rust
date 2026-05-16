use flowfull::BridgeValidator;

#[tokio::main]
async fn main() -> flowfull::Result<()> {
    let validator = BridgeValidator::builder("https://flowless.example.com")
        .bridge_secret("replace-with-your-bridge-secret")
        .build()?;

    let session = validator
        .validate_session("ses_example", Default::default())
        .await?;

    println!("validated user: {}", session.user_id);
    Ok(())
}
