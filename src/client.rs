use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::header::HeaderMap;
use reqwest::header::{HeaderValue, AUTHORIZATION};
use reqwest::Method;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

use crate::entities::*;
use crate::error::*;
use crate::raw::{RawClient, RawResponse};
use crate::transport::{ReqwestTransport, Transport, TransportRequest};

pub struct Client {
    transport: Arc<dyn Transport>,
    base_url: String,
    header: Header,
    iss: String,
    encoding_key: EncodingKey,
    token: Mutex<ClientToken>,
    retry: RetryConfig,
    retry_non_idempotent: bool,
    max_concurrency: Option<Semaphore>,
    default_headers: HeaderMap,
    on_request: Option<Arc<dyn Fn(&RequestMeta) + Send + Sync>>,
    on_response: Option<Arc<dyn Fn(&ResponseMeta) + Send + Sync>>,
}

impl Debug for Client {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_struct("Client");
        builder.field("base_url", &self.base_url);
        builder.field("kid", &self.header.kid);
        builder.field("alg", &self.header.alg);
        builder.field("iss", &self.iss);
        builder.field("retry", &self.retry);
        builder.field("retry_non_idempotent", &self.retry_non_idempotent);
        builder.field("has_max_concurrency", &self.max_concurrency.is_some());
        builder.field("default_headers_len", &self.default_headers.len());
        builder.field("has_on_request", &self.on_request.is_some());
        builder.field("has_on_response", &self.on_response.is_some());
        builder.finish()
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct ClientToken {
    exp: usize,
    token: String,
}

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: usize,
    pub min_backoff_ms: u64,
    pub max_backoff_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            min_backoff_ms: 200,
            max_backoff_ms: 2_000,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RequestMeta {
    pub method: Method,
    pub url: String,
    pub attempt: usize,
}

#[derive(Debug, Clone)]
pub struct ResponseMeta {
    pub method: Method,
    pub url: String,
    pub attempt: usize,
    pub status: Option<u16>,
    pub elapsed_ms: u128,
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
    fn can_retry_method(&self, method: &Method) -> bool {
        matches!(*method, Method::GET | Method::DELETE) || self.retry_non_idempotent
    }

    fn jitter_ms(max_ms: u64) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos();
        (nanos as u64) % (max_ms.saturating_add(1))
    }

    fn retry_after_ms(headers: &HeaderMap) -> Option<u64> {
        use chrono::{DateTime, Utc};

        let value = headers.get("retry-after")?.to_str().ok()?.trim();
        if let Ok(secs) = value.parse::<u64>() {
            return Some(secs.saturating_mul(1_000));
        }
        let dt_utc = DateTime::parse_from_rfc2822(value)
            .map(|dt| dt.with_timezone(&Utc))
            .or_else(|_| DateTime::parse_from_rfc3339(value).map(|dt| dt.with_timezone(&Utc)))
            .ok()?;
        let now = Utc::now();
        let ms = (dt_utc - now).num_milliseconds();
        Some(ms.max(0) as u64)
    }

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

