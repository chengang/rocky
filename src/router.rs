use std::collections::HashMap;
use http::RequestInfo;

pub fn new(handler: fn(RequestInfo)->String) -> HashMap<String, fn(RequestInfo)->String> {
    let mut default_router: HashMap<String, fn(RequestInfo)->String> = HashMap::new();
    default_router.insert("404".to_string(), handler );
    return default_router;
}
