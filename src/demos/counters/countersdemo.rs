use std::fmt::Display;

use super::{client::PriorityType, Client, Counter};
use crate::collections::Heap;
use crate::demos::console;
use crate::demos::menu::{display_state, menu_option, Menu, MenuResult};

const MIN_COUNTERS: usize = 3;

struct DemoState {
    counters: Vec<Counter>,
    queue: Heap<Client>,
    next_check_in: u32,
}

impl Display for DemoState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let next_client = self.queue.peek();
        let has_priority_clients = self
            .queue
            .iter()
            .any(|c| c.priority == PriorityType::PRIORITY);
        let queue_size = self.queue.len();
        writeln!(f, "===== Bookstore Demo =====")?;
        writeln!(f)?;
        writeln!(f, "Queue:")?;
        if let Some(client) = next_client {
            writeln!(f, "\tNext client: {}", client)?;
        } else {
            writeln!(f, "\tNext client: None")?;
        }
        writeln!(f, "\tPriority clients: {}", has_priority_clients)?;
        writeln!(f, "\tQueue size: {}", queue_size)?;
        writeln!(f, "Counters:")?;
        for (index, counter) in self.counters.iter().enumerate() {
            writeln!(f, "\t[{:>2}]. {}", index, counter)?;
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
            next_check_in: 0,
        }
    }
}

pub fn counters_demo() -> MenuResult {
    let mut counters_menu = Menu::new(
        display_state(),
        DemoState::new(),
        vec![
            menu_option("Add client", |s| add_client(s)),
            menu_option("Finalize service", |s| finalize_service(s)),
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
        Some(client) if client.priority == PriorityType::PRIORITY => {
            let counter = demo_state
                .counters
                .iter_mut()
                .enumerate()
                .find(|(_, c)| !c.in_use());
            if let Some((index, counter)) = counter {
                counter.serve(demo_state.queue.next().unwrap());
                println!("Priority client directed to counter {}", index);
            } else {
                println!("No counter to serve the priority client");
            }
        }
        Some(_) => {
            let counter = demo_state.counters[1..]
                .iter_mut()
                .enumerate()
                .find(|(_, c)| !c.in_use());
            if let Some((index, counter)) = counter {
                counter.serve(demo_state.queue.next().unwrap());
                println!("Non priority client directed to counter {}", index);
            } else if !demo_state.counters[0].in_use() {
                if demo_state
                    .queue
                    .iter()
                    .any(|c| c.priority == PriorityType::PRIORITY)
                {
                    demo_state.counters[0].serve(demo_state.queue.next().unwrap());
                    println!("Non priority client directed to counter 0");
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

fn add_client(demo_state: &mut DemoState) -> MenuResult {
    println!("Priority (normal/priority):");
    let priority = console::parse_line();
    if priority.is_err() {
        println!("Invalid priority");
        console::pause();
        return Ok(());
    }
    println!("Service type (particular/business):");
    let service_type = console::parse_line();
    if service_type.is_err() {
        println!("Invalid service type");
        console::pause();
        return Ok(());
    }
    let check_in = demo_state.next_check_in;
    demo_state.next_check_in += 1;
    let client = Client {
        priority: priority.unwrap(),
        check_in,
        service_type: service_type.unwrap(),
    };
    demo_state.queue.insert(client);
    println!("Client added with check in {}", check_in);
    console::pause();
    Ok(())
}

fn finalize_service(demo_state: &mut DemoState) -> MenuResult {
    println!("Counter index: ");
    let index = console::parse_line::<usize>();
    if let Ok(index) = index {
        if index < demo_state.counters.len() {
            let counter = &mut demo_state.counters[index];
            if counter.in_use() {
                demo_state.counters[index].finalize_service();
                println!("Service on counter {} finalized", index);
            } else {
                println!("The counter {} is not in use", index);
            }
            console::pause();
            return Ok(());
        }
    }
    println!("Invalid counter index");
    console::pause();
    Ok(())
}
