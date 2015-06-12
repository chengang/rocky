extern crate rocky;
use rocky::*;

pub fn new(req: Request) -> Response {
    let mut resp = Response::new(200);
    let mut redis = Redis::new("redis://127.0.0.1:6379/");
    resp.echo("ok");
    return resp;
}
