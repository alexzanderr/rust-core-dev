


use std::env;

fn increase(_integer: &mut i32) {
    *_integer += 1;
}

fn decrease(_integer: &mut i32) {
    *_integer -= 1;
}

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut integer = 123;
    increase(&mut integer);
    decrease(&mut integer);
    decrease(&mut integer);
    decrease(&mut integer);
    decrease(&mut integer);
    decrease(&mut integer);
    decrease(&mut integer);
    println!("{}", integer);
}
