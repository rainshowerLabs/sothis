use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_latest_unix_timestamp() -> u64 {
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Failed to obtain duration since UNIX epoch");
    let timestamp = duration_since_epoch.as_secs();
    timestamp
}
