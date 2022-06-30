

# Summary of Python List Section
- [`Methods`](./methods)
- [`Macros`](./macros)

# Summary of Current Page
- [`Showcase - List`](#showcase---python-string)

# Showcase - Python String
```rust

#![feature(custom_inner_attributes)]

// do not rust format in vs code on this document
// cuz it will delete everything from it
#![rustfmt::skip]
// this needs to be like this because
// it will fail with cargo-docs sub command
// my command for creating docs automatically from source code
#![allow(dead_code, unused_imports, unused_variables, unused_macros, unused_assignments, unused_mut, non_snake_case, unused_must_use)]


use python::*;
use pretty_assertions::assert_eq;

fn main() {
    let python_string = _String::from("rust static string");
    // you can print the string just like python print
    print(&python_string);
    // you can compare a Python String with a rust static string
    assert_eq!(python_string, "rust static string");

    // you can print string representation as in python `repr`
    print(repr(&python_string));


    // create a python string from a rust dynamic string
    let python_string =
        _String::from(String::from("rust dynamic string"));

    // use str() builtins python function on python string
    // note you need reference cuz the function will consume the object
    // and you dont want that
    print(_str(&python_string));

    // you can iterate over chars of python_string
    for _char in python_string.chars() {
        print(_char);
    }
}
```

```shell
❱  cargo run -p pystring -q --example showcase
```

output
```shell
rust static string
'rust static string'
rust dynamic string
r
u
s
t
d
y
n
a
m
i
c
s
t
r
i
n
g
```

use a python string in rust, yay, nice


this is just `bare bones` and `experimental`, more features will come soon. stay still!

