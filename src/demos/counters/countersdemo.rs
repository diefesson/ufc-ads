use super::{Client, Counter};
use crate::{
    collections::Heap,
    demos::{
        menu::{Menu, MenuOption},
        DemoResult,
    },
};

const MIN_COUNTERS: usize = 3;

struct DemoState {
    counters: Vec<Counter>,
    queue: Heap<Client>,
}

impl DemoState {
    fn new() -> Self {
        let counters = (0..MIN_COUNTERS).map(|_| Counter::new()).collect();
        Self {
            counters,
            queue: Heap::new(),
        }
    }
}

pub fn counters_demo(_: &mut ()) -> DemoResult {
    let state = DemoState::new();
    let options: Vec<MenuOption<_>> = vec![("Call next", call_next)];
    let mut counters_menu = Menu::new(state, options);
    counters_menu.show()
}

fn call_next(demo_state: &mut DemoState) -> DemoResult {
    let client = demo_state.queue.peek();
    match client {
        Some(client) if client.priority => {
            if let Some(counter) = demo_state.counters.iter_mut().find(|c| !c.in_use()) {
                counter.serve(demo_state.queue.next().unwrap());
            } else {
                println!("No counter to serve the priority client");
            }
        }
        Some(client) => {
            if let Some(counter) = demo_state.counters[1..].iter_mut().find(|c| !c.in_use()) {
                counter.serve(demo_state.queue.next().unwrap());
            } else if !demo_state.counters[0].in_use() {
                if demo_state.queue.iter().find(|c| c.priority).is_none() {
                    demo_state.counters[0].serve(demo_state.queue.next().unwrap());
                } else {
                    println!("The only free counter is for priority clients, but the queue has priority clients");
                }
            } else {
                println!("No counter to serve the non priority client");
            }
        }
        _ => println!("The queue is empty"),
    }
    Ok(())
}
