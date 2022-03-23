
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    // std::env::set_var("RUST_BACKTRACE", "full");
    // std::env::set_var("COLORBT_SHOW_HIDDEN", "1");

    // for i in 0..100 {
    //     let x = i * i;
    //     println!("x is {}", x);
    // }
    panic!("it was me");
    // ...
    Ok(())
}