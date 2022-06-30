
# function `random_choice`

returns a reference from collection slice based on random generated index

```rust
use core_dev::random::random_choice;
let collection = vec![1, 2, 3];
let item = random_choice(&collection);
// first of all to check if the function works
// the returned option must be != None
assert_ne!(item, None);


let item = *item.unwrap();

let mut found = false;
for number in collection {
    if item == number {
        found = true;
        break;
    }
}
// to double check we must find the item in the original collection
assert_eq!(found, true);

// doesnt work in cargo doc -
println!("{}", item);
```