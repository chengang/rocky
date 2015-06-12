extern crate rocky;
use rocky::{Request, Response};
use rocky::Redis;
use rocky::helper::*;
use std::collections::HashMap;

// or use rocky::*

pub fn index(req: Request) -> Response {
    let mut resp = Response::new(200);
    let mut redis = Redis::new("redis://127.0.0.1:6379/");

    let page = 1;
    let posts_per_page = 100;
    let start = 0 - ( page * posts_per_page );
    let end = 0 - ( ( page - 1 ) * posts_per_page ) - 1;
    let posts: Vec<(String, i64)> = redis.zrange("posts", start, end);

    let mut var_posts = Vec::new();
    for post in posts.iter() {
        let (post_content, post_ts) = post.clone();
        let mut var_post = HashMap::new();
        var_post.insert("post_content".to_string(), post_content);
        var_post.insert("post_datestr".to_string(), ts2str(post_ts));
        var_posts.insert(0, var_post);
    }

    resp.set_template("index");
    resp.assign_array("posts", var_posts );
    resp.assign_bool("have_pic", false );
    return resp;
}
