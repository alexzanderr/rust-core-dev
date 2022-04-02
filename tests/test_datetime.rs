use rstest::rstest;
use pretty_assertions::assert_eq;
use pretty_assertions::assert_ne;


#[cfg(test)]
mod test_module_datetime {
    use super::rstest;
    use super::assert_eq;
    use super::assert_ne;
    /// how to run a module that is behind a feature
    /// cargo test --test test_datetime -j 8 --features datetime -- --show-output
    /// cargo test --features <feature_name>

    #[cfg(test)]
    mod function_seconds_to_time_map {
        use super::rstest;
        use super::assert_eq;
        use super::assert_ne;
        use core_dev::datetime::datetime::seconds_to_time_map;
        use std::collections::HashMap;

        fn build_time_map<'a>(
            from_keys_and_values: Vec<(&'a str, i128)>,
        ) -> HashMap<&'a str, i128> {
            let mut time_map: HashMap<&str, i128> = vec![
                ("millennials", 0),
                ("centuries", 0),
                ("decades", 0),
                ("years", 0),
                ("weeks", 0),
                ("days", 0),
                ("hours", 0),
                ("minutes", 0),
                ("seconds", 0),
            ]
            .into_iter()
            .collect();
            for (key, value) in from_keys_and_values.into_iter() {
                time_map.entry(&key).and_modify(|e| *e = value);
            }
            time_map
        }

        #[rstest]
        #[case(3600, build_time_map(vec![("hours", 1)]))]
        #[case(3601, build_time_map(vec![
            ("hours", 1),
            ("seconds", 1),
        ]))]
        #[case(86400, build_time_map(vec![
            ("days", 1),
        ]))]
        #[case(86401, build_time_map(vec![
            ("days", 1),
            ("seconds", 1),
        ]))]
        fn test_seconds_to_time_map(
            #[case] seconds: usize,
            #[case] expected_time_map: HashMap<&str, i128>,
        ) {
            let result_time_map = seconds_to_time_map(seconds);
            assert_eq!(result_time_map, expected_time_map);
        }
    }
}
