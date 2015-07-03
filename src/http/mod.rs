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
use router::Router;

fn get_remote_addr(stream: &TcpStream) -> RemoteAddr {
    let peer = stream.peer_addr().unwrap().to_string();
    let v: Vec<&str> = peer.split(':').collect();

    let peer_ip = v[0].to_string();
    let peer_port = v[1].parse::<u16>().ok().expect("fail parse port to i32");

    RemoteAddr {ip: peer_ip, port: peer_port}
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
    let request_line = RequestLine::from_request_line(head_lines.remove(0));
    let request_header = RequestHeader::from_head_lines(head_lines);

    let mut body = head_and_body[1].to_string();
    let mut post_argv = HashMap::new();
    if request_header.content_type.contains("application/x-www-form-urlencoded") {
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

pub fn handle_client(mut stream: TcpStream, router: Router) {
    let mut response = Response::new(200);
    let request_info = get_request_info(&stream);

    if request_info.request_script_ext.eq("css") || request_info.request_script_ext.eq("js") || request_info.request_script_ext.eq("jpg") 
        || request_info.request_script_ext.eq("png") || request_info.request_script_ext.eq("ico") || request_info.request_script_ext.eq("cur") {
        let path = Path::new(&request_info.request_uri);
        response = file2response(path);
    } else if request_info.method.eq("GET") { 
        if router.routers_get.contains_key(&request_info.request_script) {
            let handler = router.routers_get.get(&request_info.request_script).unwrap();
            response = handler(request_info);
        } else if router.routers_get.contains_key("default") {
            let handler = router.routers_get.get("default").unwrap();
            response = handler(request_info);
        } else {
            response.set_status(404);
            response.echo("Not Found");
        }
    } else if request_info.method.eq("POST") { 
        if router.routers_post.contains_key(&request_info.request_script) {
            let handler = router.routers_post.get(&request_info.request_script).unwrap();
            response = handler(request_info);
        } else if router.routers_post.contains_key("default") {
            let handler = router.routers_post.get("default").unwrap();
            response = handler(request_info);
        } else {
            response.set_status(404);
            response.echo("Not Found");
        }
    } else {
        response.set_status(405);
        response.echo("Method Not Allowed");
    }

    response.render();
    let _ = stream.write(&response.response);
    let _ = stream.shutdown(Shutdown::Both);
}
