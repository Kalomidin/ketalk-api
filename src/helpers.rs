use chrono::NaiveDateTime;

pub fn get_env(key: &str) -> String {
    match  std::env::var(key) {
        Ok(val) => val,
        Err(e) => panic!("couldn't interpret {}: {}", key, e),
    }
}

pub fn new_naive_date() -> NaiveDateTime {
    // TOOD: improve timing
    chrono::NaiveDateTime::from_timestamp_millis(chrono::Utc::now().timestamp_millis()).unwrap()
}