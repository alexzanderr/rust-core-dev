use std::ops::Range;

pub const ASCII_LOWERCASE: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub const ASCII_UPPERCASE: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
    'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];


const UPPERCASE_RANGE: Range<u8> = 65..91;
const LOWERCASE_RANGE: Range<u8> = 97..123;

// const fn get_uppercase_len() -> usize {
//     uppercase_range.len()
// }

// const total_uppers: usize = get_uppercase_len();
// const total_lowers: usize = lowercase_range.len();


pub fn generate_ascii_alpha_lowercase() -> Vec<char> {
    LOWERCASE_RANGE.map(|index| index as char).collect()
}

pub fn generate_ascii_alpha_uppercase() -> Vec<char> {
    // const variable: i32 = 123;
    UPPERCASE_RANGE.map(|index| index as char).collect()
}

// const fn generate_ascii_alpha() -> Vec<char> {
//     let mut upper = generate_ascii_alpha_uppercase();
//     upper.extend(generate_ascii_alpha_lowercase());
//     upper
// }

// pub const ascii_uppercase: Vec<char> = generate_ascii_alpha_uppercase();
// pub const ascii_lowercase: Vec<char> = generate_ascii_alpha_lowercase();
