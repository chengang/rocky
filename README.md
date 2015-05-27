EXAMPLE
------

    extern crate rocky;
    use rocky::{Rocky, Request, Response};

    fn main() {
      fn default_handler(req: Request) -> Response {
        let mut resp = Response::new();
        resp.echo("404 Not Found.\n");
        resp.echo(&req.request_script);
        resp.echo(" lost.");
        return resp;
      }

      fn handler(req: Request) -> Response {
        let mut resp = Response::new();
        resp.echo("hello world.\n");
        resp.echo("your request_uri is:");
        resp.echo(&req.request_script);
        return resp;
      }

      fn template_handler(req: Request) -> Response {
        let mut resp = Response::new();
        resp.set_template("123");
        resp.assign("var", "你好".to_string());
        resp.render();
        resp.echo("your request_uri is:");
        resp.echo(&req.request_script);
        return resp;
      }

      let mut rocky = Rocky::new("127.0.0.1", 4321);
      rocky.router.get("default", default_handler );
      rocky.router.get("/hello_world", handler );
      rocky.router.get("/template", template_handler );
      rocky.run();
    }

