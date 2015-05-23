use template::Template;

pub struct Response {
    pub template: Template,
    pub body: String,
}

impl Response {
    pub fn new() -> Response {
        Response { 
            body: "".to_string(),
            template: Template::new(),
        }
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
        self.body = self.template.render();
    }
}
