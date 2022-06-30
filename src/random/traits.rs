

use rand::Rng;
use rand::random;
use rand::thread_rng;

pub trait RandomFloat<F: Sized> {
    fn random_float() -> F;
}

macro_rules! impl_random_float_for {
    ($($type:ty)*) => ($(
    // ($type: ident) => {
        impl RandomFloat<$type> for $type {
            fn random_float() -> $type {
                random()
            }
        }
    )*)
}

impl_random_float_for!{ f32 f64 }


pub trait RandomInt<I>
where I: Sized {
    fn random_int() -> I;
}

macro_rules! impl_random_int_for {
    ($($type:ty)*) => ($(
    // ($type: ident) => {
        impl RandomInt<$type> for $type {
            fn random_int() -> $type {
                thread_rng().gen::<$type>()
            }
        }
    )*)
}

impl_random_int_for!{ i8 i16 i32 i64 i128 }


pub trait RandomChoice<T> {
    fn random_choice(&self) -> Option<&T>;
}

use super::random_choice;

macro_rules! impl_random_choice_for_vec_of {
    ($($type:ty)*) => ($(
    // ($type: ident) => {
        impl RandomChoice<$type> for Vec<$type> {
            fn random_choice(&self) -> Option<&$type> {
                random_choice(self)
            }
        }
    )*)
}

impl_random_choice_for_vec_of!{ i8 i16 i32 i64 i128 }
impl_random_choice_for_vec_of!{ u8 u16 u32 u64 u128 }
impl_random_choice_for_vec_of!{ String }


