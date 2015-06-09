use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::io::prelude::*;


pub fn is_dir(path: &Path) -> bool {
    match fs::metadata(path) {
        Err(_) => return false,
        Ok(path) => return path.is_dir(),
    }
}

pub fn is_file(path: &Path) -> bool {
    match fs::metadata(path) {
        Err(e) => {println!("{}", e); return false},
        Ok(path) => return path.is_file(),
    }
}

pub fn cat(path: &Path) -> Result<String, &str> {
    let mut content = String::new();
    match File::open(path) {
        Err(e) => {
            println!("cat open error: {}", e);
            return Err("file open error");
        },
        Ok(mut fh) => {
            match fh.read_to_string(&mut content) {
                Err(e) => {
                    println!("cat read error: {}", e);
                    return Err("file read error");
                },
                Ok(_) => Ok(content),
            }
        },
    }
}

pub fn binary_cat(path: &Path) -> Result<Vec<u8>, &str> {
    let mut content = Vec::new();
    match File::open(path) {
        Err(e) => {
            println!("{}", e);
            return Err("file open eroor");
        },
        Ok(mut fh) => {
            match fh.read_to_end(&mut content) {
                Err(e) => {
                    println!("{}", e);
                    return Err("file read eroor");
                },
                Ok(_) => Ok(content),
            }
        },
    }
}

pub fn chroot_path(path: &Path, root_dir: &str) -> PathBuf {
    let public_root_path = Path::new(root_dir);
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
