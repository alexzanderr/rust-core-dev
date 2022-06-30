
# list! macro
create a `List` using `list!` just like `vec!`

example code
```rust
use python::*;


fn main() {
    let list = list![123, 123, 123.123, 'c', "hello"];
    print(list);
    let list =
        list![Int::new(2), list!["another", List::from("another")]];
    print(list);
}

```

run the code
```shell
❱  cargo run -p pylist -q --example macro
```

output
```shell
[123, 123, 123.123, 'c', 'hello']
[2, ['another', ['a', 'n', 'o', 't', 'h', 'e', 'r']]]
```
as you can see it works and we have `list-ception` too

