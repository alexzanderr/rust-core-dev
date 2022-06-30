

use std::collections::HashMap;

///
/// usage
///
/// ```rust
/// use core_dev::algorithms::two_sum;
/// let result = two_sum(vec![10, 10, 10, 10, 10], 20);
/// assert_eq!(result, vec![0, 1]);
/// ```
pub fn two_sum(
    nums_vector: Vec<i32>,
    target_sum: i32
) -> Vec<i32> {
    if nums_vector.is_empty() {
        return vec![];
    }
    {
        let minus_ten_to_minus_9 = i32::pow(-10, 9);
        let ten_to_minus_9 = i32::pow(10, 9);

        if target_sum < minus_ten_to_minus_9
        || target_sum > ten_to_minus_9 {
            panic!("target must be between (-10^9, 10^9)")
        }

        if nums_vector.len() < 2
        || nums_vector.len() as i32 > i32::pow(10, 4) {
            panic!("len or arr must be between (2, 10^4)")
        }

        for value in nums_vector.iter() {
            if value < &i32::pow(-10, 9)
            || value > &i32::pow(10, 9) {
                panic!("every element of the array
                    must be between (-10^9, 10^9)")
            }
        }
    }

    let mut pos_table: HashMap<i32, i32> = HashMap::new();

    for (index, value) in nums_vector.iter().enumerate() {
        let diff = target_sum - value;
        if pos_table.contains_key(&diff) {
            return vec![*pos_table.get(&diff).unwrap(), index as i32];
        } else {
            pos_table.insert(*value, index as i32);
        }
    }
    return vec![];
}

/// Note that the modern solution (cargo test -- --show-output) doesn't work in doctests defined in a Markdown code-fence in the docstring of your functions. Only println! (etc.) statements done in a concrete #[test] block will be respected.
///
///
/// usage
///
/// docs tests
/// https://stackoverflow.com/a/71402219/12172291
/// markdown quites are required to work as doc test
/// ```rust
/// use core_dev::algorithms::{max, min};
/// let _chars = vec!['c', 'a', 'a', 'a', 'c', 'x', 'x'];
/// println!("{}", min(&_chars));
///
/// let _numbers = vec![123, 123, 10, 20, 27, 37, 57];
/// println!("{}", max(&_chars));
/// println!("{}", max(&_numbers));
///
/// let _numbers = [-12, 123, 10, 20, 27, 37, 57];
/// println!("{}", max(&_numbers));
/// println!("{}", min(&_numbers));
/// println!("{}", max(&_numbers));
/// ```
pub fn max<T: PartialOrd + Copy>(_vec: &[T]) -> T {
    let mut _max = _vec[0];
    for &value in _vec.iter().skip(1) {
        if value > _max {
            _max = value;
        }
    }
    _max
}

///
///
///
/// usage
/// let _chars = vec!['c', 'a', 'a', 'a', 'c', 'x', 'x'];
/// print(min(&_chars));
///
/// let _numbers = vec![123, 123, 10, 20, 27, 37, 57];
/// print(max(&_chars));
/// print(max(&_numbers));
///
/// let _numbers = [-12, 123, 10, 20, 27, 37, 57];
/// print(max(&_numbers));
/// print(min(&_numbers));
/// print(max(&_numbers));
///
///
///
///
pub fn min<T: PartialOrd + Copy>(_vec: &[T]) -> T {
    let mut _min = _vec[0];
    for &value in _vec.iter().skip(1) {
        if value < _min {
            _min = value;
        }
    }
    _min
}