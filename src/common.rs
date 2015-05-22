use std::fs;
use std::fs::File;
use std::path::Path;
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

pub fn cat(path: &Path) -> String {
    let mut content = String::new();
    match File::open(path) {
        Err(_) => return "".to_string(),
        Ok(mut fh) => {
            match fh.read_to_string(&mut content) {
                Err(_) => return "".to_string(),
                Ok(_) => content,
            }
        }
    }
}
