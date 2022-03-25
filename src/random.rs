use rand::Rng;
use rand::random;
use rand::thread_rng;

use crate::stringlib::ASCII_LOWERCASE;
use crate::stringlib::ASCII_UPPERCASE;


pub fn random_float() -> f32 {
    random()
}


pub fn random_digit() -> i32 {
    thread_rng().gen_range(0..10)
}


pub fn random_ascii_char() -> char {
    thread_rng().gen_range(0..128u8) as char
}


/// https://www.asciitable.com/
pub fn random_alpha_lower_char() -> char {
    let index = thread_rng().gen_range(0..26usize);
    ASCII_LOWERCASE[index]
}


/// https://www.asciitable.com/
pub fn random_alpha_upper_char() -> char {
    let index = thread_rng().gen_range(0..26usize);
    ASCII_UPPERCASE[index]
}
