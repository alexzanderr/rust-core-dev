use pretty_assertions::assert_eq;
use rstest::rstest;

#[cfg(test)]
mod index_of_vector_trait {
    use super::assert_eq;
    use core_dev::traits::IndexOfVector;
    use super::rstest;

    #[cfg(test)]
    mod method_index_of {
        use super::IndexOfVector;
        use super::assert_eq;

        #[test]
        fn against_vec_i32() {
            let some_vector = vec![123, 123, 33, 22, 11, 20i32];
            // note that you can call index_of() method on vector
            // this method is not in the std lib
            let index = some_vector.index_of(&33);
            if let Some(index) = index {
                assert_eq!(index, 2);
            } else {
                assert_eq!(index, None);
            }
        }

        #[test]
        fn against_vec_usize() {
            let some_vector = vec![123, 123, 33, 22, 11, 20usize];
            // note that you can call index_of() method on vector
            // this method is not in the std lib
            let index = some_vector.index_of(&33);
            if let Some(index) = index {
                assert_eq!(index, 2);
            } else {
                assert_eq!(index, None);
            }
        }
    }

    #[cfg(test)]
    mod method_index_from {
        use super::IndexOfVector;
        use super::assert_eq;

        #[test]
        fn against_vec_i32() {
            let some_vector: Vec<i32> = vec![2, 0, 2, 2, 1, 2];
            // note that you can call index_of() method on vector
            // this method is not in the std lib
            let index = some_vector.index_from(&2, 0).unwrap();
            assert_eq!(index, 0);

            let index = some_vector.index_from(&2, 1).unwrap();
            assert_eq!(index, 2);

            let index = some_vector.index_from(&2, 2).unwrap();
            assert_eq!(index, 2);

            let index = some_vector.index_from(&2, 3).unwrap();
            assert_eq!(index, 3);

            let index = some_vector.index_from(&2, 4).unwrap();
            assert_eq!(index, 5);
        }
    }

    #[cfg(test)]
    mod method_rindex_of {
        use super::IndexOfVector;
        use super::assert_eq;

        #[test]
        fn against_vec_i32() {
            let some_vector: Vec<i32> = vec![2, 0, 2, 2, 1, 2];
            let index = some_vector.rindex_of(&2).unwrap();
            assert_eq!(index, some_vector.len() - 1);
        }
    }

    #[cfg(test)]
    mod method_rindex_from {
        use super::IndexOfVector;
        use super::assert_eq;
        use super::rstest;

        #[rstest]
        #[case(vec![2, 0, 2, 2, 1, 3], 2, 2, Some(3))]
        #[case(vec![2, 0, 2, 2, 1, 3], 2, 3, Some(3))]
        #[case(vec![2, 0, 2, 2, 1, 3], 2, 4, None)]
        fn against_vec_i32(
            #[case] vector_i32: Vec<i32>,
            #[case] element: i32,
            #[case] start_index: usize,
            #[case] expected_index: Option<usize>
        ) {
            let found_index =
                vector_i32.rindex_from(&element, start_index);
            assert_eq!(found_index, expected_index);
        }
    }
}

#[cfg(test)]
mod max_vector_trait {
    use core_dev::traits::MaxVector;
    use super::assert_eq;
    use super::rstest;

    #[rstest]
    #[case(vec![2, 0, 2, 2, 1, 3i8], Some(3))]
    #[case(vec![2, 0, 2, 8, 1, 3i8], Some(8))]
    #[case(vec![], None)]
    fn against_vec_i8(
        #[case] vector: Vec<i8>,
        #[case] expected_max: Option<i8>
    ) {
        let found_index = vector.find_max();
        assert_eq!(found_index, expected_max);
    }

    #[rstest]
    #[case(vec![2, 0, 2, 2, 1, 3i16], Some(3))]
    #[case(vec![2, 0, 2, 8, 1, 3i16], Some(8))]
    #[case(vec![], None)]
    fn against_vec_i16(
        #[case] vector: Vec<i16>,
        #[case] expected_max: Option<i16>
    ) {
        let found_index = vector.find_max();
        assert_eq!(found_index, expected_max);
    }

    #[rstest]
    #[case(vec![2, 0, 2, 2, 1, 3], Some(3))]
    #[case(vec![2, 0, 2, 8, 1, 3], Some(8))]
    #[case(vec![], None)]
    fn against_vec_i32(
        #[case] vector: Vec<i32>,
        #[case] expected_max: Option<i32>
    ) {
        let found_index = vector.find_max();
        assert_eq!(found_index, expected_max);
    }

