use std::fmt::{Display, Formatter};

use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    Key(jsonwebtoken::errors::Error),
    Convert(serde_json::Error),
    Reqwest(reqwest::Error),
    ServerErrors(ServerErrors),
    Message(ErrorMessage),
    Other(Box<dyn std::error::Error + Sync + Send>),
}

impl Error {
    pub(crate) fn message(content: impl Into<String>) -> Self {
        Self::Message(ErrorMessage {
            content: content.into(),
        })
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_struct("apple_development::Error");
        match self {
            Error::Key(err) => {
                builder.field("kind", &"Key");
                builder.field("source", err);
            }
            Error::Convert(err) => {
                builder.field("kind", &"Convert");
                builder.field("source", err);
            }
            Error::Reqwest(err) => {
                builder.field("kind", &"Reqwest");
                builder.field("source", err);
            }
            Error::ServerErrors(err) => {
                builder.field("kind", &"ServerErrors");
                builder.field("source", err);
            }
            Error::Message(err) => {
                builder.field("kind", &"Message");
                builder.field("source", err);
            }
            Error::Other(err) => {
                builder.field("kind", &"Other");
                builder.field("source", err);
            }
        }
        builder.finish()
    }
}

impl std::error::Error for Error {}

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerErrors {
    pub errors: Vec<ServerError>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerError {
    pub status: String,
    pub code: String,
    pub title: String,
    pub detail: String,
}

impl Display for ServerErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_struct("apple_development::ServerErrors");
        builder.field("errors", &self.errors);
        builder.finish()
    }
}

impl std::error::Error for ServerErrors {}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::Key(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Convert(value)
    }
}

#[derive(Default, Debug, Clone)]
pub struct ErrorMessage {
    pub content: String,
}

impl Display for ErrorMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_struct("apple_development::ErrorMessage");
        builder.field("content", &self.content);
        builder.finish()
    }
}

impl std::error::Error for ErrorMessage {}
