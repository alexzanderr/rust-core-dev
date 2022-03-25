



use core_dev::random::*;


fn main() {
    let rf = random_float();
    println!("{}", rf);

    let rd = random_digit();
    println!("{}", rd);


    for i in 0..10090 {
        let rc = random_alpha_upper_char();
        // print!("{:?} ", rc);
        let rc = random_alpha_lower_char();
        // print!("{:?} ", rc);
    }

    println!("something");
    println!("done");
}