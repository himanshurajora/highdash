pub fn every<T>(iterable: &[T], predicate: fn(&T, usize) -> bool) -> bool {
    for (index, item) in iterable.iter().enumerate() {
        if !predicate(&item, index) {
            return false;
        }
    }
    return true;
}
