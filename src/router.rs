use std::collections::HashMap;
use http::RequestInfo;

pub struct Router {
    pub routers: HashMap<String, fn(RequestInfo)->String>,
}

pub fn new() -> Router {
    let routers: HashMap<String, fn(RequestInfo)->String> = HashMap::new();
    let router = Router { routers: routers};
    return router;
}

impl Router {
    pub fn get(&mut self, acl: &str, handler: fn(RequestInfo)->String) {
        let mut routers = &mut self.routers;
        routers.insert(acl.to_string(), handler );
    }
}
