use std::collections::HashMap;
use http::request::Request;
use http::response::Response;

pub struct Router {
    pub routers_get:  HashMap<String, fn(Request)->Response>,
    pub routers_post: HashMap<String, fn(Request)->Response>,
}

impl Clone for Router {
    fn clone(&self) -> Router {
        Router {
            routers_get:  self.routers_get.clone(),
            routers_post: self.routers_post.clone(),
        }
    }
}

impl Router {
    pub fn new() -> Router {
        let routers_get:  HashMap<String, fn(Request)->Response> = HashMap::new();
        let routers_post: HashMap<String, fn(Request)->Response> = HashMap::new();
        Router { 
            routers_get:  routers_get,
            routers_post: routers_post,
        }
    }

    pub fn get(&mut self, url: &str, handler: fn(Request)->Response) {
        let mut routers_get = &mut self.routers_get;
        routers_get.insert(url.to_string(), handler );
    }

    pub fn post(&mut self, url: &str, handler: fn(Request)->Response) {
        let mut routers_post = &mut self.routers_post;
        routers_post.insert(url.to_string(), handler );
    }
}
