use std::{cell::RefCell, fmt::Debug, rc::Rc};

// TODO: use generics
type Key = usize;
type Value = String;

trait Node: Debug {
    fn over_limit(&self) -> bool;

    fn insert(&mut self, key: Key, value: Value) -> Option<BranchEntry>;

    fn split(&mut self) -> BranchEntry;
}

type ChildNode = Rc<RefCell<dyn Node>>;

#[derive(Debug)]
struct BranchEntry {
    key: Key,
    right: ChildNode,
}

#[derive(Debug)]
struct Branch {
    order: Key,
    left: ChildNode,
    entries: Vec<BranchEntry>,
}

impl Branch {
    fn find_insert_child(&mut self, key: Key) -> ChildNode {
        if key < self.entries[0].key {
            return Rc::clone(&self.left);
        }
        for i in 1..self.entries.len() {
            if key < self.entries[i].key {
                return Rc::clone(&mut self.entries[i - 1].right);
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
        let child = self.find_insert_child(key);
        let new = child.borrow_mut().insert(key, value);
        if let Some(entry) = new {
            self.insert_entry(entry);
            if self.over_limit() {
                return Some(self.split());
            }
        }
        return None;
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
        return middle;
    }
}

#[derive(Debug)]
struct LeafEntry {
    key: Key,
    value: Value,
}

#[derive(Debug)]
struct Leaf {
    order: Key,
    next: Option<ChildNode>,
    entries: Vec<LeafEntry>,
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
}
