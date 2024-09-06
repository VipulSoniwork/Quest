use std::fmt::Display;
use std::fs;
use std::io;
use std::path::{PathBuf, Path};
use super::request::{HttpRequest, Version}; // Correctly importing the modules
use std::fmt;

#[derive(Debug)]
pub struct HttpResponse {
    version: Version,
    status: ResponseStatus,
    content_length: usize,
    accept_ranges: AcceptRanges,
    pub response_body: String,
    pub current_path: String,
}

impl HttpResponse {
    pub fn new(request: &HttpRequest) -> io::Result<HttpResponse> {
        let version = Version::V2_0;
        let mut status = ResponseStatus::NotFound;
        let mut content_length = 0;
        let mut accept_ranges = AcceptRanges::None;
        let current_path = request.resource.path.clone();
        let mut response_body = String::new();

        let server_root_path = std::env::current_dir()?;
        let resource = request.resource.path.clone();
        let new_path = server_root_path.join(&resource);

        if new_path.exists() {
            if new_path.is_file() {
                let content = fs::read_to_string(&new_path)?;
                response_body.push_str(&content);
                content_length = content.len();
                status = ResponseStatus::OK;
                accept_ranges = AcceptRanges::Bytes;
                let content = format!(
                    "{} {} {}\nContent-Length: {}\r\n\r\n{}",
                    version,
                    status,
                    accept_ranges,
                    content_length,
                    response_body
                );
            } else {
                // Handle directory or other types of resources
                let four="<html>\
                    <body>\
                    <h1>404 NOT FOUND</h1>\
                    </body>\
                    </html>";
                content_length= four.len();
                let content = format!(
                    "{} {} Accept-Ranges: {}\nContent-Length: {}\r\n\r\n{}
",
                    version,
                    status,
                    accept_ranges,
                    content_length,
                    four,
                );
                response_body.push_str(&content);
            }
        } else {
            // Handle the case where the resource does not exist
            let content = format!(
                "{} {} Accept-Ranges: {}\nContent-Length: {}\r\n\r\n\
                <html>\
                <body>\
                <h1>404 NOT FOUND</h1>\
                </body>\
                </html>",
                version,
                status,
                accept_ranges,
                content_length
            );
            response_body.push_str(&content);
        }

        Ok(HttpResponse {
            version,
            status,
            content_length,
            accept_ranges,
            response_body,
            current_path,
        })
    }
}


#[derive(Debug)]
pub enum ResponseStatus {
    OK = 200,
    NotFound = 404,
}

impl Display for ResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            ResponseStatus::OK => "200 OK",
            ResponseStatus::NotFound => "404 NOT FOUND",
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug)]
pub enum AcceptRanges {
    Bytes,
    None,
}

impl Display for AcceptRanges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            AcceptRanges::Bytes => "accept-ranges: bytes",
            AcceptRanges::None => "accept-ranges: none",
        };
        write!(f, "{}", msg)
    }
}

