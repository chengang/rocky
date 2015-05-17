use std::collections::HashMap;
use http::Request;
use response::Response;

pub struct Router {
    pub routers: HashMap<String, fn(Request)->Response>,
}

pub fn new() -> Router {
    let routers: HashMap<String, fn(Request)->Response> = HashMap::new();
    let router = Router { routers: routers};
    return router;
}

impl Router {
    pub fn get(&mut self, acl: &str, handler: fn(Request)->Response) {
        let mut routers = &mut self.routers;
        routers.insert(acl.to_string(), handler );
    }
}
