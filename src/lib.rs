extern crate threadpool;

use self::threadpool::ThreadPool;
use std::net::TcpListener;
use std::net::Ipv4Addr;

pub mod http;
pub mod router;

pub fn start(ip: &str, port: u16, handler: fn(http::RequestInfo)->String) {
    let listener_ip = ip.parse::<Ipv4Addr>().unwrap();
    let listener = TcpListener::bind((listener_ip, port)).unwrap();

    let pool = ThreadPool::new(32);

    let router = router::new(handler);

    for stream in listener.incoming() {
        let router = router.clone();
        match stream {
            Ok(stream) => {
                pool.execute(move|| {
                    http::handle_client(stream, router);
                });
            }
            Err(e) => { let _ = e;}
        }
    }
    drop(listener);
}
