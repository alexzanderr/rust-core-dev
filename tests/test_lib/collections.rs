

#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
    non_snake_case
)]

// 3rd party
use rstest::rstest;
use rstest::*;
    
// from current crate
use core_dev::collections::Counter;



#[rstest]
#[case("your package is very cool")]
#[case("your package is very cool")]
#[case("your package is very cool")]
fn test_Counter(
    #[case] _static_string: &str
) {
    let counter = Counter::from_str(_static_string);
    println!("{}", counter);
    // assert_eq!(result, expected_result);
}


