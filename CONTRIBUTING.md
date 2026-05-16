# Contributing

Run these before submitting changes:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

The crate defaults to backend-safe session behavior. Do not change `include_session` to default `true` for Rust.

