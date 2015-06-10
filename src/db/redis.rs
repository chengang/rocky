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

    pub fn set(&mut self, key: &str, value: &str) {
        let addr: &str= from_utf8(self.addr.as_bytes()).unwrap();
        let client = redis::Client::open(addr).unwrap();
        let con = client.get_connection().unwrap();

        let _ :() = con.set(key, value).unwrap();
    }

    pub fn get(&mut self, key: &str) -> String {
        let addr: &str= from_utf8(self.addr.as_bytes()).unwrap();
        let client = redis::Client::open(addr).unwrap();
        let con = client.get_connection().unwrap();

        let result: String = con.get(key).unwrap();
        return result;
    }

    // todo: use T return value
    pub fn zrange_withscores(&mut self, key: &str, start: isize, stop: isize) -> Vec<(String, i64)> {
        let addr: &str= from_utf8(self.addr.as_bytes()).unwrap();
        let client = redis::Client::open(addr).unwrap();
        let con = client.get_connection().unwrap();

        let redis_bulk: Vec<(String, i64)> = con.zrange_withscores(key, start, stop).unwrap();
        return redis_bulk;
        /*
        let mut result = Vec::new();
        for one_line in redis_bulk.iter() {
            let (content, score): (String, i64) = (*one_line).clone();
            result.push((content, score));
        }
        return result;
        */
    }
}
