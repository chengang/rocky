extern crate rocky;
use rocky::Request;

fn main() {
    fn handler(req: Request) -> String {
        let mut response = String::new();
        response.push_str("hello world.\n");
        response.push_str("your request_uri is:");
        response.push_str(&req.request_uri);
        return response;
    }

    let mut rocky = rocky::new("127.0.0.1", 4321);
    rocky.router.get("404", handler );
    rocky.run();
}
