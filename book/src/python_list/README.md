
<div align="right">
<a href="https://play.rust-lang.org/?edition=2018&gist=d69d8e3156d4bb81c4461b60b772ab72" target="_blank">
<img align="center" width="85" src="https://raw.githubusercontent.com/serde-rs/serde-rs.github.io/master/img/runtab.png">
</a>
</div>


# Summary of Python List Section
<!-- note that this ./methods is only available for the book; this is working only on websites -->
<!-- i dont think its working if you try to access this file path from `docs.rs` website -->
- [`Methods`](./methods)
- [`Macros`](./macros)

# Summary of Current Page
- [`Showcase - Python List`](#showcase---python-list)
- [`noway`](#noway)

# Showcase - Python List
```rust
#![allow(dead_code, unused_imports, unused_variables, unused_macros, unused_assignments, unused_mut, non_snake_case, unused_must_use)]


use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

// use python::Object;
// use python::Int;
// use python::Float;
// use python::Bool;
// use python::_String;
// use python::List;

// // builtins.rs
// use python::print;
// use python::len;
// use python::repr;

// the crate name is 'python-objects'
// because there is another crate out there with `python` name
// but the lib.rs (library crate of this crate) its called `python`
// so you can import like this
extern crate python;
// actually 'extern crate' is useless
// just use only 'use python::'

// use everything from python
use python::*;

fn main() {
    // create a new python list
    let mut python_list = List::from(String::from("123123"));
    // at this point the list will look like this
    // ['1', '2', '3', '1', '2', '3']

    // append an integer
    python_list.append_back(123);
    // append a rust static string
    python_list.append_front("hello");
    python_list.append_back(123);
    // append a list
    python_list.append_back(List::from(String::from("working")));
    // note that the python list supports another python list inside

    // append a float
    python_list.append_back(123.123);
    python_list.append_back(123.123);
    python_list.append_back(123.123);
    // append a rust String
    python_list.append_back(String::from("asdasd"));

    python_list.append_back(List::from("something".to_string()));

    // append a python string
    // note that this _String is from this crate
    // its the struct that handles the String and &str data types
    python_list
        .append_back(_String::from(String::from("python string")));

    // append a python bool
    // note that Bool is the python struct that handles rust's bool
    python_list.append_back(Bool::new(true));
    python_list.append_back(Bool::new(false));
    // append a rust bool
    python_list.append_back(false);

    // print just like in python
    print(&python_list);

    // use len just like in python
    print(len(&python_list));


    // python_list.append_front("salutare");

    // iterate over the list just like in python
    // there are plans for future to remove the .iter()
    // so you can use for o in python_list { ... }, just that simple
    for o in python_list.iter() {
        print(o)
    }

    print("\n");
    print("----");
    print(python_list.__str__());
    print("----");
    print("\n");

    // create a python from parsing a static string
    let list_from_str = "123123".parse::<List>().unwrap();
    print(&list_from_str);


    let iter = (0..5).into_iter();
    // let list_from_iterator: List = iter.collect();

    // create a python list from rust iterator
    let list_from_iterator = iter.collect::<List>();
    print(&list_from_iterator);


    print(repr(&python_list));
    print(repr(&list_from_iterator));

    python_list.extend(List::from("extended"));
    print(&python_list);

    // let oo = Object::Int(Int::new(123));
    // println!("{}", oo);

    // let oo = Object::Float(Float::new(123.123));
    // println!("{}", oo);

    // let empty_list = List::new();
    // println!("{}", empty_list);

    // // i would like to to this inline stuff
    // // let one_elem = List::new().append_int(123);
    // let mut one_elem = List::new();
    // one_elem
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123);

    // print(&one_elem);
    // print(len(one_elem));

    // let mut one_elem = List::new();
    // one_elem
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123)
    //     .append_int(123);

    // print(repr(one_elem));

    // let python_string = _String::from_string(String::from("asd"));

    // print(python_string);

    // let list_from_integer = List::from(123);
    // // [1, 2, 3]
    // let list_from_string = List::from(String::from("abc"));
    // // ['a', 'b', 'c']
    // let list_from_static_string = List::from("asd");
    // // ['a', 's', 'd']

    // let python_list = List::new();
    // python_list.append(123);
    // python_list.append(123.123);
    // python_list.append(List::new());
    // python_list.append("static string");
    // python_list.append("String object".to_string());

    // python_list.extend(List::from(123));

    // print(python_list);
    // print(repr(python_list));

    // let integer = Int { value: 123i32};
    // let floater = Float { value: 123.123f32};

    // println!("{}", len(&integer));
    // println!("{}", len(integer));
}

```

```shell
❱  cargo run -p pylist -q --example showcase
```

output
```shell
['hello', '1', '2', '3', '1', '2', '3', 123, 123, ['w', 'o', 'r', 'k', 'i', 'n', 'g'], 123.123, 123.123, 123.123, 'asdasd', ['s', 'o', 'm', 'e', 't', 'h', 'i', 'n', 'g'], 'python string', True, False, False]
19
hello
1
2
3
1
2
3
123
123
['w', 'o', 'r', 'k', 'i', 'n', 'g']
123.123
123.123
123.123
asdasd
['s', 'o', 'm', 'e', 't', 'h', 'i', 'n', 'g']
python string
True
False
False
----
['hello', '1', '2', '3', '1', '2', '3', 123, 123, ['w', 'o', 'r', 'k', 'i', 'n', 'g'], 123.123, 123.123, 123.123, 'asdasd', ['s', 'o', 'm', 'e', 't', 'h', 'i', 'n', 'g'], 'python string', True, False, False]
----
['1', '2', '3', '1', '2', '3']
[0, 1, 2, 3, 4]
"['hello', '1', '2', '3', '1', '2', '3', 123, 123, ['w', 'o', 'r', 'k', 'i', 'n', 'g'], 123.123, 123.123, 123.123, 'asdasd', ['s', 'o', 'm', 'e', 't', 'h', 'i', 'n', 'g'], 'python string', True, False, False]"
'[0, 1, 2, 3, 4]'
['hello', '1', '2', '3', '1', '2', '3', 123, 123, ['w', 'o', 'r', 'k', 'i', 'n', 'g'], 123.123, 123.123, 123.123, 'asdasd', ['s', 'o', 'm', 'e', 't', 'h', 'i', 'n', 'g'], 'python string', True, False, False, 'e', 'x', 't', 'e', 'n', 'd', 'e', 'd']
```

as you can see the list contains `char`, `float (f32)`, `integer (i32)`, `a rust string`, `another python list`, a `python string` and many more data types.

this is just `bare bones` and `experimental`, more features will come soon. stay still!
