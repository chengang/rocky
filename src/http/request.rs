use std::collections::HashMap;
use http::helper::*;

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
    pub content_length: usize,
    pub content_type: String,
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
    pub post_argv: HashMap<String, String>,
    pub header: RequestHeader,
    pub body: Vec<u8>,
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

    #[allow(unused_assignments)]
    pub fn from_request_line(request_line: &str) -> RequestLine {
        let v: Vec<&str> = request_line.split(' ').collect();
        let method = v[0].to_string();
        let request_uri = v[1].to_string();
        let protocol_version = v[2].to_string();

        let mut request_script = String::new();
        let mut request_script_ext = String::new();
        let mut query_string = String::new();
        let mut get_argv = HashMap::new();
        {
            let v2: Vec<&str> = request_uri.split('?').collect();
            request_script = v2[0].to_string();
            {
                let v3: Vec<&str> = request_script.rsplit_terminator('.').collect();
                request_script_ext = v3[0].to_string();
            }
            if v2.len() > 1 {
                query_string = v2[1].to_string();
                get_argv = parse_query_string(&query_string);
            }
        }

        RequestLine {
            method: method,
            request_uri: request_uri,
            protocol_version: protocol_version,
            request_script: request_script,
            request_script_ext: request_script_ext,
            query_string: query_string,
            get_argv: get_argv,
        }
    }
}

impl RequestHeader {
    pub fn new() -> RequestHeader {
        RequestHeader {
            user_agent: "".to_string(),
            host: "".to_string(),
            accept: "".to_string(),
            content_length: 0,
            content_type: "".to_string(),
        }
    }

    pub fn from_head_lines(head_lines: Vec<&str>) -> RequestHeader {
        let mut request_header = RequestHeader::new();
        for line in head_lines {
            if line.is_empty() { break; }
            let v: Vec<&str> = line.split(' ').collect();
            match v[0] {
                "User-Agent:" => { request_header.user_agent = v[1].trim_right().to_string(); },
                "Host:" => { request_header.host = v[1].trim_right().to_string(); },
                "Accept:" => { request_header.accept = v[1].trim_right().to_string(); },
                "Content-Length:" => { request_header.content_length = v[1].trim_right().parse::<usize>().unwrap(); },
                "Content-Type:" => { request_header.content_type = v[1].trim_right().to_string(); },
                _ => {},
            }
        }
        request_header
    }
}
