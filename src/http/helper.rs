extern crate url;
use self::url::form_urlencoded;

use std::collections::HashMap;
use std::path::Path;
use http::response::Response;
use common::*;

pub fn file2response(path: &Path) -> Response {
    let mut resp = Response::new(200);
    let path_buf = chroot_path(path, "public");
    let file_content = binary_cat(path_buf.as_path());
    match file_content {
        Ok(file_content) => {
            resp.body = file_content;
            return resp; 
        },
        Err(_) => {
            resp.set_status(404);
            resp.echo("File Not Found");
            return resp; 
        },
    }
}

pub fn parse_query_string(query_string: &str) -> HashMap<String, String> {
    let mut get_argv = HashMap::new();
    let query_vec = form_urlencoded::parse(query_string.as_bytes());
    for (k, v) in query_vec.into_iter() {
        get_argv.insert(k, v);
    }
    get_argv
}
