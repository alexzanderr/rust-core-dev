use core_dev::datetime::time_attributes::TimeAttributesBuilder;


use std::io::stdout;
use std::io::Write;

fn main() {
    let mut t = TimeAttributesBuilder::default()
        .seconds(2)
        .minutes(1)
        .build();
    while t.seconds != 0 {
        t.decrement_seconds(1);
        print!("\x1b[2K{}\r", t.format_as_clock_with_level(2));
        t.normalize_decrement();
        stdout().flush().expect("failed to flush stdout");

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    println!()
}
