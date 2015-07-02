extern crate url;
use self::url::form_urlencoded;

use std::str;
use std::path::Path;
use std::net::TcpStream;
use std::net::Shutdown;
use std::io::prelude::*;
use std::collections::HashMap;

mod helper;
use http::helper::*;

pub mod status;

pub mod request;
pub mod response;
use http::request::{RemoteAddr, RequestLine, RequestHeader, Request};
use http::response::Response;

fn get_remote_addr(stream: &TcpStream) -> RemoteAddr {
    let peer = stream.peer_addr().unwrap().to_string();
    let v: Vec<&str> = peer.split(':').collect();

    let peer_ip = v[0].to_string();
    let peer_port = v[1].parse::<u16>().ok().expect("fail parse port to i32");

    RemoteAddr {ip: peer_ip, port: peer_port}
}

fn parse_query_string(query_string: &str) -> HashMap<String, String> {
    let mut get_argv = HashMap::new();
    let query_vec = form_urlencoded::parse(query_string.as_bytes());
    for (k, v) in query_vec.into_iter() {
        get_argv.insert(k, v);
    }
    get_argv
}

#[allow(unused_assignments)]
fn parse_request_line(request_line: &str) -> RequestLine {
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

fn parse_request_header(head_lines: Vec<&str>) -> RequestHeader {
    let mut request_header = RequestHeader::new();
    for line in head_lines {
        if line.is_empty() { break; }
        let v: Vec<&str> = line.split(' ').collect();
        println!("[{:?}]", v);
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

#[allow(unused_assignments)]
fn get_request_info(mut stream: &TcpStream) -> Request {
    let remote_addr = get_remote_addr(&stream);

    let mut buf = [0u8; 4096];
    let read_byte = stream.read(&mut buf).unwrap();
    let req = str::from_utf8(&buf[0..read_byte]).unwrap().to_string();

    let blank_line_raw = &[13, 10, 13, 10];
    let blank_line = str::from_utf8(blank_line_raw).unwrap();
    let head_and_body: Vec<&str> = req.splitn(2, blank_line).collect();

    let head = head_and_body[0];
    let mut head_lines: Vec<&str> = head.lines().collect(); 
    let request_line = parse_request_line(head_lines.remove(0));
    let request_header = parse_request_header(head_lines);

    let mut body = head_and_body[1].to_string();
    let mut post_argv = HashMap::new();
    if request_header.content_type.eq("application/x-www-form-urlencoded") {
        let mut body_unread_byte = request_header.content_length - body.len();
        while body_unread_byte > 0 {
            let mut buf = [0u8; 4096];
            let read_byte = stream.read(&mut buf).unwrap();
            let buf_str = str::from_utf8(&buf).unwrap();
            body.push_str(buf_str);
            body_unread_byte = body_unread_byte - read_byte;
        }
        post_argv = parse_query_string(&body);
    }

    Request {
        remote_ip: remote_addr.ip,
        remote_port: remote_addr.port,
        method: request_line.method,
        request_uri: request_line.request_uri,
        request_script: request_line.request_script,
        request_script_ext: request_line.request_script_ext,
        query_string: request_line.query_string,
        protocol_version: request_line.protocol_version,
        get_argv: request_line.get_argv,
        post_argv: post_argv,
        header: request_header,
        body: body.bytes().collect(),
    }
}

pub fn handle_client(mut stream: TcpStream, router: HashMap<String, fn(Request)->Response>) {
    let mut response = Response::new(200);
    let request_info = get_request_info(&stream);

    if router.contains_key(&request_info.request_script) {
        let handler = router.get(&request_info.request_script).unwrap();
        response = handler(request_info);
    } else if request_info.request_script_ext.eq("css") || request_info.request_script_ext.eq("js") || request_info.request_script_ext.eq("jpg") 
        || request_info.request_script_ext.eq("png") || request_info.request_script_ext.eq("ico") || request_info.request_script_ext.eq("cur") {
        let path = Path::new(&request_info.request_uri);
        response = file2response(path);
    } else if router.contains_key("default") {
        let handler = router.get("default").unwrap();
        response = handler(request_info);
    } else {
        response.set_status(404);
        response.echo("Not Found");
    }

    response.render();
    let _ = stream.write(&response.response);
    let _ = stream.shutdown(Shutdown::Both);
}
