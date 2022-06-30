

# Python Bool

<!-- tocasdasdasd -->
<!-- tocasda -->

```admonish
This code works.
```

usage

```rust
use python::*;
use pretty_assertions::assert_eq;


fn main() {
    let python_bool = Bool::new(true);
    let mut python_list = List::new();
    python_list.append_back(python_bool);
    print(&python_list);
    assert_eq!(len(&python_list), 1)
}

```


run the example from root project
```shell
❱  cargo run -p pylist -q --example append_py_bool
```

output
```shell
[True]
```

