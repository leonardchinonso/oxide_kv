pub const LOCAL_HOST: &str = "127.0.0.1";

use std::collections::HashMap;

use std::convert::TryFrom;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

use crate::database::Database;
use crate::error::OxideKvError;
use crate::model::{HttpRequest, HttpResponse, HttpVerb, StatusCode};

impl TryFrom<&str> for HttpVerb {
    type Error = OxideKvError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "get" => Ok(Self::GET),
            "post" => Ok(Self::POST),
            "put" => Ok(Self::PUT),
            "delete" => Ok(Self::DELETE),
            other => Err(OxideKvError::Server(format!(
                "Unsupported HTTP verb: {}",
                other
            ))),
        }
    }
}

pub struct Server {
    port: String,
    service: Service,
}

impl Server {
    pub fn new(db: Database, port: &str) -> Self {
        Self {
            port: port.to_string(),
            service: Service::new(db),
        }
    }

    pub fn start(&mut self) -> Result<(), OxideKvError> {
        let address = format!("{}:{}", LOCAL_HOST, self.port);
        let listener = TcpListener::bind(&address).unwrap();
        log::info!("Server started on {}", address);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_connection(stream)?;
        }

        Ok(())
    }

    fn handle_request(&mut self, request: HttpRequest) -> Result<HttpResponse, OxideKvError> {
        log::info!("Handling request...");
        log::info!("{request:#?}");

        match request.verb {
            Some(HttpVerb::POST) => {
                self.service.create_entry(request);
                Ok(HttpResponse {
                    message: "Key value created successfully".to_string(),
                    status_code: StatusCode::OK,
                    data: None,
                })
            }
            Some(HttpVerb::GET) => {
                let key = request.key.clone();
                let v = self.service.get_entry(request)?;
                Ok(HttpResponse {
                    message: "Key value retrieved successfully".to_string(),
                    status_code: StatusCode::OK,
                    data: Some(HashMap::from([(key, v)])),
                })
            }
            Some(HttpVerb::PUT) => {
                self.service.update_entry(request);
                Ok(HttpResponse {
                    message: "Key value updated successfully".to_string(),
                    status_code: StatusCode::OK,
                    data: None,
                })
            }
            Some(HttpVerb::DELETE) => {
                self.service.delete_entry(request)?;
                Ok(HttpResponse {
                    message: "Key value retrieved successfully".to_string(),
                    status_code: StatusCode::OK,
                    data: None,
                })
            }
            None => Err(OxideKvError::Server(
                "Http Verb is missing for this request".to_string(),
            )),
        }
    }

    fn handle_connection(&mut self, mut stream: TcpStream) -> Result<(), OxideKvError> {
        let http_request = Server::parse_http_request(&stream)?;
        log::debug!("Request: {http_request:#?}");

        let response = self.handle_request(http_request)?;
        log::debug!("Response: {response:#?}");

        match serde_json::to_string(&response) {
            Ok(json) => stream
                .write_all(json.as_bytes())
                .map_err(|e| OxideKvError::Server(e.to_string())),
            Err(e) => Err(OxideKvError::Server(e.to_string())),
        }
    }
}

impl Server {
    fn extract_http_verb(line: &str) -> Result<HttpVerb, OxideKvError> {
        if line.is_empty() {
            return Err(OxideKvError::Server("Empty request line".to_string()));
        }
        HttpVerb::try_from(line.split(' ').collect::<Vec<&str>>()[0])
    }

    fn get_headers(reader: &mut BufReader<&TcpStream>) -> Result<Vec<String>, OxideKvError> {
        let mut headers = Vec::new();
        for line in reader.by_ref().lines() {
            match line {
                Ok(line) => {
                    if line.is_empty() {
                        break;
                    }
                    headers.push(line);
                }
                Err(err) => return Err(OxideKvError::Server(err.to_string())),
            }
        }

        Ok(headers)
    }

    fn get_body(
        reader: &mut BufReader<&TcpStream>,
        content_length: usize,
    ) -> Result<String, OxideKvError> {
        let mut body = vec![0; content_length];
        reader
            .read_exact(&mut body)
            .map_err(|e| OxideKvError::Server(e.to_string()))?;
        Ok(String::from_utf8_lossy(&body).to_string())
    }

    fn parse_http_request(stream: &TcpStream) -> Result<HttpRequest, OxideKvError> {
        let mut buf_reader = BufReader::new(stream);
        let headers = Server::get_headers(&mut buf_reader)?;
        let verb = Server::extract_http_verb(&headers[0])?;

        let content_length = headers
            .iter()
            .find_map(|h| {
                let prefix = h.strip_prefix("Content-Length: ");
                match prefix {
                    Some(prefix) => prefix.trim().parse::<usize>().ok(),
                    None => None,
                }
            })
            .unwrap_or(0);
        let body = Server::get_body(&mut buf_reader, content_length)?;

        Ok(HttpRequest::try_from(body.as_str())?.set_verb(verb))
    }
}

struct Service {
    db: Database,
}

impl Service {
    fn new(db: Database) -> Self {
        return Service { db: db };
    }

    fn create_entry(&mut self, request: HttpRequest) {
        self.db.upsert(request.key, request.value);
    }

    fn get_entry(&self, request: HttpRequest) -> Result<String, OxideKvError> {
        self.db.get(request.key)
    }

    fn update_entry(&mut self, request: HttpRequest) {
        self.db.upsert(request.key, request.value);
    }

    fn delete_entry(&mut self, request: HttpRequest) -> Result<(), OxideKvError> {
        self.db.remove(request.key)
    }
}
