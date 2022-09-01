pub trait ToSome<T = Self>
where
    T: Sized {
    fn to_some(self) -> Option<T>;
}

#[macro_export]
macro_rules! impl_to_some_for {
    ($($type:ty)*) => ($(
    // ($type: ident) => {
        impl ToSome for $type {
            fn to_some(self) -> Option<$type> {
                Some(self)
            }
        }
    )*)
}

impl_to_some_for! {
    i8 i16 i32 i64 i128
    u8 u16 u32 u64 u128
    f32 f64
    String
}
