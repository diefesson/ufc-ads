use std::rc::Rc;

// TODO: use generics
type Key = usize;
type Value = i32;

trait Node {
    fn over_limit(&self) -> bool;

    fn insert(&mut self, key: Key, value: Value) -> Option<BranchEntry>;

    fn split(&mut self) -> BranchEntry;
}

type ChildNode = Rc<dyn Node>;
struct BranchEntry {
    key: Key,
    right: ChildNode,
}

struct Branch {
    order: Key,
    left: ChildNode,
    entries: Vec<BranchEntry>,
}

impl Branch {
    fn find_insert_child(&self, key: Key) -> &mut ChildNode {
        if key < self.entries[0].key {
            return &mut self.left;
        }
        for i in 1..self.entries.len() {
            if key < self.entries[i].key {
                return &mut self.entries[i - 1].right;
            }
        }
        return &mut self.entries.last().unwrap().right;
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
        let new = child.insert(key, value);
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
        let middle = self.entries.pop().unwrap();
        let right = Branch {
            order: self.order,
            left: middle.right,
            entries: right_entries,
        };
        middle.right = Rc::new(right);
        return middle;
    }
}

struct LeafEntry {
    key: Key,
    value: Value,
}

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
        let right: Rc<dyn Node> = Rc::new(Leaf {
            order: self.order,
            entries: right_entries,
            next: self.next.take(),
        });
        self.next = Some(Rc::clone(&right));
        let middle = BranchEntry {
            key: key,
            right: right,
        };
        return middle;
    }
}

pub struct BPlusMap {
    root: Option<ChildNode>,
}
