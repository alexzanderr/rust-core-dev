use std::path::Path;

use core_dev::core::read_lines;

fn main() {
    let data_folder = Path::new("examples/data");
    // let filename = root_data.to_owned() + "/test.txt";

    // join is not in place, returns PathBuf
    let filename = data_folder.join("test.txts");

    println!("{:?}", filename);

    for line in read_lines(String::from("examples/data/test.txt")) {
        println!("{}", line);
    }
}
