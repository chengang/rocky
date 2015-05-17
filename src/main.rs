extern crate rocky;
use rocky::http::RequestInfo;

fn main() {
    let ip = "127.0.0.1";
    let port = 4321;
    fn handler(req: RequestInfo) -> String {
        return req.request_uri;
    }
    rocky::start(ip, port, handler);
}
