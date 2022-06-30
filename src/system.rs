use std::io::{
    stdin,
    stdout,
    Read,
    Write,
};

/// the old C style system pause
pub fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
