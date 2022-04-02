pub fn sleep(millis: u64) {
    let duration = std::time::Duration::from_millis(millis);
    std::thread::sleep(duration);
}
