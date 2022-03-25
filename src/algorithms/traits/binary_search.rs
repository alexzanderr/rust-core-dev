

pub trait BinarySearch<T> {
    fn binary_search(&self, element: T) -> Option<usize>;
}


impl BinarySearch<i8> for Vec<i8> {
    fn binary_search(&self, element: i8) -> Option<usize> {
        let mut start: i32 = 0;
        let mut stop: i32 = self.len() as i32;
        while start < stop {
            let middle = (start + stop) / 2;
            if self[middle as usize] == element {
                return Some(middle as usize);
            } else if self[middle as usize] > element {
                stop = middle - 1;
            } else {
                start = middle + 1;
            }
        }
        None
    }
}