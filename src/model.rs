use std::collections::HashMap;

use crate::error::OxideKvError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub enum HttpVerb {
    GET,
    POST,
    PUT,
    DELETE,
}

impl std::fmt::Debug for HttpVerb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HttpVerb::GET => "GET",
            HttpVerb::POST => "POST",
            HttpVerb::PUT => "PUT",
            HttpVerb::DELETE => "DELETE",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum StatusCode {
    OK = 200,
    BadRequest = 400,
    ServerError = 500,
}

impl StatusCode {
    pub fn as_u16(&self) -> u16 {
        *self as u16
    }
}

#[derive(Debug, Deserialize)]
pub struct HttpRequest {
    pub verb: Option<HttpVerb>,
    pub key: String,
    pub value: String,
}

impl TryFrom<&str> for HttpRequest {
    type Error = OxideKvError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<HttpRequest>(value)
            .map_err(|e| OxideKvError::Server(format!("Failed to parse JSON: {}", e)))
    }
}

impl HttpRequest {
    pub fn set_verb(mut self, verb: HttpVerb) -> Self {
        self.verb = Some(verb);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpResponse {
    pub message: String,
    pub status_code: StatusCode,
    pub data: Option<HashMap<String, String>>,
}
