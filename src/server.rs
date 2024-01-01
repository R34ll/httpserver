#![allow(dead_code)]

use std::{
    io::{BufReader, Read},
    net::{TcpListener, TcpStream},
};

use std::io::Write;
use std::str::FromStr;

#[derive(Default, Debug)]
pub enum Method {
    Post,
    #[default]
    Get,
}

impl FromStr for Method {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "POST" => Ok(Method::Post),
            "GET" => Ok(Method::Get),
            _ => Err("Invalid Method"),
        }
    }
}

#[derive(Debug)]
pub enum StatusCode {
    Ok,
    BadRequest,
    NotFound,
    InternalServerError,
}

impl StatusCode {
    pub fn as_u16(&self) -> u16 {
        match self {
            StatusCode::Ok => 200,
            StatusCode::BadRequest => 400,
            StatusCode::NotFound => 404,
            StatusCode::InternalServerError => 500,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::NotFound => "Not Found",
            StatusCode::InternalServerError => "Internal Server Error",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ContentType {
    TextPlain,
    TextHtml,
}

impl ContentType {
    fn as_str(&self) -> &str {
        match self {
            ContentType::TextPlain => "text/plain",
            ContentType::TextHtml => "text/html",
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub enum HttpVersion {
    Http1_0,
    #[default]
    Http1_1,
    Http2_0,
}

impl HttpVersion {
    fn as_str(&self) -> &str {
        match self {
            HttpVersion::Http1_0 => "HTTP/1.0",
            HttpVersion::Http1_1 => "HTTP/1.1",
            HttpVersion::Http2_0 => "HTTP/2.0",
        }
    }
}

impl FromStr for HttpVersion {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "HTTP/1.0" => Ok(HttpVersion::Http1_0),
            "HTTP/1.1" => Ok(HttpVersion::Http1_1),
            "HTTP/2.0" => Ok(HttpVersion::Http2_0),
            _ => Err("Invalid Version"),
        }
    }
}

#[derive(Debug)]
pub struct Request {
    method: Method,
    version: HttpVersion,
    stream: TcpStream,
    host: String,
    user_agent: String,
    pub path: String,
    body: String,
}

impl Request {
    pub fn from(mut stream: TcpStream) -> Self {
        let mut reader = BufReader::new(&mut stream);
        let mut buffer = [0; 1024];

        let full_request = if reader.read(&mut buffer).is_ok() {
            String::from_utf8_lossy(&buffer).to_string()
        } else {
            String::new()
        };

        let cloned_stream = stream.try_clone().expect("Failed to clone stream");

        let lines: Vec<&str> = full_request.lines().collect();
        let components: Vec<&str> = lines.first().unwrap().split_whitespace().collect();

        let mut req = Request {
            method: Method::default(),
            version: HttpVersion::Http2_0,
            stream: cloned_stream,
            host: String::new(),
            user_agent: String::new(),
            path: String::new(),
            body: String::new(),
        };

        if lines.len() > 2 {
            req.host = lines
                .get(1)
                .unwrap()
                .split_whitespace()
                .collect::<Vec<&str>>()[1]
                .to_string();
            req.user_agent = lines
                .get(2)
                .unwrap()
                .split_whitespace()
                .collect::<Vec<&str>>()[1]
                .to_string();
            req.method = components[0].parse().unwrap();
            req.path = components[1].to_string();
            req.version = components[2].parse().unwrap();
        }

        req
    }

    pub fn response(&mut self, response: Response) {
        self.stream
            .write_all(response.generate_http_response().as_bytes())
            .expect("Error while responding");
    }
}

#[derive(Debug)]
pub struct Response {
    version: HttpVersion,
    content_type: ContentType,
    status_code: StatusCode,
    content_length: usize,
    body: String,
}

impl Response {
    pub fn new(
        version: HttpVersion,
        content_type: ContentType,
        status_code: StatusCode,
        body: &str,
    ) -> Response {
        Response {
            version,
            content_type,
            status_code,
            content_length: body.len(),
            body: body.to_string(),
        }
    }

    fn generate_http_response(&self) -> String {
        let status_line = format!(
            "{} {} {}\r\n",
            self.version.as_str(),
            self.status_code.as_u16(),
            self.status_code.as_str()
        );
        let content_type = format!("Content-Type: {}\r\n", self.content_type.as_str());
        let content_length = format!("Content-Length: {}\r\n\r\n", self.content_length);

        format!(
            "{}{}{}{}",
            status_line, content_type, content_length, self.body
        )
    }
}
