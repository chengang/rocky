use std::collections::HashMap;

pub struct RemoteAddr {
    pub ip: String,
    pub port: u16,
}

pub struct RequestLine {
    pub method: String,
    pub request_uri: String,
    pub protocol_version: String,
    pub request_script: String,
    pub query_string: String,
    pub get_argv: HashMap<String, String>,
}

pub struct RequestHeader {
    pub user_agent: String,
    pub host: String,
    pub accept: String,
}

pub struct Request {
    pub remote_ip: String,
    pub remote_port: u16,
    pub method: String,
    pub request_uri: String,
    pub protocol_version: String,
    pub request_script: String,
    pub query_string: String,
    pub get_argv: HashMap<String, String>,
    pub header: RequestHeader,
}

impl Request {
}
