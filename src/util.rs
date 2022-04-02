pub fn sizeof<T>(_object: &T) -> usize {
    std::mem::size_of_val(_object)
}
