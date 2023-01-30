app store connect
=================

`AppStoreConnect` Client for rust

```toml
appstoreconnect = "0"
```

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // create client
    let client = ClientBuilder::default()
        .with_iss(env!("iss"))
        .with_kid(env!("kid"))
        .with_ec_der(base64::decode(env!("ec_der"))?) // ec_der is base64text from .p8
        .build()?;
    // get find devices
    let devices = client.devices(DeviceQuery {
                filter_name: Some("mini".to_string()),
                ..Default::default()
            }).await?;
    // create or list profile, certs, bundleIds please visit tests.rs
    Ok(())
}
```