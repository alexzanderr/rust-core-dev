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
use rstest::rstest;
use rstest::*;

// from current crate
// use rust_core::collections::Counter;

use core_dev::aesthetics::asciify_str;
use pretty_assertions::assert_eq;

use core_dev::aesthetics::ansi;


#[test]
fn test_asciify_str() {
    let result = asciify_str("salutare", None).unwrap();
    println!("{}", result);
    println!("{}text{}", ansi.red, ansi.endc);
    println!("{}text{}", ansi.red, ansi.endc);

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
}

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
