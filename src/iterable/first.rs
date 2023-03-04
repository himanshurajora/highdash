use super::head;

pub fn first<T>(iterable: &[T]) -> Option<&T> {
    head(iterable)
}
