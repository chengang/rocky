extern crate time;
use self::time::Timespec;
use self::time::at;
use self::time::strftime;

pub fn ts2str(ts: u32) -> String {
    let timespec = Timespec::new(1433860335, 0);
    let tm = at(timespec);
    let result = strftime("%F %T", &tm).unwrap();
    return result;
}
