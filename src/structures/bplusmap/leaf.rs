use super::{BranchEntry, ChildNode, Key, Node, Value};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct LeafEntry {
    pub key: Key,
    pub value: Value,
}

#[derive(Debug)]
pub struct Leaf {
    pub order: Key,
    pub next: Option<ChildNode>,
    pub entries: Vec<LeafEntry>,
}

impl Leaf {
    fn insert_entry(&mut self, entry: LeafEntry) {
        for i in 0..self.entries.len() {
            if entry.key < self.entries[i].key {
                self.entries.insert(i, entry);
                return;
            }
        }
        self.entries.push(entry);
    }
}

impl Node for Leaf {
    fn over_limit(&self) -> bool {
        self.entries.len() > 2 * self.order
    }

    fn insert(&mut self, key: Key, value: Value) -> Option<BranchEntry> {
        self.insert_entry(LeafEntry { key, value });
        if self.over_limit() {
            return Some(self.split());
        }
        return None;
    }

    fn split(&mut self) -> BranchEntry {
        let right_entries = self.entries.split_off(self.order);
        let key = right_entries[0].key;
        let right: Rc<RefCell<dyn Node>> = Rc::new(RefCell::new(Leaf {
            order: self.order,
            entries: right_entries,
            next: self.next.take(),
        }));
        self.next = Some(Rc::clone(&right));
        return BranchEntry {
            key: key,
            right: right,
        };
    }
}
