use std::{
    time::SystemTime,
};
use chrono::{DateTime, Utc};

pub fn get_env(key: &str) -> String {
    match  std::env::var(key) {
        Ok(val) => val,
        Err(e) => panic!("couldn't interpret {}: {}", key, e),
    }
  }


pub fn iso_date() -> String {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    return now.to_rfc3339();
}