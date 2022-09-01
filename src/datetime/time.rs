use super::time_attributes::TimeAttributesBuilder;
use super::time_attributes::TimeAttributes;

pub fn get_utc_timestamp() -> i64 {
    chrono::Utc::now().timestamp()
}

pub fn get_bucharest_timestamp() -> i64 {
    chrono::Utc::now().timestamp() + 3600 * 3
}

pub fn sleep_by_millis(millis: u64) {
    let duration = std::time::Duration::from_millis(millis);
    std::thread::sleep(duration);
}

pub fn sleep_by_secs(seconds: f32) {
    let duration = std::time::Duration::from_secs_f32(seconds as f32);
    std::thread::sleep(duration);
}

pub fn seconds_to_time_struct(seconds: usize) -> TimeAttributes {
    TimeAttributesBuilder::default().seconds(seconds).normalize().build()
}

pub fn minutes_to_time_struct(minutes: usize) -> TimeAttributes {
    TimeAttributesBuilder::default().minutes(minutes).normalize().build()
}
