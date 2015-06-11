use std::collections::HashMap;
use std::path::Path;
use common::*;

enum ParseStatus {
    Out,
    In,
    PrefixMatchOne,
    SuffixMatchOne,
}

enum RenderStatusForeach {
    In,
    Out,
}

enum RenderStatusIf {
    In,
    Out,
}

#[derive(Debug)]
pub enum TokenType {
    Html,
    Var,
    Foreach,
    ForeachVar,
    ForeachClose,
    If,
    IfClose,
}

#[derive(Debug)]
pub struct Token {
    t: TokenType,
    v: String,
}

pub struct Template {
    pub dir: String,
    pub name: String,
    pub suffix: String,
    pub tokens: Vec<Token>,
    pub vars: HashMap<String, String>,
    pub foreach_vars: HashMap<String, Vec<HashMap<String, String>>>,
    pub if_vars: HashMap<String, bool>,
}

// todo : err when not ParseStatus::Out in the end.
fn file_to_tokens(path: &Path) -> Vec<Token> {
    let mut token = String::new();
    let mut tokens = Vec::new();
    let mut parse_status = ParseStatus::Out;

    let mut template_content;
    let file_content = cat(path);
    match file_content {
        Ok(file_content) => {
            template_content = file_content;
        },
        Err(_) => {
            return tokens;
        },
    }
    let characters: Vec<(usize, char)> = template_content.char_indices().collect();
    for character in characters {
        let (_, utf8_char) = character;
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
                } else {
                    token.push(utf8_char);
                }
            },
            ParseStatus::PrefixMatchOne => {
                if utf8_char == '{' {
                    parse_status = ParseStatus::In;
                    tokens.push(Token {t: TokenType::Html, v: token});
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
                    {
                        let words: Vec<&str> = token.trim().split(' ').collect();
                        if words.len() == 1 {
                            if words[0] == "endforeach" {
                                tokens.push(Token {t: TokenType::ForeachClose, v: "".to_string()} );
                            } else if words[0] == "endif" {
                                tokens.push(Token {t: TokenType::IfClose, v: "".to_string()} );
                            } else if words[0].starts_with(".") {
                                tokens.push(Token {t: TokenType::ForeachVar, v: words[0].trim_left_matches('.').to_string()} );
                            } else {
                                tokens.push(Token {t: TokenType::Var, v: words[0].to_string()} );
                            }
                        } else if words[0] == "foreach" {
                            tokens.push(Token {t: TokenType::Foreach, v: words[1].to_string()} );
                        } else if words[0] == "if" {
                            tokens.push(Token {t: TokenType::If, v: words[1].to_string()} );
                        }
                    }
                    token = String::new();
                } else {
                    parse_status = ParseStatus::Out;
                    token.push('}');
                    token.push(utf8_char);
                }
            },
        }
    }
    tokens.push(Token {t: TokenType::Html, v: token} );
    return tokens;
}

impl Template {
    pub fn new() -> Template {
        Template { 
            dir: "template".to_string(),
            name: String::new(),
            suffix: "html".to_string(),
            tokens: Vec::new(), 
            vars: HashMap::new(),
            foreach_vars: HashMap::new(),
            if_vars: HashMap::new(),
        }
    }

    pub fn set_template(&mut self, path: &str) {
        self.name = path.to_string();
        let mut path_string = String::new();
        path_string.push_str(&self.dir);
        path_string.push_str("/");
        path_string.push_str(path);
        path_string.push_str(".");
        path_string.push_str(&self.suffix);
        self.tokens = file_to_tokens(Path::new(&path_string));
    }

    pub fn assign(&mut self, var: &str, data: String) {
        self.vars.insert(var.to_string(), data);
    }

    pub fn assign_array(&mut self, var: &str, data: Vec<HashMap<String, String>>) {
        self.foreach_vars.insert(var.to_string(), data);
    }

    pub fn assign_bool(&mut self, var: &str, data: bool) {
        self.if_vars.insert(var.to_string(), data);
    }

