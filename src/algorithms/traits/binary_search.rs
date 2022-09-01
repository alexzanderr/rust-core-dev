pub trait BinarySearch<T> {
    fn binary_search(
        &self,
        element: T
    ) -> Option<usize>;
}

// inspiration from
// https://doc.rust-lang.org/src/core/cmp.rs.html#1307-1317
macro_rules! impl_binary_search_for {
    ($($vec_type:ty)*) => ($(
    // ($vec_type: ident) => {
        impl BinarySearch<$vec_type> for Vec<$vec_type> {
            fn binary_search(&self, element: $vec_type) -> Option<usize> {
                if self.is_empty() {
                    return None;
                }
                let mut start: $vec_type = 0;
                let mut stop: $vec_type = self.len() as $vec_type;
                while start < stop {
                    let middle = (start + stop) / 2;

                    match self[middle as usize] {
                        elem if elem == element => {
                            return Some(middle as usize)
                        },
                        elem if elem > element => {
                            stop = middle - 1;
                        },
                        elem if elem < element => start = middle + 1,
                        _ => {},
                    }
                    // if self[middle as usize] == element {
                    //     return Some(middle as usize);
                    // } else if self[middle as usize] > element {
                    //     stop = middle - 1;
                    // } else {
                    //     start = middle + 1;
                    // }
                }
                None
            }
        }
    )*)
}

impl_binary_search_for! { i8 i16 i32 i64 i128 }
