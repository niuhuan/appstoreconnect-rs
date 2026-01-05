app store connect
=================

This repository is an `AppStoreConnect` api client, allow your invoke api in Rust. The full api docs
in [here](https://developer.apple.com/documentation/appstoreconnectapi).

## Easily to use

1. First. You need request `Issuer ID`, `KeyId` and `Key` in the website : https://appstoreconnect.apple.com/access/api.

2. Adding appstoreconnect

   Run this command in your terminal to add the latest version of `appstoreconnect`.
    ```shell
    $ cargo add appstoreconnect
    ```

3. build and use the client

   `iss` : `Issuer ID`  <br />
   `kid` : `KeyId`  <br />
   `ec_der` : base64 text in `key.p8` remove `\n` <br />
    ```rust
    #[tokio::main]
    async fn main() -> Result<()> {
        let iss = std::env::var("APPSTORECONNECT_ISS")?;
        let kid = std::env::var("APPSTORECONNECT_KID")?;
        let ec_der_b64 = std::env::var("APPSTORECONNECT_EC_DER_BASE64")?;
        // create client
        let client = ClientBuilder::default()
            .with_iss(iss)
            .with_kid(kid)
            .with_ec_der(base64::decode(ec_der_b64)?) 
            .build()?;
        // get find devices
        let devices = client.devices(DeviceQuery {
                    filter_name: Some("mini".to_string()),
                    ..Default::default()
                }).await?;
        Ok(())
    }
    ```

4. Escape-hatch (raw request) and paging

   ```rust
   use appstoreconnect::query::Query;

   // any endpoint / any query params
   let apps: appstoreconnect::entities::PageResponse<appstoreconnect::entities::App> =
       client
           .raw()
           .get("/v1/apps")
           .query(Query::new().fields("apps", ["name", "bundleId"]).page_limit(50))
           .send_json()
           .await?;

   // auto follow `links.next`
   let mut pager = apps.pager(&client);
   while let Some(page) = pager.next_page().await? {
       for app in page.data {
           println!("{}", app.attributes.bundle_id);
       }
   }
   ```

5. More example : Create or list profile, certs, bundleIds please
   visit [src/tests.rs](https://github.com/niuhuan/appstoreconnect-rs/blob/master/src/tests.rs)

## features

- [ ] App Store
    - [ ] Apps
        - [x] List Apps
        - [ ] Modify an App
    - [ ] Builds
- [X] Bundle IDs
    - [x] List Bundle IDs
    - [x] Register New Bundle ID
    - [x] List Bundle ID capabilities
- [ ] Bundle ID Capabilities
- [x] Certificates
    - [x] List and Download Certificates
    - [x] Create a Certificate
    - [x] Revoke a Certificate
- [x] Devices
    - [x] Register a New Device
    - [x] List Devices
- [x] Profiles
    - [x] Create a Profile
    - [x] List and Download Profiles
    - [x] Delete a Profile
- [ ] Users
    - [X] List users
    - [X] Read User Information
    - [x] Modify a User Account
    - [x] App Accesses
        - [x] List All Apps Visible to a User
        - [ ] Add Visible Apps to a User
        - [ ] Remove Visible Apps from a User
- [ ] User Invitations
- [ ] Sandbox Testers