    pub fn render(&mut self) -> String {
        let mut template_content = String::new();
        let mut token_stack = Vec::new(); // for 'foreach' cmd
        let mut render_status_foreach = RenderStatusForeach::Out;
        let mut render_status_if = RenderStatusIf::Out;
        let mut if_var = String::new();

        for token in self.tokens.iter() {
            match token.t {
                TokenType::Html => {
                    match render_status_foreach {
                        RenderStatusForeach::In => { 
                            match render_status_if {
                                RenderStatusIf::In => { 
                                    if *self.if_vars.get(&if_var).unwrap() { token_stack.push(token) };
                                },
                                RenderStatusIf::Out => { token_stack.push(token) },
                            }
                        },
                        RenderStatusForeach::Out => { 
                            match render_status_if {
                                RenderStatusIf::In => { 
                                    if *self.if_vars.get(&if_var).unwrap() { template_content.push_str(&token.v) };
                                },
                                RenderStatusIf::Out => { template_content.push_str(&token.v) },
                            }
                        },
                    }
                },
                TokenType::Var => {
                    match render_status_foreach {
                        RenderStatusForeach::In => { 
                            match render_status_if {
                                RenderStatusIf::In => { 
                                    if *self.if_vars.get(&if_var).unwrap() { token_stack.push(token) };
                                },
                                RenderStatusIf::Out => { token_stack.push(token) },
                            }
                        },
                        RenderStatusForeach::Out => { 
                            match render_status_if {
                                RenderStatusIf::In => { 
                                    if *self.if_vars.get(&if_var).unwrap() { template_content.push_str(self.vars.get(&token.v).unwrap()); };
                                },
                                RenderStatusIf::Out => { template_content.push_str(self.vars.get(&token.v).unwrap()); },
                            }
                        }
                    }
                },
                TokenType::Foreach => {
                    render_status_foreach = RenderStatusForeach::In;
                    token_stack.push(token);
                },
                TokenType::ForeachVar => {
                    match render_status_if {
                        RenderStatusIf::In => { 
                            if *self.if_vars.get(&if_var).unwrap() { token_stack.push(token); };
                        },
                        RenderStatusIf::Out => { token_stack.push(token); },
                    }
                },
                TokenType::ForeachClose => {
                    render_status_foreach = RenderStatusForeach::Out;
                    let mut my_token_stack = Vec::new();
                    let foreach_name;
                    loop {
                        let my_token = token_stack.pop().unwrap();
                        match my_token.t {
                            TokenType::Foreach => { 
                                foreach_name = my_token.v.clone();
                                break; 
                            },
                            TokenType::Html | TokenType::Var | TokenType::ForeachVar => {
                                my_token_stack.insert(0, my_token);
                            },
                            TokenType::If | TokenType::IfClose => {
                                my_token_stack.insert(0, my_token);
                            },
                            TokenType::ForeachClose => {},
                        }
                    }

                    let my_foreach_vec = self.foreach_vars.get(&foreach_name).unwrap();
                    for element in my_foreach_vec.iter() {
                        for my_token in my_token_stack.iter() {
                            match my_token.t {
                                TokenType::Html => {
                                    template_content.push_str(&my_token.v);
                                },
                                TokenType::Var => {
                                    let c = self.vars.get(&my_token.v).unwrap();
                                    template_content.push_str(c);
                                },
                                TokenType::ForeachVar => {
                                    let c = element.get(&my_token.v).unwrap();
                                    template_content.push_str(c);
                                },
                                TokenType::Foreach => {},
                                TokenType::ForeachClose => {},
                                TokenType::If => {},
                                TokenType::IfClose => {},
                            }
                        }
                    }
                },
                TokenType::If => {
                    render_status_if = RenderStatusIf::In;
                    if_var = token.v.clone();
                },
                TokenType::IfClose => {
                    render_status_if = RenderStatusIf::Out;
                },
            }
        }
        return template_content;
    }
}
