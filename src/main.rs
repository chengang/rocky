extern crate rocky;
use rocky::{Request, Response};

fn main() {
    fn default_handler(req: Request) -> Response {
        let mut response = Response::new();
        response.echo("404 Not Found.\n");
        response.echo(&req.request_script);
        response.echo(" lost.");
        return response;
    }

    fn handler(req: Request) -> Response {
        let mut response = Response::new();
        response.echo("hello world.\n");
        response.echo("your request_uri is:");
        response.echo(&req.request_script);
        return response;
    }

    let mut rocky = rocky::new("127.0.0.1", 4321);
    rocky.router.get("default", default_handler );
    rocky.router.get("/hello_world", handler );
    rocky.run();
}
