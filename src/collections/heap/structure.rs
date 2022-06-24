use super::{left, parent, right};

pub struct Heap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> Heap<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn insert(&mut self, value: T) {
        self.data.push(value);
        self.up(self.data.len() - 1);
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn next(&mut self) -> T {
        let value = self.data.swap_remove(0);
        if !self.is_empty() {
            self.down(0);
        }
        value
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn down(&mut self, mut position: usize) {
        loop {
            let left = left(position);
            let right = right(position);
            if self.exists(right) && self.data[right] > self.data[position] {
                if self.data[left] > self.data[right] {
                    self.data.swap(left, position);
                    position = left;
                } else {
                    self.data.swap(right, position);
                    position = right;
                }
            } else if self.exists(left) && self.data[left] > self.data[position] {
                self.data.swap(left, position);
                position = left;
            } else {
                break;
            }
        }
    }

    fn up(&mut self, mut position: usize) {
        while position > 0 && self.data[parent(position)] < self.data[position] {
            self.data.swap(parent(position), position);
            position = parent(position);
        }
    }

    fn exists(&self, position: usize) -> bool {
        position < self.data.len()
    }
}
