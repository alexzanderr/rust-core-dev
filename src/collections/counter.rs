use core::fmt;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::hash::Hash;
use std::str::Chars;

pub struct Counter {
    _counter_char: HashMap<char, i32>,
    chars:         bool,
    _counter_int:  HashMap<i32, i32>
}

impl Counter {
    fn create_hashmap_from_chars(
        _char_iter: Chars<'_>
    ) -> HashMap<char, i32> {
        let mut _counter = HashMap::new();
        for _char in _char_iter {
            *_counter.entry(_char).or_insert(0) += 1;
        }
        _counter
    }
}

impl Display for Counter {
    fn fmt(
        &self,
        f: &mut Formatter<'_>
    ) -> Result {
        if self.chars {
            write!(f, "{:?}", self._counter_char)
        } else {
            write!(f, "{:?}", self._counter_int)
        }
    }
}

impl From<&str> for Counter {
    fn from(_str: &str) -> Self {
        Counter {
            _counter_char: Counter::create_hashmap_from_chars(
                _str.chars()
            ),
            chars:         true,
            _counter_int:  HashMap::new()
        }
    }
}

impl From<String> for Counter {
    fn from(_string: String) -> Self {
        Counter {
            _counter_char: Counter::create_hashmap_from_chars(
                _string.chars()
            ),
            chars:         true,
            _counter_int:  HashMap::new()
        }
    }
}

impl From<i32> for Counter {
    fn from(num: i32) -> Self {
        let mut __i32_number: i32 = num;
        let mut _counter: HashMap<i32, i32> = HashMap::new();

        while __i32_number != 0 {
            let digit = __i32_number % 10;
            *_counter.entry(digit).or_insert(0) += 1;
            __i32_number /= 10;
        }

        Counter {
            _counter_char: HashMap::new(),
            chars:         false,
            _counter_int:  _counter
        }
    }
}

// impl Iterator for Counter {

//     fn next(&mut self) -> Option<> {
//         let res = self.vals.get(self.count);
//         match res {
//             Some(v) => {
//                 self.count += 1;
//                 Some(*v)
//             },
//             None => None,
//         }
//     }
// }
