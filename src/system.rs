use std::io::{
    stdin,
    stdout,
    Read,
    Write
};

/// the old C style system pause
pub fn pause() {
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    let mut buffer = [0; 1024];
    stdin().read_exact(&mut buffer).unwrap();
}
