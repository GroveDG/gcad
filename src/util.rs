pub(crate) fn locate<'a, T: PartialEq>(
    iter: impl IntoIterator<Item = &'a T>,
    item: &'a T,
) -> Option<usize> {
    for (i, t) in iter.into_iter().enumerate() {
        if t == item {
            return Some(i);
        }
    }
    None
}
