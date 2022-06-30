

#![no_main]
use libfuzzer_sys::fuzz_target;

use core_dev::algorithms::two_sum;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let _ = two_sum(vec![10, 10, 10, 10, 10], 20);
});
