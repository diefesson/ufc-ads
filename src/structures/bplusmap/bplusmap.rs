use super::{Branch, ChildNode, Key, Leaf, LeafEntry, Value};
use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug)]
pub struct BPlusMap {
    order: usize,
    root: Option<ChildNode>,
}

impl BPlusMap {
    pub fn new(order: usize) -> Self {
        Self {
            order: order,
            root: None,
        }
    }

    pub fn insert(&mut self, key: Key, value: Value) {
        if let Some(root) = &self.root {
            let middle = root.borrow_mut().insert(key, value);
            if let Some(middle) = middle {
                self.root = Some(Rc::new(RefCell::new(Branch {
                    order: self.order,
                    entries: vec![middle],
                    left: Rc::clone(root),
                })));
            }
        } else {
            self.root = Some(Rc::new(RefCell::new(Leaf {
                order: self.order,
                entries: vec![LeafEntry { key, value }],
                next: None,
            })));
        }
    }

    pub fn update(&mut self, key: Key, value: Value) -> bool {
        if let Some(root) = &self.root {
            root.borrow_mut().update(key, value)
        } else {
            false
        }
    }
}