    #[rstest]
    #[case(vec![2, 0, 2, 2, 1, 3i64], Some(3))]
    #[case(vec![2, 0, 2, 8, 1, 3i64], Some(8))]
    #[case(vec![], None)]
    fn against_vec_i64(
        #[case] vector: Vec<i64>,
        #[case] expected_max: Option<i64>
    ) {
        let found_index = vector.find_max();
        assert_eq!(found_index, expected_max);
    }

    #[rstest]
    #[case(vec![2, 0, 2, 2, 1, 3i128], Some(3))]
    #[case(vec![2, 0, 2, 8, 1, 3i128], Some(8))]
    #[case(vec![], None)]
    fn against_vec_i128(
        #[case] vector: Vec<i128>,
        #[case] expected_max: Option<i128>
    ) {
        let found_index = vector.find_max();
        assert_eq!(found_index, expected_max);
    }
}

#[cfg(test)]
mod string_extended_trait {
    use super::assert_eq;
    use super::rstest;
    use core_dev::traits::StringExtended;

    #[cfg(test)]
    mod method_split_lines {
        use super::assert_eq;
        use super::rstest;
        use core_dev::traits::StringExtended;

        #[test]
        fn against_string() {
            let some_string =
                String::from("hello\nworld\nthese\nare\nsome\nlines");
            let mut lines = some_string.split_lines().into_iter();
            assert_eq!(lines.next(), Some("hello".to_string()));
            assert_eq!(lines.next(), Some("world".to_string()));
            assert_eq!(lines.next(), Some("these".to_string()));
            assert_eq!(lines.next(), Some("are".to_string()));
            assert_eq!(lines.next(), Some("some".to_string()));
            assert_eq!(lines.next(), Some("lines".to_string()));
            assert_eq!(lines.next(), None);
        }

        #[test]
        fn against_multiline_string() {
            let some_string = String::from(
                "hello
world
these
are
some
lines"
            );
            let mut lines = some_string.split_lines().into_iter();
            assert_eq!(lines.next(), Some("hello".to_string()));
            assert_eq!(lines.next(), Some("world".to_string()));
            assert_eq!(lines.next(), Some("these".to_string()));
            assert_eq!(lines.next(), Some("are".to_string()));
            assert_eq!(lines.next(), Some("some".to_string()));
            assert_eq!(lines.next(), Some("lines".to_string()));
            assert_eq!(lines.next(), None);
        }

        #[test]
        fn against_str() {
            let some_string = "hello\nworld\nthese\nare\nsome\nlines";
            let mut lines = some_string.split_lines().into_iter();
            assert_eq!(lines.next(), Some("hello".to_string()));
            assert_eq!(lines.next(), Some("world".to_string()));
            assert_eq!(lines.next(), Some("these".to_string()));
            assert_eq!(lines.next(), Some("are".to_string()));
            assert_eq!(lines.next(), Some("some".to_string()));
            assert_eq!(lines.next(), Some("lines".to_string()));
            assert_eq!(lines.next(), None);
        }

        #[test]
        fn against_str_multiline() {
            let some_string = "hello
world
these
are
some
lines";
            let mut lines = some_string.split_lines().into_iter();

            assert_eq!(lines.next(), Some("hello".to_string()));
            assert_eq!(lines.next(), Some("world".to_string()));
            assert_eq!(lines.next(), Some("these".to_string()));
            assert_eq!(lines.next(), Some("are".to_string()));
            assert_eq!(lines.next(), Some("some".to_string()));
            assert_eq!(lines.next(), Some("lines".to_string()));
            assert_eq!(lines.next(), None);
        }
    }

    #[cfg(test)]
    mod method_get_char {
        use std::ops::Index;

        use super::assert_eq;
        use super::rstest;
        use super::StringExtended;

        #[test]
        fn against_string() {
            let some_string = String::from("rust is great");
            let found_char = some_string.get_char(0).unwrap();
            assert_eq!(found_char, 'r');
        }

        #[test]
        fn against_str() {
            let some_string = "rust is great";
            let found_char = some_string.get_char(0).unwrap();
            assert_eq!(found_char, 'r');
        }
    }

    #[cfg(test)]
    mod method_capitalize {
        use super::assert_eq;
        use super::rstest;
        use super::StringExtended;

        #[test]
        fn against_string() {
            let some_string = String::from("rust is great");
            let capitalized = some_string.capitalize();
            assert_eq!(capitalized, String::from("Rust is great"));
        }

        #[test]
        fn against_str() {
            let some_string = "rust is great";
            let capitalized = some_string.capitalize();
            assert_eq!(capitalized, "Rust is great");
        }

        #[test]
        fn failed() {
            assert_eq!(true, true);
        }
    }
}
