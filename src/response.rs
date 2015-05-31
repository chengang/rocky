use template::Template;
use http::status::http_status;

pub struct Response {
    pub template: Template,
    pub body: String,
    pub status: u16,
    pub response: String,
}

impl Response {
    pub fn new() -> Response {
        Response { 
            body: "".to_string(),
            template: Template::new(),
            status: 403,
            response: "".to_string(),
        }
    }

    pub fn status(&mut self, status: u16) {
        self.status = status;
    }

    pub fn echo(&mut self, content: &str) {
        let mut body = &mut self.body;
        body.push_str(content);
    }

    pub fn set_template(&mut self, template_name: &str) {
        self.template.set_template(template_name);
    }

    pub fn assign(&mut self, var: &str, data: String) {
        self.template.assign(var, data);
    }

    pub fn render(&mut self) {
        let template_content = self.template.render();
        self.body.push_str(&template_content);
        let http_status = http_status();
        let http_status_string = http_status.get(&self.status).unwrap();
        self.response = format!("HTTP/1.0 {}\r\n\
                   Server: Rocky\r\n\
                   Content-Length: {}\r\n\
                   \r\n\
                   {}",
                   http_status_string, self.body.len(), self.body);
    }
}
