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
        let version = Version::V1_1;
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
                content_length = content.len();
                status = ResponseStatus::OK;
                accept_ranges = AcceptRanges::Bytes;

                // Create the response and then append the content at the end
                response_body = format!(
                    "{} {}\n{}\ncontent-length: {}\r\n\r\n{}",
                    version,
                    status,
                    accept_ranges,
                    content_length,
                    content // Now adding the content properly after the headers
                );
            }
        } else {
            // Handle the case where the file does not exist or is not a file
            response_body = format!(
                "{} {}\n{}\ncontent-length: {}\r\n\r\n{}",
                version,
                status,
                accept_ranges,
                content_length,
                response_body // This could be an error message or an empty body
            );
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

