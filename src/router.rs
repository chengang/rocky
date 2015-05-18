use std::collections::HashMap;
use request::Request;
use response::Response;

pub struct Router {
    pub routers: HashMap<String, fn(Request)->Response>,
}

impl Router {
    pub fn new() -> Router {
        let routers: HashMap<String, fn(Request)->Response> = HashMap::new();
        Router { routers: routers }
    }

    pub fn get(&mut self, acl: &str, handler: fn(Request)->Response) {
        let mut routers = &mut self.routers;
        routers.insert(acl.to_string(), handler );
    }
}
