//! Useful Docs for this module
//!
//! <https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html#generate-random-numbers-within-a-range>

use rand::Rng;
use rand::random;
use rand::thread_rng;

use crate::stringlib::ASCII_LOWERCASE;
use crate::stringlib::ASCII_UPPERCASE;

pub fn random_float() -> f32 {
    random()
}

/// <https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html#generate-random-values-of-a-custom-type>
pub fn random_i32() -> i32 {
    thread_rng().gen::<i32>()
}

pub fn random_digit() -> i32 {
    thread_rng().gen_range(0..10)
}

pub fn random_ascii_char() -> char {
    thread_rng().gen_range(0..128u8) as char
}

/// <https://www.asciitable.com/>
pub fn random_alpha_lower_char() -> char {
    let index = thread_rng().gen_range(0..26usize);
    ASCII_LOWERCASE[index]
}

/// <https://www.asciitable.com/>
pub fn random_alpha_upper_char() -> char {
    let index = thread_rng().gen_range(0..26usize);
    ASCII_UPPERCASE[index]
}

#[doc = include_str!("../../docs/random/random/fn_random_choice.md")]
/// extra doc
pub fn random_choice<T>(collection: &[T]) -> Option<&T> {
    // if collections is empty
    // then there is nothing to return from it
    if collection.is_empty() {
        return None;
    }
    // else generate index from 0 .. len - 1
    let index = thread_rng().gen_range(0..collection.len());
    // dbg!(&index);
    // return reference to the collection item
    collection.get(index)
}

/// returns random index based on the collection length
/// index = [0, len - 1]
#[inline]
pub fn random_index<T>(collection: &[T]) -> usize {
    if collection.is_empty() {
        0
    } else {
        thread_rng().gen_range(0..collection.len()).into()
    }
}
