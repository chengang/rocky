extern crate rocky;
use rocky::{Request, Response};
use rocky::Redis;
use rocky::helper::*;

// or use rocky::*

pub fn index(req: Request) -> Response {
    let mut resp = Response::new(200);
    let mut redis = Redis::new("redis://127.0.0.1:6379/");
    let posts: Vec<(String, i64)> = redis.zrange_withscores("posts", -2, -1);
    let (post_content, post_ts) = posts[0].clone();
    let post_datestr = ts2str(post_ts);
    
    resp.set_template("index");
    resp.assign(".post_content", post_content );
    resp.assign(".post_ts", post_datestr );
    return resp;
}
