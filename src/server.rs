pub const LOCAL_HOST: &str = "127.0.0.1";

use std::convert::TryFrom;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

enum HttpVerb {
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

impl TryFrom<&str> for HttpVerb {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "get" => Ok(Self::GET),
            "post" => Ok(Self::POST),
            "put" => Ok(Self::PUT),
            "delete" => Ok(Self::DELETE),
            other => Err(format!("Unsupported HTTP verb: {}", other)),
        }
    }
}

pub struct Server {
    port: String,
}

impl Server {
    pub fn new(port: &str) -> Self {
        Self {
            port: port.to_string(),
        }
    }

    pub fn start(&self) {
        let address = format!("{}:{}", LOCAL_HOST, self.port);
        let listener = TcpListener::bind(&address).unwrap();
        log::info!("Server started on {}", address);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            Server::handle_connection(stream);
        }
    }
}

impl Server {
    fn extract_http_verb(line: &str) -> Result<HttpVerb, String> {
        if line.is_empty() {
            return Err("Empty request line".to_string());
        }
        HttpVerb::try_from(line.split(' ').collect::<Vec<&str>>()[0])
    }

    fn get_headers(reader: &mut BufReader<&TcpStream>) -> Result<Vec<String>, String> {
        let mut headers = Vec::new();
        for line in reader.by_ref().lines() {
            match line {
                Ok(line) => {
                    if line.is_empty() {
                        break;
                    }
                    headers.push(line);
                }
                Err(err) => return Err(err.to_string()),
            }
        }

        Ok(headers)
    }

    fn get_body(
        reader: &mut BufReader<&TcpStream>,
        content_length: usize,
    ) -> Result<String, String> {
        let mut body = vec![0; content_length];
        reader.read_exact(&mut body).unwrap();
        Ok(String::from_utf8_lossy(&body).to_string())
    }

    fn parse_http_request(stream: &TcpStream) -> Result<Vec<String>, String> {
        let mut buf_reader = BufReader::new(stream);
        let headers = Server::get_headers(&mut buf_reader)?;

        log::debug!("{headers:#?}");

        let verb = Server::extract_http_verb(&headers[0])?;

        log::debug!("{verb:#?}");

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

        log::debug!("{body:#?}");

        Server::handle_request(verb, body)
    }

    fn handle_request(verb: HttpVerb, body: String) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    fn http_ok_response() -> String {
        "HTTP/1.1 200 OK\r\n\r\n".to_string()
    }

    fn handle_connection(mut stream: TcpStream) {
        let http_request = Server::parse_http_request(&stream);
        log::debug!("Request: {http_request:#?}");
        let response = Server::http_ok_response();
        log::debug!("Response: {response:#?}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}
