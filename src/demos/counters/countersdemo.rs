use std::fmt::Display;

use super::{Client, Counter};
use crate::{
    collections::Heap,
    demos::{
        console,
        menu::{display_state, menu_option, title, Menu, MenuResult},
    },
};

const MIN_COUNTERS: usize = 3;

struct DemoState {
    counters: Vec<Counter>,
    queue: Heap<Client>,
}

impl Display for DemoState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let next_client = self.queue.peek();
        let has_priority_clients = self.queue.iter().any(|c| c.priority);
        let queue_size = self.queue.len();
        writeln!(f, "===== Bookstore Demo =====")?;
        if let Some(client) = next_client {
            writeln!(f, "Next client: {}", client)?;
        } else {
            writeln!(f, "Next client: None")?;
        }
        writeln!(f, "Priority clients: {}", has_priority_clients)?;
        writeln!(f, "Queue size: {}", queue_size)?;
        writeln!(f, "Counters:")?;
        for (index, counter) in self.counters.iter().enumerate() {
            writeln!(f, "[{:>2}]. {}", index, counter)?;
        }
        Ok(())
    }
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

pub fn counters_demo() -> MenuResult {
    let mut counters_menu = Menu::new(
        display_state(),
        DemoState::new(),
        vec![
            menu_option("Call next", |s| call_next(s)),
            menu_option("Add counter", |s| add_counter(s)),
            menu_option("Remove counter", |s| remove_counter(s)),
        ],
    );
    counters_menu.show()
}

fn call_next(demo_state: &mut DemoState) -> MenuResult {
    let client = demo_state.queue.peek();
    match client {
        Some(client) if client.priority => {
            if let Some(counter) = demo_state.counters.iter_mut().find(|c| !c.in_use()) {
                counter.serve(demo_state.queue.next().unwrap());
            } else {
                println!("No counter to serve the priority client");
            }
        }
        Some(_) => {
            if let Some(counter) = demo_state.counters[1..].iter_mut().find(|c| !c.in_use()) {
                counter.serve(demo_state.queue.next().unwrap());
            } else if !demo_state.counters[0].in_use() {
                if demo_state.queue.iter().any(|c| c.priority) {
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
    console::pause();
    Ok(())
}

fn add_counter(demo_state: &mut DemoState) -> MenuResult {
    demo_state.counters.push(Counter::new());
    println!("Added counter");
    console::pause();
    Ok(())
}

fn remove_counter(demo_state: &mut DemoState) -> MenuResult {
    if demo_state.counters.len() > MIN_COUNTERS {
        demo_state.counters.pop();
        println!("Removed counter")
    } else {
        println!("The min number of counters is {}", MIN_COUNTERS);
    }
    console::pause();
    Ok(())
}
