




use pretty_assertions::assert_eq;

use rstest::rstest;
use rstest::fixture;


#[cfg(test)]
mod binary_search_trait {
    use assert_cmd::prelude::OutputOkExt;
    use core_dev::algorithms::traits::BinarySearch;

    // use core_dev::algorithms::traits::BinarySearch;
    use super::assert_eq;


    #[test]
    fn against_i8() {
        let mut some_vec = vec![123, 123, 22, 1, 2, 3i8];
        assert_eq!(some_vec.binary_search(3).unwrap(), 5);
    }

    // Uses the quickcheck crate for property-based tests
    // https://github.com/BurntSushi/quickcheck
    //
    // This function will be automatically called with a variety of inputs. Since this is a unit function,
    // the test will pass as long as it doesn't panic. You could also return a `Testable` value like a `bool`
    #[quickcheck]
    fn binary_search_doesnt_panic(input: Vec<i64>, target: i64) {
        println!("{:?} - {}", input, target);
        let result = input.binary_search(target);
        println!("{:?}", result)
        // factorial(input);
    }


}


#[fixture]
pub fn fixture() -> u32 { 42 }

#[rstest]
fn should_success(fixture: u32) {
    assert_eq!(fixture, 42);
}

// #[rstest]
// fn should_fail(fixture: u32) {
//     assert_ne!(fixture, 42);
// }

/// If you need to just providing a bunch of values for which you need to run your test, you can use #[values(list, of, values)] argument attribute:
#[rstest]
fn should_be_invalid(
    #[values(None, Some(""), Some("    "))]
    value: Option<&str>
) {
    // assert!(!valid(value))
}

#[rstest]
#[case(vec![123, 123, 123, 123, 2, 8], 10, vec![4, 5])]
#[case(vec![123, 123, 123, 123, 2, 8, 10, 100], 110, vec![6, 7])]
#[case(vec![123, 123, 123, 123, 10, 100], 10, vec![])]
#[case(vec![], 10, vec![])]
#[case(vec![10, 10, 10, 10, 10], 20, vec![0, 1])]
fn test_two_sum(
    #[case] nums_vector: Vec<i32>,
    #[case] target_sum: i32,
    #[case] expected_result: Vec<i32>
) {
    let result = core_dev::algorithms::two_sum(nums_vector, target_sum);
    assert_eq!(result, expected_result);
}


use rstest_reuse::{self, *};

// #[template]
// #[rstest]
// #[case(2, 2)]
// #[case(4/2, 2)]
// #[case(8/2, 4)]
// fn two_simple_cases(#[case] a: u32, #[case] b: u32) {
//     println!("this is like a trait, but for test cases");
//     println!("this will never run");

// }

// #[apply(two_simple_cases)]
// fn it_works(#[case] a: u32, #[case] b: u32) {
//     assert!(a == b);
// }

use std::net::SocketAddr;

// If you need a value where its type implement FromStr() trait you can use a literal string to build it:
#[rstest]
#[case("1.2.3.4:8080", 8080)]
#[case("127.0.0.1:9000", 9000)]
fn check_port(#[case] addr: SocketAddr, #[case] expected: u16) {
    assert_eq!(expected, addr.port());
}

// rstest provides out of the box async support. Just mark your test function as async and it'll use #[async-std::test] to annotate it. This feature can be really useful to build async parametric tests using a tidy syntax:
// #[rstest]
// #[case(5, 2, 3)]
// #[should_panic]
// #[case(42, 40, 1)]
// async fn my_async_test(#[case] expected: u32, #[case] a: u32, #[case] b: u32) {
//     assert_eq!(expected, async_sum(a, b).await);
// }