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
