use std::{vec::Vec};

/// add index_of method for standard lib vector, which doesnt exist without this trait
pub trait IndexOfVector<T> {
    /// get the index of a specified element
    fn index_of(
        &self,
        element: &T
    ) -> Option<usize>;

    /// get the index of specified element from start index
    fn index_from(
        &self,
        element: &T,
        start_index: usize
    ) -> Option<usize>;

    /// get the index of a spcified element from reverse vector
    fn rindex_of(
        &self,
        element: &T
    ) -> Option<usize>;

    /// get the index of a spcified element from reverse vector from starting point
    fn rindex_from(
        &self,
        element: &T,
        start_index: usize
    ) -> Option<usize>;
}

impl<T> IndexOfVector<T> for Vec<T>
where
    T: PartialEq
{
    fn index_of(
        &self,
        element: &T
    ) -> Option<usize> {
        self.iter().position(|e| e == element)
    }

    fn index_from(
        &self,
        element: &T,
        start_index: usize
    ) -> Option<usize> {
        let option = self[start_index..].iter().position(|e| e == element);

        option.map(|index| index + start_index)
    }

    fn rindex_of(
        &self,
        element: &T
    ) -> Option<usize> {
        self.iter().rposition(|e| e == element)
    }

    fn rindex_from(
        &self,
        element: &T,
        start_index: usize
    ) -> Option<usize> {
        let option =
            self[start_index..].iter().rposition(|e| e == element);

        option.map(|index| index + start_index)
    }
}
