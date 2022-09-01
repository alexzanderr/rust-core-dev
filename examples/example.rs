#[derive(Debug)]
struct Number {
    value: i32
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number {
            value: item
        }
    }
}

impl From<i64> for Number {
    fn from(item: i64) -> Self {
        Number {
            value: item as i32
        }
    }
}

fn main() {
    let num = Number::from(30i32);
    let num2 = Number::from(31230i64);
    println!("My number is {:?}", num);
    println!("My number is {:?}", num2);
}
