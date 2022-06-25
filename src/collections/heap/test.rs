#![cfg(test)]

use super::Heap;

#[test]
fn heap_ordering() {
    const UNTIL: i32 = 1000;
    let mut heap = Heap::new();
    for i in 0..UNTIL {
        heap.insert(i);
    }
    for i in (0..UNTIL).rev() {
        assert_eq!(i, heap.next().unwrap(), "unexpected value for ordering");
    }
}
