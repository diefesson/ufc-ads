pub fn parent(position: usize) -> usize {
    (position - 1) / 2
}

pub fn left(position: usize) -> usize {
    2 * position + 1
}

pub fn right(position: usize) -> usize {
    2 * position + 2
}
