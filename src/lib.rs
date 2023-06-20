use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::Method;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use tokio::sync::Mutex;

pub use crate::entities::*;
pub use crate::error::*;

pub mod entities;
pub mod error;
#[cfg(test)]
mod tests;

pub struct Client {
    agent: reqwest::Client,
    header: Header,
    iss: String,
    encoding_key: EncodingKey,
    token: Mutex<ClientToken>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct ClientToken {
    exp: usize,
    token: String,
}

#[derive(Debug, Clone, Serialize)]
struct Claims<'a> {
    iss: &'a String,
    // Optional. Issuer
    iat: usize,
    // Optional. Issued at (as UTC timestamp)
    exp: usize,
    // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    aud: &'a str, // Optional. Audience
}

impl Client {
    fn gen_token(iss: &String, header: &Header, encoding_key: &EncodingKey) -> Result<ClientToken> {
        let now = Utc::now().timestamp() as usize;
        let claims = Claims {
            iss,
            iat: now - (60 * 5),
            exp: now + (60 * 15),
            aud: "appstoreconnect-v1",
        };
        let token = encode(header, &claims, &encoding_key)?;
        Ok(ClientToken {
            exp: now + (60 * 10),
            token,
        })
    }

    async fn load_token(&self) -> Result<String> {
        let mut lock = self.token.lock().await;
        let now = Utc::now().timestamp() as usize;
        if now > lock.exp {
            *lock = Self::gen_token(&self.iss, &self.header, &self.encoding_key)?;
        }
        Ok(lock.token.clone())
    }

