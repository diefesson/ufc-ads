use super::{ChildNode, Key, Node, Value};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct BranchEntry {
    pub key: Key,
    pub right: ChildNode,
}

#[derive(Debug)]
pub struct Branch {
    pub order: Key,
    pub left: ChildNode,
    pub entries: Vec<BranchEntry>,
}

impl Branch {
    fn find_child(&self, key: Key) -> ChildNode {
        if key < self.entries[0].key {
            return Rc::clone(&self.left);
        }
        for i in 1..self.entries.len() {
            if key < self.entries[i].key {
                return Rc::clone(&self.entries[i - 1].right);
            }
        }
        return Rc::clone(&self.entries.last().unwrap().right);
    }

    fn insert_entry(&mut self, entry: BranchEntry) {
        for i in 0..self.entries.len() {
            if entry.key < self.entries[i].key {
                self.entries.insert(i, entry);
                return;
            }
        }
        self.entries.push(entry);
    }
}

impl Node for Branch {
    fn over_limit(&self) -> bool {
        self.entries.len() > 2 * self.order
    }

    fn insert(&mut self, key: Key, value: Value) -> Option<BranchEntry> {
        let child = self.find_child(key);
        let new = child.borrow_mut().insert(key, value);
        if let Some(entry) = new {
            self.insert_entry(entry);
            if self.over_limit() {
                return Some(self.split());
            }
        }
        None
    }

    fn get(&self, key: Key) -> Option<String> {
        let child = self.find_child(key);
        let value = child.borrow().get(key);
        value
    }

    fn update(&mut self, key: Key, value: Value) -> bool {
        let child = self.find_child(key);
        return child.borrow_mut().update(key, value);
    }

    fn split(&mut self) -> BranchEntry {
        let right_entries = self.entries.split_off(self.order + 1);
        let mut middle = self.entries.pop().unwrap();
        let right = Branch {
            order: self.order,
            left: middle.right,
            entries: right_entries,
        };
        middle.right = Rc::new(RefCell::new(right));
        middle
    }

    fn len(&self) -> usize {
        self.entries.len()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
