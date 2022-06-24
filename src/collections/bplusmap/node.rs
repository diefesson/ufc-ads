use super::BranchEntry;
use std::{any::Any, cell::RefCell, fmt::Debug, rc::Rc};

// TODO: use generics
pub type Key = usize;
pub type Value = String;

pub trait Node: Debug {
    fn over_limit(&self) -> bool;

    fn len(&self) -> usize;

    fn insert(&mut self, key: Key, value: Value) -> Option<BranchEntry>;

    fn get(&self, key: Key) -> Option<String>;

    fn update(&mut self, key: Key, value: Value) -> bool;

    fn split(&mut self) -> BranchEntry;

    fn as_any(&self) -> &dyn Any;
}

pub type ChildNode = Rc<RefCell<dyn Node>>;
