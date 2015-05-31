use std::collections::HashMap;

pub fn http_status() -> HashMap<u16, String> {
    let mut status = HashMap::new();
    status.insert(100, "100 CONTINUE".to_string() );
    status.insert(101, "101 SWITCHING PROTOCOLS".to_string() );
    status.insert(102, "102 PROCESSING".to_string() );

    status.insert(200, "200 OK".to_string() );
    status.insert(201, "201 CREATED".to_string() );
    status.insert(202, "202 ACCEPTED".to_string() );
    status.insert(203, "203 NON AUTHORITATIVE INFORMATION".to_string() );
    status.insert(204, "204 NO CONTENT".to_string() );
    status.insert(205, "205 RESET CONTENT".to_string() );
    status.insert(206, "206 PARTIAL CONTENT".to_string() );
    status.insert(207, "207 MULTI STATUS".to_string() );
    status.insert(208, "208 ALREADY REPORTED".to_string() );

    status.insert(300, "300 MULTIPLE CHOICES".to_string() );
    status.insert(301, "301 MOVED PERMANENTLY".to_string() );
    status.insert(302, "302 FOUND".to_string() );
    status.insert(303, "303 SEE OTHER".to_string() );
    status.insert(304, "304 NOT MODIFIED".to_string() );
    status.insert(305, "305 USE PROXY".to_string() );
    status.insert(307, "307 TEMPORARY REDIRECT".to_string() );

    status.insert(400, "400 BAD REQUEST".to_string() );
    status.insert(401, "401 UNAUTHORIZED".to_string() );
    status.insert(402, "402 PAYMENT REQUIRED".to_string() );
    status.insert(403, "403 FORBIDDEN".to_string() );
    status.insert(404, "404 NOT FOUND".to_string() );
    status.insert(405, "405 METHOD NOT ALLOWED".to_string() );
    status.insert(406, "406 NOT ACCEPTABLE".to_string() );
    status.insert(407, "407 PROXY AUTHENTICATION REQUIRED".to_string() );
    status.insert(408, "408 REQUEST TIMEOUT".to_string() );
    status.insert(409, "409 CONFLICT".to_string() );
    status.insert(410, "410 GONE".to_string() );
    status.insert(411, "411 LENGTH REQUIRED".to_string() );
    status.insert(412, "412 PRECONDITION FAILED".to_string() );
    status.insert(413, "413 REQUEST ENTITY TOO LARGE".to_string() );
    status.insert(414, "414 REQUEST URI TOO LARGE".to_string() );
    status.insert(415, "415 UNSUPPORTED MEDIA TYPE".to_string() );
    status.insert(416, "416 REQUEST RANGE NOT SATISFIABLE".to_string() );
    status.insert(417, "417 EXPECTATION FAILED".to_string() );
    status.insert(418, "418 I AM A TEAPOT".to_string() );
    status.insert(422, "422 UNPROCESSABLE ENTITY".to_string() );
    status.insert(423, "423 LOCKED".to_string() );
    status.insert(424, "424 FAILED DEPENDENCY".to_string() );
    status.insert(425, "425 NO CODE".to_string() );
    status.insert(426, "426 UPGRADE REQUIRED".to_string() );
    status.insert(428, "428 PRECONDITION REQUIRED".to_string() );
    status.insert(429, "429 TOO MANY REQUESTS".to_string() );
    status.insert(431, "431 REQUEST HEADER FIELDS TOO LARGE".to_string() );
    status.insert(449, "449 RETRY WITH".to_string() );

    status.insert(500, "500 INTERNAL SERVER ERROR".to_string() );
    status.insert(501, "501 NOT IMPLEMENTED".to_string() );
    status.insert(502, "502 BAD GATEWAY".to_string() );
    status.insert(503, "503 SERVICE UNAVAILABLE".to_string() );
    status.insert(504, "504 GATEWAY TIMEOUT".to_string() );
    status.insert(505, "505 HTTP VERSION NOT SUPPORTED".to_string() );
    status.insert(506, "506 VARIANT ALSO NEGOTIATES".to_string() );
    status.insert(507, "507 INSUFFICIENT STORAGE".to_string() );
    status.insert(509, "509 BANDWIDTH LIMIT EXCEEDED".to_string() );
    status.insert(510, "510 NOT EXTENDED".to_string() );
    status.insert(511, "511 NETWORK AUTHENTICATION REQUIRED".to_string() );

    return status;
}
