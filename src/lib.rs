extern crate threadpool;

use self::threadpool::ThreadPool;
use std::net::TcpListener;
use std::net::Ipv4Addr;

pub mod http;
pub mod router;
pub mod template;
pub mod common;
pub mod helper;
pub mod db;
pub use http::request::Request;
pub use http::response::Response;
pub use router::Router;
pub use template::Template;
pub use common::*;
pub use helper::*;
pub use db::redis::Redis;

pub struct Rocky {
    listener: TcpListener,
    pub router: Router,
}

impl Rocky {
    pub fn new(ip: &str, port: u16) -> Rocky {
        let listener_ip = ip.parse::<Ipv4Addr>().unwrap();
        let listener = TcpListener::bind((listener_ip, port)).unwrap();
        Rocky {
            listener: listener,
            router: Router::new(),
        }
    }

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
