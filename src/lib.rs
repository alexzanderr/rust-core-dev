#![allow(
    non_snake_case,
    non_upper_case_globals,
    unused_imports,
    unused_mut,
    unused_must_use
)]
#![feature(unboxed_closures)]
#![feature(thread_is_running)]
#![feature(trait_alias)]

pub mod util;
pub mod core;
pub mod collections;
pub mod traits;

#[cfg(feature = "aesthetics")]
pub mod aesthetics;

#[cfg(feature = "pathlib")]
pub mod pathlib;

#[cfg(feature = "spinners")]
pub mod spinners;

#[cfg(feature = "random")]
pub mod random;

pub mod icons;
pub mod stringlib;

#[cfg(feature = "algorithms")]
pub mod algorithms;

pub mod shell;

#[cfg(feature = "datetime")]
pub mod datetime;

pub mod audio;


pub mod terminal;

pub mod linuxapi;