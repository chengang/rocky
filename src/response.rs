pub struct Response {
    pub body: String,
}

impl Response {
    pub fn new() -> Response {
        Response { body: "".to_string()}
    }

    pub fn echo(&mut self, content: &str) {
        let mut body = &mut self.body;
        body.push_str(content);
    }
}
