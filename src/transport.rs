use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use reqwest::header::HeaderMap;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::raw::RawResponse;

#[derive(Debug, Clone)]
pub struct TransportRequest {
    pub method: Method,
    pub url: String,
    pub query: Vec<(String, String)>,
    pub body: Option<String>,
    pub headers: HeaderMap,
}

pub trait Transport: Send + Sync {
    fn execute<'a>(
        &'a self,
        request: TransportRequest,
    ) -> Pin<Box<dyn Future<Output = Result<RawResponse>> + Send + 'a>>;
}

#[derive(Debug, Clone)]
pub struct ReqwestTransport {
    client: reqwest::Client,
}

impl ReqwestTransport {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

impl Default for ReqwestTransport {
    fn default() -> Self {
        Self::new(reqwest::Client::default())
    }
}

impl Transport for ReqwestTransport {
    fn execute<'a>(
        &'a self,
        request: TransportRequest,
    ) -> Pin<Box<dyn Future<Output = Result<RawResponse>> + Send + 'a>> {
        Box::pin(async move {
            let mut builder = self
                .client
                .request(request.method, request.url.as_str())
                .headers(request.headers);
            if !request.query.is_empty() {
                builder = builder.query(&request.query);
            }
            if let Some(body) = request.body {
                builder = builder.header("Content-Type", "application/json").body(body);
            }

            let resp = builder.send().await?;
            let status = resp.status().as_u16();
            let headers = resp.headers().clone();
            let body = resp.text().await?;
            Ok(RawResponse {
                status,
                headers,
                body,
            })
        })
    }
}

#[derive(Clone)]
pub struct MockTransport {
    handler: Arc<dyn Fn(TransportRequest) -> Result<RawResponse> + Send + Sync>,
}

impl std::fmt::Debug for MockTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MockTransport").finish()
    }
}

impl MockTransport {
    pub fn new(handler: impl Fn(TransportRequest) -> Result<RawResponse> + Send + Sync + 'static) -> Self {
        Self {
            handler: Arc::new(handler),
        }
    }
}

impl Transport for MockTransport {
    fn execute<'a>(
        &'a self,
        request: TransportRequest,
    ) -> Pin<Box<dyn Future<Output = Result<RawResponse>> + Send + 'a>> {
        let handler = self.handler.clone();
        Box::pin(async move { (handler)(request) })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TransportRequestRecord {
    method: String,
    url: String,
    query: Vec<(String, String)>,
    body: Option<String>,
    headers: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RawResponseRecord {
    status: u16,
    headers: Vec<(String, String)>,
    body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExchangeRecord {
    request: TransportRequestRecord,
    response: RawResponseRecord,
}

fn headers_to_vec(headers: &HeaderMap) -> Vec<(String, String)> {
    headers
        .iter()
        .filter_map(|(k, v)| Some((k.to_string(), v.to_str().ok()?.to_string())))
        .collect()
}

fn vec_to_headers(pairs: &[(String, String)]) -> HeaderMap {
    let mut headers = HeaderMap::new();
    for (k, v) in pairs {
        if let (Ok(name), Ok(value)) = (
            reqwest::header::HeaderName::from_bytes(k.as_bytes()),
            reqwest::header::HeaderValue::from_str(v),
        ) {
            headers.insert(name, value);
        }
    }
    headers
}

#[derive(Debug, Clone)]
pub struct RecordingTransport<T> {
    inner: T,
    path: PathBuf,
    lock: Arc<Mutex<()>>,
}

impl<T> RecordingTransport<T> {
    pub fn new(inner: T, path: impl Into<PathBuf>) -> Self {
        Self {
            inner,
            path: path.into(),
            lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl<T> Transport for RecordingTransport<T>
where
    T: Transport,
{
    fn execute<'a>(
        &'a self,
        request: TransportRequest,
    ) -> Pin<Box<dyn Future<Output = Result<RawResponse>> + Send + 'a>> {
        Box::pin(async move {
            let request_record = TransportRequestRecord {
                method: request.method.to_string(),
                url: request.url.clone(),
                query: request.query.clone(),
                body: request.body.clone(),
                headers: headers_to_vec(&request.headers),
            };

            let response = self.inner.execute(request).await?;
            let response_record = RawResponseRecord {
                status: response.status,
                headers: headers_to_vec(&response.headers),
                body: response.body.clone(),
            };

            let exchange = ExchangeRecord {
                request: request_record,
                response: response_record,
            };
            let _guard = self
                .lock
                .lock()
                .map_err(|_| Error::message("recording transport lock poisoned"))?;
            let line = serde_json::to_string(&exchange)?;
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.path)
                .and_then(|mut f| {
                    use std::io::Write;
                    writeln!(f, "{line}")
                })
                .map_err(|e| Error::message(format!("recording transport write failed: {e}")))?;

            Ok(response)
        })
    }
}

#[derive(Debug, Clone)]
pub struct ReplayTransport {
    exchanges: Arc<Vec<ExchangeRecord>>,
    index: Arc<std::sync::atomic::AtomicUsize>,
    strict: bool,
}

impl ReplayTransport {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .map_err(|e| Error::message(format!("replay transport read failed: {e}")))?;
        let mut exchanges = Vec::new();
        for (i, line) in content.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let exchange: ExchangeRecord = serde_json::from_str(line).map_err(|e| {
                Error::message(format!("replay transport parse failed at line {}: {e}", i + 1))
            })?;
            exchanges.push(exchange);
        }
        Ok(Self {
            exchanges: Arc::new(exchanges),
            index: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            strict: true,
        })
    }

    pub fn set_strict(&mut self, strict: bool) {
        self.strict = strict
    }

    pub fn with_strict(mut self, strict: bool) -> Self {
        self.set_strict(strict);
        self
    }
}

impl Transport for ReplayTransport {
    fn execute<'a>(
        &'a self,
        request: TransportRequest,
    ) -> Pin<Box<dyn Future<Output = Result<RawResponse>> + Send + 'a>> {
        Box::pin(async move {
            let i = self.index.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let Some(exchange) = self.exchanges.get(i) else {
                return Err(Error::message("replay transport exhausted"));
            };

            if self.strict {
                if exchange.request.method != request.method.to_string() || exchange.request.url != request.url
                {
                    return Err(Error::message(format!(
                        "replay mismatch: expected {} {}, got {} {}",
                        exchange.request.method,
                        exchange.request.url,
                        request.method,
                        request.url
                    )));
                }
            }

            Ok(RawResponse {
                status: exchange.response.status,
                headers: vec_to_headers(&exchange.response.headers),
                body: exchange.response.body.clone(),
            })
        })
    }
}

