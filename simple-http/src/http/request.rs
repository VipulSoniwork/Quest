use std:: {collections:: HashMap, fmt:: Display, io, str:: FromStr};
use std::result::Result;
use std::fmt;
use super::response::HttpResponse;

#[derive(Debug)]
pub struct HttpRequest {
    method: Method,
   pub resource: Resource,
    version: Version,
    headers: HttpHeader,
    pub request_body: String, // Fixed the field name
}


 // Import Result type if necessary
 impl HttpRequest {
    pub fn response(&self) -> io::Result<HttpResponse> {
        HttpResponse::new(self)
    }
     pub fn new(request: &str) -> io::Result<HttpRequest> {
         // Parse the method
         let method = Method::new(request);
 
         // Parse the resource
         let resource = if let Some(resource) = Resource::new(request) {
             resource
         } else {
             Resource { path: "".to_string() }
         };
 
         // Parse the version, map errors to io::Error
         let version = Version::new(request).map_err(|err| {
             io::Error::new(io::ErrorKind::InvalidData, err.msg)
         })?;
 
         // Parse the headers
         let headers = if let Some(headers) = HttpHeader::new(request) {
             headers
         } else {
             HttpHeader { headers: HashMap::new() }
         };
 
         // Parse the request body
         let request_body = if let Some((_header, body)) = request.split_once("\r\n\r\n") {
             body.to_string()
         } else {
             String::new()
         };
 
         // Construct and return the HttpRequest
         Ok(HttpRequest {
             method,
             resource,
             version,
             headers,
             request_body,
         })
     }
 }
 


#[derive(Debug)]
pub struct HttpHeader {
    headers: HashMap<String, String>, // Corrected HashMap field
}

impl HttpHeader {
    pub fn new(request: &str) -> Option<HttpHeader> {
        let mut httpheader = HttpHeader {
            headers: HashMap::new(),
        };

        // Split request at the first occurrence of "\r\n\r\n" to separate headers
        let (_, header_str) = request.split_once("\r\n\r\n")?;

        // Iterate over each line in the header section
        for line in header_str.split_terminator("\r\n") {
            if line.is_empty() {
                break;
            }

            // Split each line into a header and a value
            let (header, value) = line.split_once(":")?;
            httpheader.headers.insert(header.trim().to_string(), value.trim().to_string());
        }

        Some(httpheader)
    }
}


#[derive(Debug)]
pub enum Version {
    V1_1,
    V2_0,
}

impl Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Version::V1_1 => "HTTP/1.1",
            Version::V2_0 => "HTTP/2",
        };
        write!(f, "{}", msg) // Fixed syntax for `write!`
    }
}


#[derive(Debug)]
pub struct VersionError {
    msg: String, // Error message field
}

impl Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg) // Write the error message to the formatter
    }
}

impl Version {
    pub fn new(request: &str) -> Result<Self, VersionError> {
        Version::from_str(request) // Call FromStr implementation to parse version
    }
}

impl FromStr for Version {
    type Err = VersionError; // Specify the associated error type

    fn from_str(request: &str) -> Result<Self, Self::Err> {
        // Split the request at the first occurrence of "\r\n" to separate request line
        if let Some((method_line, _rest)) = request.split_once("\r\n") {
            let method_line_parts = method_line.split_ascii_whitespace(); // Split by whitespace

            for part in method_line_parts {
                match part {
                    "HTTP/1.1" => return Ok(Version::V1_1),
                    "HTTP/2" | "HTTP/2.0" => return Ok(Version::V2_0),
                    _ => continue, // Ignore non-version parts of the request line
                }
            }
        }

        // If no valid version is found, return an error
        let invalid = format!("Unknown protocol version in {}", request);
        let version_error = VersionError { msg: invalid };
        Err(version_error)
    }
}

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Uninitialized, // Correct spelling of Uninitialized
}

impl Method {
    pub fn new(request: &str) -> Method {
        // Split the request at the first occurrence of "\r\n" to separate the request line
        if let Some((method_line, _rest)) = request.split_once("\r\n") {
            // Split the method line into method and the rest of the request
            if let Some((method, _rest)) = method_line.split_once(' ') {
                return match method {
                    "GET" => Method::Get,
                    "POST" => Method::Post,
                    _ => Method::Uninitialized, // Handle unknown methods
                };
            }
        }
        Method::Uninitialized // Default to Uninitialized if parsing fails
    }
    pub fn identify(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized, // Use `_` as a wildcard to match anything else
        }
    }
    
}

#[derive(Debug)]
pub struct Resource {
    pub path: String, // Proper field definition
}

impl Resource {
    pub fn new(request: &str) -> Option<Resource> {
        if let Some((request_line, _)) = request.split_once("\r\n") {
            if let Some((method, rest)) = request_line.split_once(' ') {
                return match Method::identify(method) {
                    Method::Get | Method::Post => {
                        if let Some((resource, _protocol_version)) = rest.split_once(' ') {
                            let resource = resource.trim().trim_start_matches('/');
                            return Some(Resource {
                                path: resource.to_string(),
                            });
                        }
                        None // Return None if splitting resource and protocol fails
                    }
                    Method::Uninitialized => None,
                };
            }
        }
        None
    }
}
