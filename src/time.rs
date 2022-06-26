use chrono::prelude::DateTime;
use chrono::Local;
use std::time::{Duration, UNIX_EPOCH};

pub fn unixts_to_string(unixts: u64) -> String {
    let d = UNIX_EPOCH + Duration::from_nanos(unixts);
    let datetime = DateTime::<Local>::from(d);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
