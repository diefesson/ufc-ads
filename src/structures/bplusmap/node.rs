use super::BranchEntry;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

// TODO: use generics
pub type Key = usize;
pub type Value = String;

pub trait Node: Debug {
    fn over_limit(&self) -> bool;

    fn insert(&mut self, key: Key, value: Value) -> Option<BranchEntry>;

    fn split(&mut self) -> BranchEntry;
}

pub type ChildNode = Rc<RefCell<dyn Node>>;
