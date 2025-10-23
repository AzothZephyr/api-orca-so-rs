# orca public api client

a rust client library for the orca public api at api.orca.so. you can find their docs [on their documentation website.](https://api.orca.so/docs)

## usage

```rust,no_run
use orca_public_api_client::client::client::OrcaClient;

#[tokio::main]
async fn main() {
    let client = OrcaClient::new();
    let protocol_info = client.get_protocol_info("solana").await.unwrap();
    println!("{:?}", protocol_info);
}
```

## installation

add the following to your `cargo.toml` file:

```toml
[dependencies]
api-orca-so-rs = { git = "https://github.com/AzothZephyr/api-orca-so-rs" }
```
