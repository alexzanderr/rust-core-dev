


use pretty_assertions::assert_eq;
use pretty_assertions::assert_ne;
use rstest::rstest;


#[cfg(test)]
mod pathlib {
    use super::assert_eq;
    use super::assert_ne;
    use super::rstest;

    #[cfg(test)]
    mod function_get_files_recursive {
        use core_dev::{pathlib::get_files_recursive, traits::StringExtended};
        use super::assert_eq;


        #[test]
        fn dot_non_relative() {
            let to_ignore = vec!["target", ".git", "vec_extended"];
            let files = get_files_recursive(
                ".",
                false,
                Some(&to_ignore)
            );
            for file in files {
                let path = std::path::Path::new(&file);
                assert_eq!(path.is_absolute(), true);
                assert_eq!(path.is_dir(), false);
                let first_folder = file.split_to_vec_string("/")[0].clone();
                for to_ig in &to_ignore {
                    assert_ne!(first_folder, *to_ig);
                }
                println!("{}", file);
            }
        }

        #[test]
        fn dot_relative() {
            let to_ignore = vec!["target", ".git"];
            let files = get_files_recursive(
                ".",
                true,
                Some(&to_ignore)
            );
            for file in files {
                let path = std::path::Path::new(&file);
                assert_eq!(path.is_relative(), true);
                assert_eq!(path.is_dir(), false);
                let first_folder = file.split_to_vec_string("/")[0].clone();
                for to_ig in &to_ignore {
                    assert_ne!(first_folder, *to_ig);
                }
                println!("{}", file);
            }
        }

        #[test]
        fn dir_non_relative() {
            let files = get_files_recursive(
                "src",
                false,
                None
            );
            for file in files {
                let path = std::path::Path::new(&file);
                assert_eq!(path.is_absolute(), true);
                println!("{}", file);
            }
        }

        #[test]
        fn dir_relative() {
            let files = get_files_recursive(
                "src",
                true,
                None
            );
            for file in files {
                let path = std::path::Path::new(&file);
                assert_eq!(path.is_relative(), true);
                println!("{}", file);
            }
        }

    }
}