    async fn request_raw(
        &self,
        method: Method,
        url: &str,
        query: Option<Vec<(String, String)>>,
        body: Option<serde_json::Value>,
    ) -> Result<(u16, String)> {
        let request = self
            .agent
            .request(method, url)
            .header("Authorization", self.load_token().await?.as_str());
        let request = match query {
            None => request,
            Some(v) => request.query(&v),
        };
        let resp = match body {
            None => request.send(),
            Some(body) => request
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&body)?)
                .send(),
        };
        let resp = resp.await?;
        let status = resp.status();
        let text = resp.text().await?;
        Ok((status.as_u16(), text))
    }

    async fn request<T: for<'de> serde::Deserialize<'de>>(
        &self,
        method: Method,
        url: &str,
        query: Option<Vec<(String, String)>>,
        body: Option<serde_json::Value>,
    ) -> Result<T> {
        let (status, text) = self.request_raw(method, url, query, body).await?;
        if status / 100 == 2 {
            Ok(serde_json::from_str(text.as_str())?)
        } else {
            let e: ServerErrors = serde_json::from_str(text.as_str())?;
            Err(Error::ServerErrors(e))
        }
    }

    async fn request_none_body(
        &self,
        method: Method,
        url: &str,
        query: Option<Vec<(String, String)>>,
        body: Option<serde_json::Value>,
    ) -> Result<()> {
        let (status, text) = self.request_raw(method, url, query, body).await?;
        if status / 100 == 2 {
            Ok(())
        } else {
            let e: ServerErrors = serde_json::from_str(text.as_str())?;
            Err(Error::ServerErrors(e))
        }
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/list_apps

    pub async fn apps(&self, bundle_id_query: BundleIdQuery) -> Result<PageResponse<App>> {
        self.request(
            Method::GET,
            "https://api.appstoreconnect.apple.com/v1/apps",
            Some(bundle_id_query.queries()),
            None,
        )
        .await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/list_bundle_ids

    pub async fn bundle_ids(
        &self,
        bundle_id_query: BundleIdQuery,
    ) -> Result<PageResponse<BundleId>> {
        self.request(
            Method::GET,
            "https://api.appstoreconnect.apple.com/v1/bundleIds",
            Some(bundle_id_query.queries()),
            None,
        )
        .await
    }

    pub async fn bundle_ids_by_url(&self, url: &str) -> Result<PageResponse<BundleId>> {
        self.request(Method::GET, url, None, None).await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/list_and_download_certificates

    pub async fn certificates(
        &self,
        certificate_query: CertificateQuery,
    ) -> Result<PageResponse<Certificate>> {
        self.request(
            Method::GET,
            "https://api.appstoreconnect.apple.com/v1/certificates",
            Some(certificate_query.queries()),
            None,
        )
        .await
    }

    pub async fn certificates_by_url(&self, url: &str) -> Result<PageResponse<Certificate>> {
        self.request(Method::GET, url, None, None).await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/revoke_a_certificate

    pub async fn revoke_a_certificate(&self, certificate_id: impl AsRef<str>) -> Result<()> {
        self.request_none_body(
            Method::DELETE,
            format!(
                "https://api.appstoreconnect.apple.com/v1/certificates/{}",
                certificate_id.as_ref()
            )
            .as_str(),
            None,
            None,
        )
        .await?;
        Ok(())
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/list_and_download_profiles

    pub async fn profiles(&self, profile_query: ProfileQuery) -> Result<PageResponse<Profile>> {
        self.request(
            Method::GET,
            "https://api.appstoreconnect.apple.com/v1/profiles",
            Some(profile_query.queries()),
            None,
        )
        .await
    }

    pub async fn profiles_by_url(&self, url: &str) -> Result<PageResponse<Profile>> {
        self.request(Method::GET, url, None, None).await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/create_a_profile

    pub async fn create_a_profile(
        &self,
        request: ProfileCreateRequest,
    ) -> Result<EntityResponse<Profile>> {
        self.request(
            Method::POST,
            "https://api.appstoreconnect.apple.com/v1/profiles",
            None,
            Some(serde_json::to_value(request)?),
        )
        .await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/delete_a_profile

    pub async fn delete_a_profile(&self, profile_id: &str) -> Result<()> {
        self.request_none_body(
            Method::DELETE,
            format!(
                "https://api.appstoreconnect.apple.com/v1/profiles/{}",
                profile_id
            )
            .as_str(),
            None,
            None,
        )
        .await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/list_devices

    pub async fn devices(&self, device_query: DeviceQuery) -> Result<PageResponse<Device>> {
        self.request(
            Method::GET,
            "https://api.appstoreconnect.apple.com/v1/devices",
            Some(device_query.queries()),
            None,
        )
        .await
    }

    pub async fn devices_by_url(&self, url: &str) -> Result<PageResponse<Device>> {
        self.request(Method::GET, url, None, None).await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/register_a_new_device

    pub async fn register_a_new_device(
        &self,
        request: DeviceCreateRequest,
    ) -> Result<EntityResponse<Device>> {
        self.request(
            Method::POST,
            "https://api.appstoreconnect.apple.com/v1/devices",
            None,
            Some(serde_json::to_value(request)?),
        )
        .await
    }
}

#[derive(Default, Debug, Clone)]
pub struct ClientBuilder {
    iss: Option<String>,
    kid: Option<String>,
    ec_der: Option<Vec<u8>>,
}

impl ClientBuilder {
    pub fn set_iss(&mut self, iss: impl Into<String>) {
        self.iss = Some(iss.into())
    }

    pub fn with_iss(mut self, iss: impl Into<String>) -> Self {
        self.set_iss(iss);
        self
    }

    pub fn set_kid(&mut self, kid: impl Into<String>) {
        self.kid = Some(kid.into())
    }

    pub fn with_kid(mut self, kid: impl Into<String>) -> Self {
        self.set_kid(kid);
        self
    }

    pub fn set_ec_der(&mut self, ec_der: impl Into<Vec<u8>>) {
        self.ec_der = Some(ec_der.into())
    }

    pub fn with_ec_der(mut self, ec_der: impl Into<Vec<u8>>) -> Self {
        self.set_ec_der(ec_der);
        self
    }

    pub fn build(self) -> Result<Client> {
        let mut header = Header::default();
        header.alg = Algorithm::ES256;
        header.kid = match self.kid.clone() {
            Some(kid) => Some(kid),
            None => return Err(Error::message("kid must be set")),
        };
        header.typ = Some("JWT".to_string());

        let iss = match self.iss.clone() {
            Some(iss) => iss,
            None => return Err(Error::message("iss must be set")),
        };

        let ec_der = match self.ec_der.clone() {
            Some(ec_der) => ec_der,
            None => return Err(Error::message("ec_der must be set")),
        };
        let encoding_key = EncodingKey::from_ec_der(ec_der.as_ref());

        let token = Mutex::new(Client::gen_token(&iss, &header, &encoding_key)?);
        Ok(Client {
            agent: Default::default(),
            iss,
            header,
            encoding_key,
            token,
        })
    }
}
