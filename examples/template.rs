use string_template::Template;
use std::collections::HashMap;

fn main() {
    let template = Template::new("1. {{package-name}}\n2. {{lang}} developer.\n3. {{something}}\n4. {{something_missing}}");

    let mut args = HashMap::new();
    args.insert("package-name", "cargo-create");
    args.insert("lang", "Rust");
    // daca ai keys in plus merge
    args.insert("something", "that doesnt exist");

    let s = template.render(&args);
    println!("{}", s);
}
