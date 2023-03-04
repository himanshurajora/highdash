pub fn map<T, U>(iterable: &[T], predicate: fn(&T, usize) -> U) -> Vec<U> {
    let mut result: Vec<U> = vec![];
    for (index, item) in iterable.iter().enumerate()  {
        result.push(predicate(&item, index));
    }
    result
}
