use std::path::Path;
use std::net::TcpStream;
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

fn ht_readline(mut stream: &TcpStream) -> String {
    let mut result = String::new();
    loop {
        let mut buf = [0u8];
        let _ = stream.read(&mut buf);
        if buf[0]==13 { break; }
        if buf[0]==10 { continue; }
        result.push(buf[0] as char);
    }
    return result;
}

#[allow(unused_assignments)]
fn get_request_line(stream: &TcpStream) -> RequestLine {
    let line = ht_readline(&stream);
    let v: Vec<&str> = line.split(' ').collect();
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
            let v3: Vec<&str> = v2[1].split('&').collect();
            for kv_pair in v3.iter() {
                let v4: Vec<&str> = kv_pair.split('=').collect();
                if v4.len() > 1 {
                    get_argv.insert(v4[0].to_string(), v4[1].to_string());
                } else {
                    get_argv.insert(v4[0].to_string(), "".to_string());
                }
            }
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

fn get_request_header(stream: &TcpStream) -> RequestHeader {
    let mut request_header = RequestHeader {
        user_agent: "".to_string(),
        host: "".to_string(),
        accept: "".to_string(),
    };
    loop {
        let line = ht_readline(&stream);
        if line.is_empty() { break; }
        let v: Vec<&str> = line.split(' ').collect();
        match v[0] {
            "User-Agent:" => { request_header.user_agent = v[1].to_string(); },
            "Host:" => { request_header.host = v[1].to_string(); },
            "Accept:" => { request_header.accept = v[1].to_string(); },
            _ => {},
        }
    }

    return request_header;
}

fn get_request_info(stream: &TcpStream) -> Request {
    let request_line = get_request_line(&stream);
    let request_header = get_request_header(&stream);
    let remote_addr = get_remote_addr(&stream);
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
        header: request_header,
    }
}

pub fn handle_client(mut stream: TcpStream, router: HashMap<String, fn(Request)->Response>) {
    let mut response = Response::new();
    let request_info = get_request_info(&stream);

    if router.contains_key(&request_info.request_script) {
        response.set_status(200);
        let handler = router.get(&request_info.request_script).unwrap();
        response = handler(request_info);
    } else if request_info.request_script_ext.eq("css") || request_info.request_script_ext.eq("js") {
        let path = Path::new(&request_info.request_uri);
        response = file2response(path);
    } else if request_info.request_script_ext.eq("jpg") || request_info.request_script_ext.eq("png") || request_info.request_script_ext.eq("ico") {
        let path = Path::new(&request_info.request_uri);
        response = file2vec(path);
    } else if router.contains_key("default") {
        response.set_status(200);
        let handler = router.get("default").unwrap();
        response = handler(request_info);
    } else {
        response.set_status(404);
        response.echo("Not Found");
    }

    response.render();
    let _ =  stream.write(&response.response);
}
