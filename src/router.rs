use std::collections::HashMap;
use http::RequestInfo;

pub fn new() -> HashMap<String, fn(RequestInfo)->String> {
    let default_router: HashMap<String, fn(RequestInfo)->String> = HashMap::new();
    return default_router;
}
