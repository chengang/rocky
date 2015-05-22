use std::collections::HashMap;
use std::fs;
use std::path::Path;
pub use common::*;

enum ParseStatus {
    Out,
    In,
    PrefixMatchOne,
    SuffixMatchOne,
}

// todo : 1. {} in var, 2.err when not ParseStatus::Out in the end.
#[allow(unused_variables)]
fn file_to_tokens(path: &Path) -> Vec<String> {
    let mut token = String::new();
    let mut tokens = Vec::new();
    let mut parse_status = ParseStatus::Out;

    let characters: Vec<(usize, char)> = cat(path).char_indices().collect();
    for character in characters {
        let (unused_pos, utf8_char) = character;
        match parse_status {
            ParseStatus::Out => {
                if utf8_char == '{' {
                    parse_status = ParseStatus::PrefixMatchOne;
                } else {
                    token.push(utf8_char);
                }
            },
            ParseStatus::In => { 
                if utf8_char == '}' {
                    parse_status = ParseStatus::SuffixMatchOne;
                } else if utf8_char != ' ' {
                    token.push(utf8_char);
                }
            },
            ParseStatus::PrefixMatchOne => {
                if utf8_char == '{' {
                    parse_status = ParseStatus::In;
                    tokens.push(token.clone());
                    token = String::new();
                } else {
                    parse_status = ParseStatus::Out;
                    token.push('{'); 
                    token.push(utf8_char); 
                }
            },
            ParseStatus::SuffixMatchOne => {
                if utf8_char == '}' {
                    parse_status = ParseStatus::Out;
                    tokens.push(token.clone());
                    token = String::new();
                } else {
                    parse_status = ParseStatus::Out;
                    token.push('}');
                    token.push(utf8_char);
                }
            },
        }
    }
    tokens.push(token);
    return tokens;
}

pub struct Template {
    pub templates: HashMap<String, String>,
}

impl Template{
    pub fn new(path: &str) -> Template {
        let mut templates: HashMap<String, String> = HashMap::new();
        let path = Path::new(path);
        if is_dir(path) {
            match fs::read_dir(path) {
                Err(why) => println!("! {:?}", why.kind()),
                Ok(paths) => for entry in paths {
                    let mut template_name = String::new();
                    let mut template_content = String::new();
                    match entry {
                        Err(_) => {},
                        Ok(dir_entry) => {
                            let dir_entry_path = dir_entry.path();
                            let template_path = dir_entry_path.as_path().clone();
                            template_name =
                                dir_entry_path.file_name().unwrap().to_os_string().into_string().unwrap();
                            if is_file(template_path) {
                                let tokens = file_to_tokens(template_path);
                                for x in tokens.iter() {
                                        template_content.push_str(&x);
                                }
                            } 
                        },
                    }
                    templates.insert(template_name, template_content);
                },
            }
        }
        Template { templates: templates }
    }
}
