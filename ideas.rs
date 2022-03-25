

fn main() {
    let current_line = line!();
    let current_file = file!();
    let current_column = column!();
    println!("defined on line|column: {}:{}", current_line, current_column);
    println!("defined in file: {}", current_file);
    println!("defined in file: {:?}", std::fs::canonicalize(&current_file).unwrap());

    // println!("{:?}", function!());
}