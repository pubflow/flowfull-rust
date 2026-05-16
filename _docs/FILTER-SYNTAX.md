# Filter Syntax

The query builder uses universal bracket syntax:

```text
field[operator]=value
```

Examples:

```rust
client.query("/products")
    .where_("price", flowfull::gte(100))
    .where_("category", flowfull::in_op(["hardware", "software"]))
    .get::<serde_json::Value>()
    .await?;
```

Generated query:

```text
price[gte]=100&category[in]=hardware,software
```

Supported operators include comparison, string, array, null, range, and custom operators.

