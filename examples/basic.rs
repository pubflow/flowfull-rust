use flowfull::FlowfullClient;

#[tokio::main]
async fn main() -> flowfull::Result<()> {
    let client = FlowfullClient::new("https://api.example.com")?;
    let data: serde_json::Value = client.get("/health").await?;
    println!("{data:#?}");
    Ok(())
}
