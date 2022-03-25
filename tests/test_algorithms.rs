


use pretty_assertions::assert_eq;

#[cfg(test)]
mod binary_search_trait {
    use core_dev::algorithms::traits::BinarySearch;

    // use core_dev::algorithms::traits::BinarySearch;
    use super::assert_eq;


    #[test]
    fn against_i8() {
        let mut some_vec = vec![123, 123, 22, 1, 2, 3i8];
        assert_eq!(some_vec.binary_search(3).unwrap(), 5);
    }
}