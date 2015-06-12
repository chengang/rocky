extern crate time;
use self::time::Timespec;
use self::time::at;
use self::time::strftime;
use self::time::now;

pub fn ts2str(ts: i64) -> String {
    let timespec = Timespec::new(ts, 0);
    let tm = at(timespec);
    let result = strftime("%F %T", &tm).unwrap();
    return result;
}

pub fn timestamp() -> i64 {
    let tm_now = now().to_timespec();
    let result = tm_now.sec;
    return result; 
}
