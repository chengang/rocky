use template::Template;
use http::status::http_status;

pub struct Response {
    pub template: Template,
    pub body: Vec<u8>,
    pub status: u16,
    pub response: Vec<u8>,
}

impl Response {
    pub fn new(status: u16) -> Response {
        Response { 
            template: Template::new(),
            body: Vec::new(),
            status: status,
            response: Vec::new(),
        }
    }

    pub fn set_status(&mut self, status: u16) {
        self.status = status;
    }

    pub fn echo(&mut self, content: &str) {
        let mut body = &mut self.body;
        body.extend(content.to_string().into_bytes());
    }

    pub fn set_template(&mut self, template_name: &str) {
        self.template.set_template(template_name);
    }

    pub fn assign(&mut self, var: &str, data: String) {
        self.template.assign(var, data);
    }

    pub fn render(&mut self) {
        let template_content = self.template.render();
        self.body.extend(template_content.into_bytes());
        let http_status = http_status();
        let http_status_string = http_status.get(&self.status).unwrap();
        let result = format!("HTTP/1.0 {}\r\n\
                   Server: Rocky\r\n\
                   Content-Length: {}\r\n\
                   \r\n",
                   http_status_string, self.body.len());
        self.response.extend(result.into_bytes());
        self.response.extend(self.body.clone());
    }
}
