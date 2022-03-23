
use std::process::Command;
// use std::process::Output;

fn main() {
    let process_communicate = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    let exit_code = process_communicate.status;
    let status = process_communicate.status.success();
    let output = String::from_utf8_lossy(&process_communicate.stdout);
    let output = output.trim();

    let error = String::from_utf8_lossy(&process_communicate.stderr);
    let error = error.trim();

    println!("{}", exit_code);
    println!("{:?}", output);
    println!("{:?}", error);
    println!("{}", status);

    use std::arch::asm;
    let x: u64;
    unsafe {
        asm!("mov {}, 5", out(reg) x);
    }
    assert_eq!(x, 5);

    unsafe {
        asm!("nop");
        asm! { "nop" }
    }
}
