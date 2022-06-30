#![allow(
    dead_code,
    unused_variables,
    non_snake_case,
    non_upper_case_globals,
    unused_imports,
    unused_mut,
    unused_must_use,
    non_camel_case_types,
)]
// #![feature(const_for)]
#![feature(unboxed_closures)]
#![feature(slice_pattern)]
// #![feature(trait_alias)]
#![allow(
    clippy::module_inception,
    clippy::useless_format,
    clippy::derive_hash_xor_eq,
    clippy::unused_io_amount,
    clippy::new_without_default
)]

pub mod util;

#[cfg(feature = "core")]
pub mod core;

pub mod collections;


#[cfg(feature = "synh")]
pub mod synh;

#[cfg(feature = "traits")]
pub mod traits;

#[cfg(feature = "aesthetics")]
pub mod aesthetics;
// #[cfg(feature = "aesthetics")]
// pub use ansi_term::Color::Red;

#[cfg(feature = "pathlib")]
pub mod pathlib;

#[cfg(feature = "spinners")]
pub mod spinners;

#[cfg(feature = "random")]
pub mod random;


#[cfg(feature = "icons")]
pub mod icons;

#[cfg(feature = "stringlib")]
pub mod stringlib;

#[cfg(feature = "algorithms")]
pub mod algorithms;

#[cfg(feature = "shell")]
pub mod shell;

#[cfg(feature = "datetime")]
pub mod datetime;

#[cfg(feature = "audio")]
pub mod audio;


#[cfg(feature = "terminal")]
pub mod terminal;

#[cfg(feature = "linuxapi")]
pub mod linuxapi;

#[cfg(feature = "image")]
pub mod imagelib;

#[cfg(feature = "weather")]
pub mod weather;


#[cfg(feature = "email")]
pub mod email;

pub mod system;
