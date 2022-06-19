use super::{
    leaf::Leaf,
    node::{ChildNode, Key, Value},
};
use std::{cell::RefCell, rc::Rc};

pub struct BPMIter {
    leaf: Option<ChildNode>,
    index: usize,
}

impl BPMIter {
    pub fn new(leaf: Option<ChildNode>) -> Self {
        Self { leaf, index: 0 }
    }

    fn next_leaf(&self) -> Option<ChildNode> {
        let borrow = RefCell::borrow(self.leaf.as_ref().unwrap());
        let leaf = borrow.as_any().downcast_ref::<Leaf>().unwrap();
        leaf.next.as_ref().map(Rc::clone)
    }

    fn advance(&mut self) {
        self.index += 1;
        if self.index >= self.leaf.as_ref().unwrap().borrow().len() {
            self.leaf = self.next_leaf();
            self.index = 0;
        }
    }
}

impl Iterator for BPMIter {
    type Item = (Key, Value);

    fn next(&mut self) -> Option<Self::Item> {
        if self.leaf.is_some() {
            let key;
            let value;
            {
                let borrow = RefCell::borrow(self.leaf.as_ref().unwrap());
                let leaf = borrow.as_any().downcast_ref::<Leaf>().unwrap();
                let entry = &leaf.entries[self.index];
                key = entry.key;
                value = entry.value.clone();
            }
            self.advance();
            Some((key, value))
        } else {
            None
        }
    }
}
