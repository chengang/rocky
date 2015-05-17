extern crate rocky;
use rocky::Request;

fn main() {
    fn default_handler(req: Request) -> String {
        let mut response = String::new();
        response.push_str("404 Not Found.\n");
        response.push_str(&req.request_script);
        response.push_str(" lost.");
        return response;
    }

    fn handler(req: Request) -> String {
        let mut response = String::new();
        response.push_str("hello world.\n");
        response.push_str("your request_uri is:");
        response.push_str(&req.request_script);
        return response;
    }

    let mut rocky = rocky::new("127.0.0.1", 4321);
    rocky.router.get("default", default_handler );
    rocky.router.get("/hello_world", handler );
    rocky.run();
}
