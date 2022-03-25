use std::io;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;


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




