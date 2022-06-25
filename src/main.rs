mod collections;
mod demos;

use demos::counters::counters_demo;

use crate::demos::bookstore::bookstore_demo;
use crate::demos::menu::{Menu, MenuOption};
use crate::demos::DemoResult;

fn main() -> DemoResult {
    let state = ();
    let options: Vec<MenuOption<_>> =
        vec![("Bookstore", bookstore_demo), ("Counters", counters_demo)];
    let mut demo_player = Menu::new(state, options);
    demo_player.show()
}
