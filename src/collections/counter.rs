



use core::fmt;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::hash::Hash;
use std::str::Chars;



pub struct Counter {
    _counter_char: HashMap<char, i32>,
    chars: bool,
    _counter_int: HashMap<i32, i32>,
}


impl Counter {
    fn create_hashmap_from_chars(_char_iter: Chars<'_>) -> HashMap<char, i32> {
        let mut _counter = HashMap::new();
        for _char in _char_iter {
            *_counter.entry(_char).or_insert(0) += 1;
        }
        _counter
    }

    pub fn from_str(_static_string: &str) -> Counter {
        Counter {
            _counter_char: Counter::create_hashmap_from_chars(_static_string.chars()),
            chars: true,
            _counter_int: HashMap::new(),
        }
    }

    pub fn from_string(_string: String) -> Counter {
        Counter {
            _counter_char: Counter::create_hashmap_from_chars(_string.chars()),
            chars: true,
            _counter_int: HashMap::new()
        }
    }

    pub fn from_string_ref(_string_ref: &String) -> Counter {
        Counter {
            _counter_char: Counter::create_hashmap_from_chars(_string_ref.chars()),
            chars: true,
            _counter_int: HashMap::new()
        }
    }

    pub fn from_i32(_i32_number: i32) -> Counter {
        let mut __i32_number: i32 = _i32_number;
        let mut _counter: HashMap<i32, i32> = HashMap::new();

        while __i32_number != 0 {
            let digit = __i32_number % 10;
            *_counter.entry(digit).or_insert(0) += 1;
            __i32_number /= 10;
        }

        Counter {
            _counter_char: HashMap::new(),
            chars: false,
            _counter_int: _counter,
        }
    }
}


impl Display for Counter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.chars {
            write!(f, "{:?}", self._counter_char)
        } else {
            write!(f, "{:?}", self._counter_int)
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