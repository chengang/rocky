extern crate rocky;
use rocky::{Request, Response};

pub fn index(req: Request) -> Response {
    let mut resp = Response::new();
    resp.set_template("index");
    resp.assign("var", "你好".to_string());
    resp.render();
    return resp;
}
