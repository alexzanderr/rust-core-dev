

# Python List Append Method


## Table of Contents
- [`Definition`](#definition)
- [`Implementation`](#implementation)
- [`Usage`](#usage-for-end-user-programmer)
- [`References`](#references)


## Definition

```rust
    fn append_back(&mut self, _: T) -> &mut Self;
}


/// inline append for integer
/// example
///
```

as you can see the entire `magic` sits behind a `trait` with a simple function

## Implementation
lets take 2 examples:
- one for `i32`

```rust
impl Append<i64> for List {
    fn append_back(&mut self, _integer: i64) -> &mut Self {
        self._list.push_back(Object::Int64(Int::new(_integer)));
        self
    }
}

```

this takes an integer and pushes it back on the list by creating a new Object enum of Int32 which will hold a Int object which holds the inputted value from user, seems complicated huh ? not quite! there wouldnt be another way for managing `any data type list`. if you find a more  efficient and simpler way to do this, please submit a `Pull Request`!!

<br>

- one for `List` (yes it can append another `List` inside itself; which I call it `list-ception`)

```rust
impl Append<Object> for List {
    fn append_back(&mut self, object: Object) -> &mut Self {
        self._list.push_back(object);
        self
    }
}

```



### Usage for end user programmer

```rust
#![allow(unused_imports)]

use python::*;


fn main() {
    let mut list = List::new();


    list.append_back("from str");
    list.append_back(String::from("from String"));
    list.append_back(List::from("extend from list"));

    list.append_back(123);
    list.append_back(123.123f32);
    list.append_back(123.123f64);


    print(list);
}

```




```shell
❱  cargo run -p pylist -q --example append
```

output
```shell
['from str', 'from String', ['e', 'x', 't', 'e', 'n', 'd', ' ', 'f', 'r', 'o', 'm', ' ', 'l', 'i', 's', 't'], 123, 123.123, 123.123]
```
you can append almost anything


# References
see also

- [`List`](https://github.com/alexzanderr/rust-python-objects/blob/main/src/list/list.rs)
 struct implementation source code
- [`Append`](https://github.com/alexzanderr/rust-python-objects/blob/main/src/list/append.rs)
 trait implementation source code

