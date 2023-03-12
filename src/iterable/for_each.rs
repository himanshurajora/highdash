pub fn for_each<T>(iterable: &[T], predicate: impl Fn(&T, usize) -> ()) -> () {
    for (index, item) in iterable.iter().enumerate() {
        predicate(&item, index)
    }
}
