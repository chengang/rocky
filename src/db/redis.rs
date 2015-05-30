use std::str::from_utf8;

extern crate redis;
use self::redis::Commands;

pub struct Redis {
    pub addr: String,
}

impl Redis {
    pub fn new(addr: &str) -> Redis {
        Redis {
            addr: addr.to_string(),
        }
    }

    pub fn get(&mut self, key: &str) -> String {
        let addr: &str= from_utf8(self.addr.as_bytes()).unwrap();
        let client = redis::Client::open(addr).unwrap();
        let con = client.get_connection().unwrap();

        let result: String = con.get(key).unwrap();
        return result;
    }

    pub fn set(&mut self, key: &str, value: &str) {
        let addr: &str= from_utf8(self.addr.as_bytes()).unwrap();
        let client = redis::Client::open(addr).unwrap();
        let con = client.get_connection().unwrap();

        let _ :() = con.set(key, value).unwrap();
    }
}
