use std::io;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::time::{
    Duration,
    Instant,
};


/// pub fn read_lines<P>(filename: P) -> Vec<String>
/// where P: AsRef<Path> {
/// usage
/// from String
///
/// for line in read_lines(String::from("examples/data/test.txt")) { ... }
///
/// from &str
/// for line in read_lines("examples/data/test.txt") { ... }
///
/// from Path object
/// for line in read_lines(Path::new("examples/data/test.txt")) { ... }
pub fn read_lines<P: AsRef<Path>>(filename: P) -> Vec<String> {
    // convert to Path object
    let path: &Path = filename.as_ref();
    if !path.exists() {
        return vec![];
    }

    let mut lines = Vec::new();
    let file = File::open(filename).unwrap();
    for line in io::BufReader::new(file).lines() {
        lines.push(line.unwrap());
    }
    lines
}


pub fn get_execution_time<F>(_function: F) -> Duration
where
    F: Fn() -> (), {
    let execution = Instant::now();
    _function();
    execution.elapsed()
}

pub fn print_execution_time<F>(_function: F)
where
    F: Fn() -> (), {
    let start = Instant::now();
    _function();
    let duration = start.elapsed();

    println!("Time elapsed in _function() is: {:?}", duration);
}


pub fn set_keyboard_interrupt_handler<F>(_function: F)
where
    F: Fn() -> () + Send + 'static, {
    ctrlc::set_handler(_function).unwrap()
}
