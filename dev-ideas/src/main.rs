// redundant
// extern crate python;
use python::*;


// macro_rules! list {
//     [] => {
//         List::new()
//     };

//     [$($argument: expr), *] => {
//         {
//             let mut new_list = List::new();
//             $(
//                 new_list.append_back($argument);
//             )*
//             new_list
//         }
//     };
// }


fn main() {
    println!("Hello, world!");
    // notice the curly brackets
    // just like dict in python
    // btw dict is incoming
    let list = list! {123, 123, 123.123, 'c', "hello"};
    print(list);
    // notice the curly brackets
    let list =
        list! {Int::new(2), list!{"another", List::from("another")}};
    print(list);


    let mut vector = Vec::new();
    for c in "asjikodb asjibdaisjbd asjdb".chars() {
        vector.push(c);
    }
    for c in vector.iter_mut() {
        *c = 'c';
    }
    for c in vector.iter() {
        print(c);
    }
}
