use std::path::Path;
use std::path::PathBuf;
use response::Response;
use common::*;

pub fn file2response(path: &Path) -> Response {
    let mut resp = Response::new();
    let file_content = cat(chroot_path(path).as_path());
    resp.echo(&file_content);
    return resp;
}

pub fn file2vec(path: &Path) -> Vec<u8> {
    let resp = binary_cat(chroot_path(path).as_path());
    return resp;
}

fn chroot_path(path: &Path) -> PathBuf {
    let public_root_path = Path::new("public");
    let mut path_buf = public_root_path.to_path_buf();
    let mut is_first_component = true;
    for component in path.iter() {
        if is_first_component {
            is_first_component = false;
            continue;
        }
        path_buf.push(component);
    }
    let path = path_buf.as_path().to_owned();
    return path;
}
