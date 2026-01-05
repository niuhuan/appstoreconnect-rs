use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Method;
use serde_json::Value;

use crate::client::Client;
use crate::error::Result;
use crate::query::Query;

#[derive(Debug, Clone)]
pub struct RawResponse {
    pub status: u16,
    pub headers: HeaderMap,
    pub body: String,
}

pub struct RawClient<'a> {
    client: &'a Client,
}

impl<'a> RawClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn request(self, method: Method, path_or_url: impl Into<String>) -> RawRequestBuilder<'a> {
        RawRequestBuilder::new(self.client, method, path_or_url)
    }

    pub fn get(self, path_or_url: impl Into<String>) -> RawRequestBuilder<'a> {
        self.request(Method::GET, path_or_url)
    }

    pub fn post(self, path_or_url: impl Into<String>) -> RawRequestBuilder<'a> {
        self.request(Method::POST, path_or_url)
    }

    pub fn patch(self, path_or_url: impl Into<String>) -> RawRequestBuilder<'a> {
        self.request(Method::PATCH, path_or_url)
    }

    pub fn delete(self, path_or_url: impl Into<String>) -> RawRequestBuilder<'a> {
        self.request(Method::DELETE, path_or_url)
    }
}

pub struct RawRequestBuilder<'a> {
    client: &'a Client,
    method: Method,
    path_or_url: String,
    query: Query,
    body: Option<Value>,
    headers: HeaderMap,
}

impl<'a> RawRequestBuilder<'a> {
    pub(crate) fn new(client: &'a Client, method: Method, path_or_url: impl Into<String>) -> Self {
        Self {
            client,
            method,
            path_or_url: path_or_url.into(),
            query: Query::new(),
            body: None,
            headers: HeaderMap::new(),
        }
    }

    pub fn query(mut self, query: Query) -> Self {
        self.query = query;
        self
    }

    pub fn query_kv(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query = self.query.push(key, value);
        self
    }

    pub fn header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(name, value);
        self
    }

    pub fn body_json(mut self, value: Value) -> Self {
        self.body = Some(value);
        self
    }

    pub async fn send_raw(self) -> Result<RawResponse> {
        self.client
            .request_raw(self.method, &self.path_or_url, self.query.build(), self.body, self.headers)
            .await
    }

    pub async fn send_json<T>(self) -> Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        self.client
            .request_json(self.method, &self.path_or_url, self.query.build(), self.body, self.headers)
            .await
    }

    pub async fn send_unit(self) -> Result<()> {
        self.client
            .request_unit(self.method, &self.path_or_url, self.query.build(), self.body, self.headers)
            .await
    }
}
