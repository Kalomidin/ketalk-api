use chrono::NaiveDateTime;
use std::time::{SystemTime};

pub fn get_env(key: &str) -> String {
  match std::env::var(key) {
    Ok(val) => val,
    Err(e) => panic!("couldn't interpret {}: {}", key, e),
  }
}

pub fn new_naive_date() -> NaiveDateTime {
  // TOOD: improve timing
  chrono::NaiveDateTime::from_timestamp_millis(chrono::Utc::now().timestamp_millis()).unwrap()
}

pub fn get_timestamp_as_nano() -> String {
  SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap()
    .as_nanos()
    .to_string()
}
