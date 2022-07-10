mod collections;
mod demos;

use demos::counters::counters_demo;
use demos::menu::{menu_option, title};
use demos::studygroups::study_groups_demo;

use crate::demos::bookstore::bookstore_demo;
use crate::demos::menu::{Menu, MenuResult};

fn main() -> MenuResult {
    let state = ();
    let mut demo_player = Menu::new(
        title("Select a Demo"),
        state,
        vec![
            menu_option("Bookstore", |_| bookstore_demo()),
            menu_option("Counters", |_| counters_demo()),
            menu_option("Study groups", |_| study_groups_demo()),
        ],
    );
    demo_player.show()
}
