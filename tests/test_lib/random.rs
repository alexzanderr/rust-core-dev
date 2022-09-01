#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
    non_snake_case,
    unused_must_use,
    non_upper_case_globals,
    non_camel_case_types
)]

#[cfg(test)]
mod random {
    use rstest::rstest;
    use pretty_assertions::assert_eq;

    #[cfg(test)]
    mod fn_random_choice {
        use super::{
            rstest,
            assert_eq
        };
        use core_dev::random::random_choice;

        #[rstest]
        // #[case(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])]
        #[case(vec![], None)]
        #[case(vec![1], Some(&1))]
        fn special_cases(
            #[case] collection: Vec<i32>,
            #[case] expected_result: Option<&i32>
        ) {
            let item = random_choice(&collection);
            assert_eq!(item, expected_result);
        }

        #[rstest]
        #[case(vec![1, 2, 3i32])]
        #[case(vec![100, 100, 100, 100, 100, 100, 100, 100i32])]
        fn against_vec_i32(#[case] collection: Vec<i32>) {
            dbg!(&collection);
            let item = random_choice(&collection);
            // first of all to check if the function works
            // the returned option must be != None
            assert_ne!(item, None);

            let item = *item.unwrap();

            let mut found = false;
            for number in collection {
                if item == number {
                    found = true;
                    break;
                }
            }
            // to double check we must find the item in the original collection
            assert_eq!(found, true);

            // doesnt work in cargo doc -
            // println!("{}", item);
        }
    }

    #[cfg(test)]
    mod trait_random_choice {
        use super::rstest;
        use core_dev::random::traits::RandomChoice;

        #[rstest]
        // #[case(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])]
        #[case(vec![], None)]
        #[case(vec![1], Some(&1))]
        fn special_cases(
            #[case] collection: Vec<i32>,
            #[case] expected_result: Option<&i32>
        ) {
            let item = collection.random_choice();
            assert_eq!(item, expected_result);
        }

        #[rstest]
        #[case(vec![1, 2, 3i32])]
        #[case(vec![100, 100, 100, 100, 100, 100, 100, 100i32])]
        fn against_vec_i32(#[case] collection: Vec<i32>) {
            dbg!(&collection);
            let item = collection.random_choice();
            // first of all to check if the function works
            // the returned option must be != None
            assert_ne!(item, None);

            let item = *item.unwrap();

            let mut found = false;
            for number in collection {
                if item == number {
                    found = true;
                    break;
                }
            }
            // to double check we must find the item in the original collection
            assert_eq!(found, true);

            // doesnt work in cargo doc -
            // println!("{}", item);
        }

        #[rstest]
        #[case(vec!["asd", "1231", "123123"])]
        #[case(vec!["hello", "rust", "is", "the", "best", "lang"])]
        fn against_vec_string(#[case] collection: Vec<&str>) {
            dbg!(&collection);
            // im too lazy to write .to_string() for every string in the above cases
            let collection: Vec<String> =
                collection.into_iter().map(|s| s.to_string()).collect();

            let item = collection.random_choice();
            // first of all to check if the function works
            // the returned option must be != None
            assert_ne!(item, None);

            let item = item.unwrap();

            let mut found = false;
            for _string in collection.iter() {
                if item == _string {
                    found = true;
                    break;
                }
            }
            // to double check we must find the item in the original collection
            assert_eq!(found, true);

            // doesnt work in cargo doc -
            // println!("{}", item);
        }
    }

    #[cfg(test)]
    mod random_float {
        use super::{
            rstest,
            assert_eq
        };
        use core_dev::random::traits::RandomFloat;
        use core_dev::random::traits::RandomInt;

        #[test]
        fn random_float() {
            let rf = f32::random_float();
            println!("{}", rf);
            let rf = f64::random_float();
            println!("{}", rf);
            let rf = i32::random_int();
            println!("{}", rf);
        }
    }
}
