

use std::io::Write;

pub fn printcr(text: &str) {
    print!("\x1b[2K{}\r", text);
    std::io::stdout().flush().expect("failed to flush stdout");
}