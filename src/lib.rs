extern crate threadpool;

use self::threadpool::ThreadPool;
use std::net::TcpListener;
use std::net::Ipv4Addr;
pub use http::RequestInfo;
pub use router::Router;

pub mod http;
pub mod router;

pub struct Rocky {
    listener: TcpListener,
    pub router: Router,
}

pub fn new(ip: &str, port: u16) -> Rocky {
    let listener_ip = ip.parse::<Ipv4Addr>().unwrap();
    let listener = TcpListener::bind((listener_ip, port)).unwrap();
    let router = router::new();
    let rocky = Rocky {
        listener: listener,
        router: router,
    };
    return rocky;
}

impl Rocky {
    pub fn run(&self) {
        let pool = ThreadPool::new(32);
        for stream in self.listener.incoming() {
            let router = self.router.routers.clone();
            match stream {
                Ok(stream) => {
                    pool.execute(move|| {
                        http::handle_client(stream, router);
                    });
                }
                Err(e) => { let _ = e;}
            }
        }
        //drop(self.listener);
    }
}
