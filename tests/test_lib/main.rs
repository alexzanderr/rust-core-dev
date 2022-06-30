

#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
    non_snake_case,
    unused_must_use,
    non_upper_case_globals,
    non_camel_case_types,
)]


#[macro_use]
extern crate core_dev;

#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod algorithms;
mod collections;
mod datetime;
mod core;
mod aesthetics;
mod pathlib;
mod shell;
mod stringlib;
mod traits;
mod random;