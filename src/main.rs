extern crate rocky;
use rocky::Rocky;

mod app;
use app::*;

fn main() {
    let mut rocky = Rocky::new("127.0.0.1", 4321);
    rocky.router.get("/", index::index);
    rocky.router.get("/post/new", post::new);
    rocky.run();
}
