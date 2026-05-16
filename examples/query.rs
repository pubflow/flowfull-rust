use flowfull::{FlowfullClient, SortDirection, gte, in_op};

#[tokio::main]
async fn main() -> flowfull::Result<()> {
    let client = FlowfullClient::new("https://api.example.com")?;
    let users: serde_json::Value = client
        .query("/users")
        .where_("age", gte(18))
        .where_("status", in_op(["active", "verified"]))
        .sort("created_at", SortDirection::Desc)
        .page(1)
        .limit(20)
        .get()
        .await?;

    println!("{users:#?}");
    Ok(())
}
