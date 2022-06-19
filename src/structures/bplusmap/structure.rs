use super::{BPMIter, Branch, ChildNode, Key, Leaf, LeafEntry, Value};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct BPlusMap {
    order: usize,
    root: Option<ChildNode>,
}

impl BPlusMap {
    pub fn new(order: usize) -> Self {
        Self { order, root: None }
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

    pub fn get(&self, key: Key) -> Option<Value> {
        if let Some(root) = &self.root {
            root.borrow().get(key)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self, key: Key, value: Value) -> bool {
        if let Some(root) = &self.root {
            root.borrow_mut().update(key, value)
        } else {
            false
        }
    }

    fn first_leaf(&self) -> Option<ChildNode> {
        if let Some(root) = &self.root {
            let mut current = Rc::clone(root);
            let mut next: ChildNode;
            loop {
                {
                    let borrow = current.borrow();
                    let node = borrow.as_any();
                    if node.is::<Leaf>() {
                        break;
                    }
                    next = Rc::clone(&node.downcast_ref::<Branch>().unwrap().left);
                }
                current = next;
            }
            Some(current)
        } else {
            None
        }
    }

    pub fn iter(&self) -> BPMIter {
        BPMIter::new(self.first_leaf())
    }
}
