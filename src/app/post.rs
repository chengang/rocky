extern crate rocky;
use rocky::*;

pub fn new(req: Request) -> Response {
    let mut resp = Response::new(200);
    let mut redis = Redis::new("redis://127.0.0.1:6379/");

    let content = req.get_argv.get("content").unwrap();
    let _ :i64 = redis.zadd("posts", &content, timestamp() );
    resp.echo("ok");
    return resp;
}
