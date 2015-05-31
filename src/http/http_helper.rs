use std::path::Path;
use response::Response;
use common::*;

pub fn file2response(path: &Path) -> Response {
    let mut resp = Response::new();
    let path_buf = chroot_path(path, "public");
    let file_content = cat(path_buf.as_path());
    match file_content {
        Ok(file_content) => {
            resp.echo(&file_content);
            return resp; 
        },
        Err(_) => {
            resp.status(404);
            resp.echo("404");
            return resp; 
        },
    }
}

pub fn file2vec(path: &Path) -> Vec<u8> {
    let path_buf = chroot_path(path, "public");
    let file_content = binary_cat(path_buf.as_path());
    match file_content {
        Ok(file_content) => {
            return file_content; 
        },
        Err(_) => {
            return Vec::new(); 
        },
    }
}
