use std::collections::HashMap;

#[derive(Debug)]
pub struct RemoteAddr {
    pub ip: String,
    pub port: u16,
}

#[derive(Debug)]
pub struct RequestLine {
    pub method: String,
    pub request_uri: String,
    pub protocol_version: String,
    pub request_script: String,
    pub request_script_ext: String,
    pub query_string: String,
    pub get_argv: HashMap<String, String>,
}

#[derive(Debug)]
pub struct RequestHeader {
    pub user_agent: String,
    pub host: String,
    pub accept: String,
}

#[derive(Debug)]
pub struct Request {
    pub remote_ip: String,
    pub remote_port: u16,
    pub method: String,
    pub request_uri: String,
    pub protocol_version: String,
    pub request_script: String,
    pub request_script_ext: String,
    pub query_string: String,
    pub get_argv: HashMap<String, String>,
    pub header: RequestHeader,
}

impl RequestLine {
    pub fn new() -> RequestLine {
        RequestLine {
            method: "".to_string(),
            request_uri: "".to_string(),
            request_script: "".to_string(),
            request_script_ext: "".to_string(),
            query_string: "".to_string(),
            protocol_version: "".to_string(),
            get_argv: HashMap::new(),
        }
    }
}

impl RequestHeader {
    pub fn new() -> RequestHeader {
        RequestHeader {
            user_agent: "".to_string(),
            host: "".to_string(),
            accept: "".to_string(),
        }
    }
}
