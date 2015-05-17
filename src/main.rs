extern crate rocky;
use rocky::RequestInfo;

fn main() {
    fn handler(req: RequestInfo) -> String {
        return req.request_uri;
    }

    let mut rocky = rocky::new("127.0.0.1", 4321);
    rocky.router.insert("404".to_string(), handler );
    rocky.run();
}