    pub fn raw(&self) -> RawClient<'_> {
        RawClient::new(self)
    }

    fn url(&self, path_or_url: &str) -> String {
        if path_or_url.starts_with("https://") || path_or_url.starts_with("http://") {
            return path_or_url.to_string();
        }
        let base = self.base_url.trim_end_matches('/');
        if path_or_url.starts_with('/') {
            format!("{base}{path_or_url}")
        } else {
            format!("{base}/{path_or_url}")
        }
    }

    pub(crate) async fn request_raw(
        &self,
        method: Method,
        path_or_url: &str,
        query: Vec<(String, String)>,
        body: Option<serde_json::Value>,
        headers: HeaderMap,
    ) -> Result<RawResponse> {
        use std::time::Duration;
        use std::time::Instant;

        let _permit = match &self.max_concurrency {
            None => None,
            Some(semaphore) => Some(
                semaphore
                    .acquire()
                    .await
                    .map_err(|_| Error::message("request semaphore closed"))?,
            ),
        };

        let url = self.url(path_or_url);
        let mut attempt: usize = 0;
        let can_retry_method = self.can_retry_method(&method);
        let body_text = body.map(|v| serde_json::to_string(&v)).transpose()?;

        loop {
            if let Some(on_request) = self.on_request.as_ref() {
                on_request(&RequestMeta {
                    method: method.clone(),
                    url: url.clone(),
                    attempt,
                });
            }
            let started = Instant::now();

            let token = self.load_token().await?;

            let mut all_headers = self.default_headers.clone();
            all_headers.extend(headers.clone());
            let auth_value = HeaderValue::from_str(format!("Bearer {token}").as_str())
                .map_err(|_| Error::message("invalid bearer token header"))?;
            all_headers.insert(AUTHORIZATION, auth_value);

            let resp = match self
                .transport
                .execute(TransportRequest {
                    method: method.clone(),
                    url: url.clone(),
                    query: query.clone(),
                    body: body_text.clone(),
                    headers: all_headers,
                })
                .await
            {
                Ok(resp) => resp,
                Err(err) => {
                    if let Some(on_response) = self.on_response.as_ref() {
                        on_response(&ResponseMeta {
                            method: method.clone(),
                            url: url.clone(),
                            attempt,
                            status: None,
                            elapsed_ms: started.elapsed().as_millis(),
                        });
                    }
                    let is_retryable_transport_error = match &err {
                        Error::Reqwest(err) => err.is_timeout() || err.is_connect(),
                        _ => false,
                    };
                    let can_retry = can_retry_method
                        && attempt < self.retry.max_retries
                        && is_retryable_transport_error;
                    if !can_retry {
                        return Err(err);
                    }
                    attempt += 1;
                    let backoff_ms = self
                        .retry
                        .min_backoff_ms
                        .saturating_mul(1u64.saturating_mul((attempt - 1) as u64));
                    let backoff_ms = backoff_ms.min(self.retry.max_backoff_ms);
                    tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                    continue;
                }
            };

            let status = resp.status;
            let headers = resp.headers.clone();
            let body_text = resp.body;

            if let Some(on_response) = self.on_response.as_ref() {
                on_response(&ResponseMeta {
                    method: method.clone(),
                    url: url.clone(),
                    attempt,
                    status: Some(status),
                    elapsed_ms: started.elapsed().as_millis(),
                });
            }

            let is_retryable_status = status == 429 || status / 100 == 5;
            if can_retry_method && attempt < self.retry.max_retries && is_retryable_status {
                attempt += 1;
                let backoff_ms = if status == 429 {
                    Self::retry_after_ms(&headers).map(|ms| {
                        let ms = ms.saturating_add(Self::jitter_ms(250));
                        ms.min(self.retry.max_backoff_ms)
                    })
                } else {
                    None
                }
                .unwrap_or_else(|| {
                    self.retry
                        .min_backoff_ms
                        .saturating_mul(1u64.saturating_mul((attempt - 1) as u64))
                        .min(self.retry.max_backoff_ms)
                        .saturating_add(Self::jitter_ms(100))
                });
                tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                continue;
            }

            return Ok(RawResponse {
                status,
                headers,
                body: body_text,
            });
        }
    }

    pub(crate) async fn request_json<T: for<'de> serde::Deserialize<'de>>(
        &self,
        method: Method,
        path_or_url: &str,
        query: Vec<(String, String)>,
        body: Option<serde_json::Value>,
        headers: HeaderMap,
    ) -> Result<T> {
        let resp = self
            .request_raw(method, path_or_url, query, body, headers)
            .await?;
        if resp.status / 100 == 2 {
            Ok(serde_json::from_str(resp.body.as_str())?)
        } else if let Ok(e) = serde_json::from_str::<ServerErrors>(resp.body.as_str()) {
            Err(Error::ServerErrors(e))
        } else {
            Err(Error::http_with_headers(resp.status, resp.headers, resp.body))
        }
    }

    pub(crate) async fn request_unit(
        &self,
        method: Method,
        path_or_url: &str,
        query: Vec<(String, String)>,
        body: Option<serde_json::Value>,
        headers: HeaderMap,
    ) -> Result<()> {
        let resp = self
            .request_raw(method, path_or_url, query, body, headers)
            .await?;
        if resp.status / 100 == 2 {
            Ok(())
        } else if let Ok(e) = serde_json::from_str::<ServerErrors>(resp.body.as_str()) {
            Err(Error::ServerErrors(e))
        } else {
            Err(Error::http_with_headers(resp.status, resp.headers, resp.body))
        }
    }

    async fn request<T: for<'de> serde::Deserialize<'de>>(
        &self,
        method: Method,
        url: &str,
        query: Option<Vec<(String, String)>>,
        body: Option<serde_json::Value>,
    ) -> Result<T> {
        self.request_json(
            method,
            url,
            query.unwrap_or_default(),
            body,
            HeaderMap::new(),
        )
        .await
    }

    async fn request_none_body(
        &self,
        method: Method,
        url: &str,
        query: Option<Vec<(String, String)>>,
        body: Option<serde_json::Value>,
    ) -> Result<()> {
        self.request_unit(
            method,
            url,
            query.unwrap_or_default(),
            body,
            HeaderMap::new(),
        )
        .await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/list_apps

    pub async fn apps(&self, bundle_id_query: BundleIdQuery) -> Result<PageResponse<App>> {
        self.request(
            Method::GET,
            "/v1/apps",
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
            "/v1/bundleIds",
            Some(bundle_id_query.queries()),
            None,
        )
        .await
    }

    pub async fn bundle_ids_by_url(&self, url: &str) -> Result<PageResponse<BundleId>> {
        self.request(Method::GET, url, None, None).await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/register_a_new_bundle_id
    // POST https://api.appstoreconnect.apple.com/v1/bundleIds

    pub async fn register_new_bundle_id(
        &self,
        request: BundleIdCreateRequest,
    ) -> Result<EntityResponse<BundleId>> {
        self.request(
            Method::POST,
            "/v1/bundleIds",
            None,
            Some(serde_json::to_value(request)?),
        )
        .await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/list_all_capabilities_for_a_bundle_id
    // GET https://api.appstoreconnect.apple.com/v1/bundleIds/{id}/bundleIdCapabilities

    pub async fn bundle_id_capabilities(
        &self,
        bundle_id: &str,
    ) -> Result<BundleIdCapabilitiesWithoutIncludesResponse> {
        self.request(
            Method::GET,
            format!("/v1/bundleIds/{}/bundleIdCapabilities", bundle_id).as_str(),
            None,
            None,
        )
        .await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/list_and_download_certificates

    pub async fn certificates(
        &self,
        certificate_query: CertificateQuery,
    ) -> Result<PageResponse<Certificate>> {
        self.request(
            Method::GET,
            "/v1/certificates",
            Some(certificate_query.queries()),
            None,
        )
        .await
    }

    pub async fn certificates_by_url(&self, url: &str) -> Result<PageResponse<Certificate>> {
        self.request(Method::GET, url, None, None).await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/revoke_a_certificate

    pub async fn revoke_certificate(&self, certificate_id: impl AsRef<str>) -> Result<()> {
        self.request_none_body(
            Method::DELETE,
            format!(
                "/v1/certificates/{}",
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
            "/v1/profiles",
            Some(profile_query.queries()),
            None,
        )
        .await
    }

    pub async fn profiles_by_url(&self, url: &str) -> Result<PageResponse<Profile>> {
        self.request(Method::GET, url, None, None).await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/create_a_profile

    pub async fn create_profile(
        &self,
        request: ProfileCreateRequest,
    ) -> Result<EntityResponse<Profile>> {
        self.request(
            Method::POST,
            "/v1/profiles",
            None,
            Some(serde_json::to_value(request)?),
        )
        .await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/delete_a_profile

    pub async fn delete_profile(&self, profile_id: &str) -> Result<()> {
        self.request_none_body(
            Method::DELETE,
            format!("/v1/profiles/{}", profile_id).as_str(),
            None,
            None,
        )
        .await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/list_devices

    pub async fn devices(&self, device_query: DeviceQuery) -> Result<PageResponse<Device>> {
        self.request(
            Method::GET,
            "/v1/devices",
            Some(device_query.queries()),
            None,
        )
        .await
    }

    pub async fn devices_by_url(&self, url: &str) -> Result<PageResponse<Device>> {
        self.request(Method::GET, url, None, None).await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/register_a_new_device

    pub async fn register_new_device(
        &self,
        request: DeviceCreateRequest,
    ) -> Result<EntityResponse<Device>> {
        self.request(
            Method::POST,
            "/v1/devices",
            None,
            Some(serde_json::to_value(request)?),
        )
        .await
    }

    // https://api.appstoreconnect.apple.com/v1/users

    pub async fn users(&self, users_query: UsersQuery) -> Result<PageResponse<User>> {
        self.request(
            Method::GET,
            "/v1/users",
            Some(users_query.queries()),
            None,
        )
        .await
    }

    pub async fn users_by_url(&self, url: &str) -> Result<PageResponse<User>> {
        self.request(Method::GET, url, None, None).await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/read_user_information

    pub async fn user_information(&self, user_id: &str) -> Result<EntityResponse<User>> {
        self.request(
            Method::GET,
            format!("/v1/users/{}", user_id).as_str(),
            None,
            None,
        )
        .await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/modify_a_user_account

    pub async fn modify_user(
        &self,
        user_id: &str,
        data: UserUpdateRequest,
    ) -> Result<EntityResponse<User>> {
        self.request(
            Method::PATCH,
            format!("/v1/users/{}", user_id).as_str(),
            None,
            Some(serde_json::to_value(data)?),
        )
        .await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/remove_a_user_account

    pub async fn remove_user(&self, user_id: &str) -> Result<()> {
        self.request_none_body(
            Method::DELETE,
            format!("/v1/users/{}", user_id).as_str(),
            None,
            None,
        )
        .await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/list_all_apps_visible_to_a_user
    // https://api.appstoreconnect.apple.com/v1/users/{id}/visibleApps

    pub async fn user_visible_apps(
        &self,
        user_id: &str,
        user_visible_apps_query: UserVisibleAppsQuery,
    ) -> Result<PageResponse<App>> {
        self.request(
            Method::GET,
            format!("/v1/users/{user_id}/visibleApps").as_str(),
            Some(user_visible_apps_query.queries()),
            None,
        )
        .await
    }

    // https://developer.apple.com/documentation/appstoreconnectapi/create_a_certificate
    // https://api.appstoreconnect.apple.com/v1/certificates

    // csrContent:
    //
    // 1.
    // create an private key:
    //     `openssl genpkey -algorithm RSA -out key.key -aes256`
    // or create an private key without encryption:
    //     `openssl genpkey -algorithm RSA -out key.key`
    //
    // 2.
    // create a certificate signing request:
    //     `openssl req -new -key key.key -out csr.csr`

    pub async fn create_certificate(
        &self,
        request: CertificateCreateRequest,
    ) -> Result<EntityResponse<Certificate>> {
        self.request(
            Method::POST,
            "/v1/certificates",
            None,
            Some(serde_json::to_value(request)?),
        )
        .await
    }
}

#[derive(Default, Clone)]
pub struct ClientBuilder {
    iss: Option<String>,
    kid: Option<String>,
    ec_der: Option<Vec<u8>>,
    token: Option<String>,
    base_url: Option<String>,
    agent: Option<reqwest::Client>,
    transport: Option<Arc<dyn Transport>>,
    retry: RetryConfig,
    retry_non_idempotent: bool,
    max_concurrency: Option<usize>,
    default_headers: HeaderMap,
    on_request: Option<Arc<dyn Fn(&RequestMeta) + Send + Sync>>,
    on_response: Option<Arc<dyn Fn(&ResponseMeta) + Send + Sync>>,
}

impl Debug for ClientBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_struct("ClientBuilder");
        builder.field("iss_is_set", &self.iss.is_some());
        builder.field("kid_is_set", &self.kid.is_some());
        builder.field("ec_der_len", &self.ec_der.as_ref().map(|v| v.len()));
        builder.field("token_is_set", &self.token.is_some());
        builder.field("base_url", &self.base_url);
        builder.field("agent_is_set", &self.agent.is_some());
        builder.field("transport_is_set", &self.transport.is_some());
        builder.field("retry", &self.retry);
        builder.field("retry_non_idempotent", &self.retry_non_idempotent);
        builder.field("max_concurrency", &self.max_concurrency);
        builder.field("default_headers_len", &self.default_headers.len());
        builder.field("has_on_request", &self.on_request.is_some());
        builder.field("has_on_response", &self.on_response.is_some());
        builder.finish()
    }
}

impl ClientBuilder {
    pub fn set_token(&mut self, token: impl Into<String>) {
        self.token = Some(token.into())
    }

    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.set_token(token);
        self
    }

    pub fn set_transport(&mut self, transport: Arc<dyn Transport>) {
        self.transport = Some(transport)
    }

    pub fn with_transport(mut self, transport: Arc<dyn Transport>) -> Self {
        self.set_transport(transport);
        self
    }

    pub fn set_retry_non_idempotent(&mut self, enabled: bool) {
        self.retry_non_idempotent = enabled
    }

    pub fn with_retry_non_idempotent(mut self, enabled: bool) -> Self {
        self.set_retry_non_idempotent(enabled);
        self
    }
    pub fn set_on_request(
        &mut self,
        on_request: impl Fn(&RequestMeta) + Send + Sync + 'static,
    ) {
        self.on_request = Some(Arc::new(on_request))
    }

    pub fn with_on_request(
        mut self,
        on_request: impl Fn(&RequestMeta) + Send + Sync + 'static,
    ) -> Self {
        self.set_on_request(on_request);
        self
    }

    pub fn set_on_response(
        &mut self,
        on_response: impl Fn(&ResponseMeta) + Send + Sync + 'static,
    ) {
        self.on_response = Some(Arc::new(on_response))
    }

    pub fn with_on_response(
        mut self,
        on_response: impl Fn(&ResponseMeta) + Send + Sync + 'static,
    ) -> Self {
        self.set_on_response(on_response);
        self
    }

    pub fn set_base_url(&mut self, base_url: impl Into<String>) {
        self.base_url = Some(base_url.into())
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.set_base_url(base_url);
        self
    }

    pub fn set_agent(&mut self, agent: reqwest::Client) {
        self.agent = Some(agent)
    }

    pub fn with_agent(mut self, agent: reqwest::Client) -> Self {
        self.set_agent(agent);
        self
    }

    pub fn set_retry(&mut self, retry: RetryConfig) {
        self.retry = retry
    }

    pub fn with_retry(mut self, retry: RetryConfig) -> Self {
        self.set_retry(retry);
        self
    }

    pub fn set_max_concurrency(&mut self, max_concurrency: usize) {
        self.max_concurrency = Some(max_concurrency)
    }

    pub fn with_max_concurrency(mut self, max_concurrency: usize) -> Self {
        self.set_max_concurrency(max_concurrency);
        self
    }

    pub fn set_default_headers(&mut self, headers: HeaderMap) {
        self.default_headers = headers
    }

    pub fn with_default_headers(mut self, headers: HeaderMap) -> Self {
        self.set_default_headers(headers);
        self
    }

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
        header.typ = Some("JWT".to_string());

        let (iss, encoding_key, token) = match self.token {
            Some(token) => (
                self.iss.unwrap_or_default(),
                EncodingKey::from_ec_der(&[]),
                Mutex::new(ClientToken {
                    exp: usize::MAX,
                    token,
                }),
            ),
            None => {
                header.kid = match self.kid.clone() {
                    Some(kid) => Some(kid),
                    None => return Err(Error::message("kid must be set")),
                };

                let iss = match self.iss.clone() {
                    Some(iss) => iss,
                    None => return Err(Error::message("iss must be set")),
                };

                let ec_der = match self.ec_der.clone() {
                    Some(ec_der) => ec_der,
                    None => return Err(Error::message("ec_der must be set")),
                };
                let encoding_key = EncodingKey::from_ec_der(ec_der.as_ref());
                let token = Client::gen_token(&iss, &header, &encoding_key)?;

                (iss, encoding_key, Mutex::new(token))
            }
        };

        let base_url = self
            .base_url
            .unwrap_or_else(|| "https://api.appstoreconnect.apple.com".to_string());

        let transport: Arc<dyn Transport> = match self.transport {
            Some(transport) => transport,
            None => Arc::new(ReqwestTransport::new(self.agent.unwrap_or_default())),
        };
        Ok(Client {
            transport,
            base_url,
            iss,
            header,
            encoding_key,
            token,
            retry: self.retry,
            retry_non_idempotent: self.retry_non_idempotent,
            max_concurrency: self.max_concurrency.map(Semaphore::new),
            default_headers: self.default_headers,
            on_request: self.on_request,
            on_response: self.on_response,
        })
    }
}
