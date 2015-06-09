extern crate time;
use self::time::Timespec;
use self::time::at;
use self::time::strftime;

pub fn ts2str(ts: i64) -> String {
    let timespec = Timespec::new(ts, 0);
    let tm = at(timespec);
    let result = strftime("%F %T", &tm).unwrap();
    return result;
}
