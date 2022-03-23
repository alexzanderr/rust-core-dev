

use globwalk;

use std::path::Path;
use crate::traits::StringExtended;
use std::fs;


pub fn get_files_recursive<P>(
    target_folder: P,
    relative_to_cwd: bool,
    to_ignore: Option<&Vec<&str>>
) -> Vec<String>
where P: AsRef<Path>
{

    let target_folder = target_folder.as_ref().display().to_string();
    let mut found_dot = false;
    let glob_pattern: String;
    if target_folder == "." {
        found_dot = true;
        glob_pattern = String::from("**");
    } else {
        glob_pattern = format!("{}/**", target_folder);
    }

    let mut files = Vec::new();

    for entry in globwalk::glob(&glob_pattern).unwrap() {
        if let Ok(entry) = entry {
            let mut entry = entry.path().display().to_string();
            if entry.get_char(0).unwrap() == '.' {
                entry = entry[2..].to_string();
            }

            let path = Path::new(&entry);
            if path.is_file() {
                // let mut parts: Vec<String> = entry.split_to_vec_string("/");
                // parts.remove(0);
                // files.push(parts.join("/"));
                if let Some(to_ignore_vec) = &to_ignore {
                    let mut entry_parts = entry.split_to_vec_string("/");
                    if entry_parts.len() >= 2 {
                        entry_parts = entry_parts[..entry_parts.len() - 1].to_vec();
                    }

                    let mut shall_continue = false;
                    for to_ig in to_ignore_vec.iter() {
                        // check folder `recursively`
                        // suppose you have a path like this
                        // `src/target/main/something/target/something/file.md`
                        // and you want to ignore target folder
                        // you must iterate through every part and continue
                        // if you find the ignored one
                        for entry_part in entry_parts.iter() {
                            if entry_part == to_ig {
                                shall_continue = true;
                                break;
                            }
                        }
                    }

                    if shall_continue {
                        continue
                    }
                }

                if found_dot {
                    if relative_to_cwd {
                        files.push(entry);
                    } else {
                        files.push(fs::canonicalize(entry).unwrap().display().to_string());
                    }
                } else {
                    if relative_to_cwd {
                        files.push(entry);
                    } else {
                        files.push(fs::canonicalize(&path).unwrap().display().to_string());
                    }
                }
            }
        }
    }
    files
}


// pub fn get_paths_recursive<P>(
//     target_folder: P,
//     relative_to_cwd: bool,
//     to_ignore: Option<Vec<&str>>
// ) -> Vec<P>
// where P: AsRef<Path>
// {

//     let target_folder = target_folder.as_ref().display().to_string();
//     let mut found_dot = false;
//     let glob_pattern: String;
//     if target_folder == "." {
//         found_dot = true;
//         glob_pattern = String::from("**");
//     } else {
//         glob_pattern = format!("{}/**", target_folder);
//     }

//     let mut paths = Vec::new();

//     for entry in globwalk::glob(&glob_pattern).unwrap() {
//         if let Ok(entry) = entry {
//             let entry = entry.path().display().to_string();
//             let path = Path::new(&entry);
//             if path.is_file() {
//                 // let mut parts: Vec<String> = entry.split_to_vec_string("/");
//                 // parts.remove(0);
//                 // files.push(parts.join("/"));
//                 let to_append: Path;
//                 if found_dot {
//                     to_append = Path::new(&entry[2..]);
//                 } else {
//                     if relative_to_cwd {
//                         to_append = Path::new(entry[2..]);
//                     } else {
//                         to_append = Path::new(fs::canonicalize(&path).unwrap());
//                     }
//                 }
//                 paths.push(to_append);
//             }
//         }
//     }
//     paths
// }
