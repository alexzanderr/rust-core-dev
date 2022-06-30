#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
    non_snake_case,
    non_upper_case_globals
)]


// 3rd party

// from current crate
// use rust_core::collections::Counter;

// use core_dev::aesthetics::asciify_str;

// use core_dev::aesthetics::ansi;

use pretty_assertions::assert_eq;
// rstest
use rstest::rstest;
use rstest::fixture;
use rstest_reuse::template;
use rstest_reuse::apply;
use rstest_reuse::{self,};



use test_generator::test_resources;


#[test_resources("test_data/*/words.txt")]
fn test_data_from_files(file: &str) {
    let contents = std::fs::read_to_string(file).unwrap();
    let lines = contents.split("\n").collect::<Vec<&str>>();
    for line in lines {
        println!("{}", line);
    }
    // assert!(std::path::Path::new(resource).exists());
}

use assert2::assert;
use assert2::check;

#[test]
fn test_assert2() {
    let x = 32;
    check!(x == 32);
    // check!(x == 31);
    // assert!(x == 123);
}

#[cfg(test)]
mod mod_ansi {
    use super::rstest;
    use super::fixture;
    use super::assert_eq;
    use super::template;
    use super::apply;
    use super::rstest_reuse;

    /// this is what should look like when you convert a string into ansi codes with
    /// red, green, blue ..., functions
    pub fn generate_ansi_red(content: &str) -> String {
        format!("\u{1b}[31m{}\u{1b}[0m", content)
    }

    use core_dev::aesthetics::ansi::red;
    use core_dev::aesthetics::ansi::red_bold;
    use core_dev::aesthetics::ansi::red_on_bg;

    #[template]
    #[rstest]
    #[case("hello")]
    #[case("there")]
    #[case("rust")]
    #[case("is")]
    #[case("the")]
    #[case("best")]
    fn words(#[case] input_content: &str) {
    }

    #[apply(words)]
    fn test_red(#[case] input_content: &str) {
        // this is 100% correct
        let preprocessed_expected_result =
            generate_ansi_red(input_content);

        let result = red(input_content);
        // println!("{:?}", result);
        assert_eq!(result, preprocessed_expected_result);

        println!("{}", red_on_bg("x", "yellow"));
    }

    use core_dev::impl_fixed_color_function;


    use paste::paste;
    use ansi_term::Color;

    #[apply(words)]
    fn against_123(#[case] input_content: &str) {
        impl_fixed_color_function!(123);
        let result = fixed_color_123(input_content);
        println!("{:?}", result)
    }
}


// #[test]
// fn test_asciify_str() {
//     let result = asciify_str("salutare", None).unwrap();
//     println!("{}", result);
//     println!("{}text{}", ansi.red, ansi.endc);
//     println!("{}text{}", ansi.red, ansi.endc);

// sublime format imi sterge liniile de dupa _    _
// de-aia nu merge, dar ce faci frate la stringuri de multiline cand vrei sa dai format la trailing whitespace ??
//     let expected_result = String::from(
// r#"            _       _
//            | |     | |
//   ___  __ _| |_   _| |_ __ _ _ __ ___
//  / __|/ _` | | | | | __/ _` | '__/ _ \
//  \__ \ (_| | | |_| | |_ (_| | | |  __/
//  |___/\__,_|_|\__,_|\__\__,_|_|  \___|
// "#);

//     println!("{}", expected_result);
//     assert_eq!(result, expected_result);
// }

// #[rstest]
// #[case("your package is very cool")]
// #[case("your package is very cool")]
// #[case("your package is very cool")]
// fn test_Counter(
// #[case] _static_string: &str
// ) {
// let counter = Counter::from_str(_static_string);
// println!("{}", counter);
// assert_eq!(result, expected_result);
// }
//
