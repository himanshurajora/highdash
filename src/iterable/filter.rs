pub fn filter<T: Clone>(iterable: &Vec<T>, predicate: fn(&T, usize) -> bool) -> Vec<T> {
    let mut result: Vec<T> = vec![];
    for (index, item) in  iterable.iter().enumerate() {
        if predicate(&item.clone(), index) {
            result.push(item.clone());
        }
    }
    result
}

