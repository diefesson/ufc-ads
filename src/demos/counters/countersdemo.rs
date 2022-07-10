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

impl DemoState {
    fn new(counters_count: usize) -> Self {
        let counters = (0..counters_count).map(|_| Counter::new()).collect();
        Self {
            counters,
            queue: Heap::new(),
            next_check_in: 0,
        }
    }
}

impl Display for DemoState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let next_client = self.queue.peek();
        let has_priority_clients = self
            .queue
            .iter()
            .any(|c| c.priority == PriorityType::Priority);
        let queue_size = self.queue.len();
        writeln!(f, "===== Counters Demo (heap) =====")?;
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

pub fn counters_demo() -> MenuResult {
    let mut counters_menu = Menu::new(
        display_state(),
        DemoState::new(MIN_COUNTERS),
        vec![
            menu_option("Emit ticket", |s| emit_ticket(s)),
            menu_option("Call next", |s| call_next(s)),
            menu_option("Finalize service", |s| finalize_service(s)),
            menu_option("Open counter", |s| open_counter(s)),
            menu_option("Close counter", |s| close_counter(s)),
        ],
    );
    counters_menu.show()
}

fn call_next(demo_state: &mut DemoState) -> MenuResult {
    let client = demo_state.queue.peek();
    match client {
        Some(client) if client.priority == PriorityType::Priority => {
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
            let has_priority = demo_state
                .queue
                .iter()
                .any(|c| c.priority == PriorityType::Priority);
            let counter = demo_state
                .counters
                .iter_mut()
                .enumerate()
                .skip(has_priority as usize)
                .find(|(_, c)| !c.in_use());
            if let Some((index, counter)) = counter {
                counter.serve(demo_state.queue.next().unwrap());
                println!("Non priority client directed to counter {}", index);
            } else if has_priority && !demo_state.counters[0].in_use() {
                println!("The only free counter is already reserved for priority clients in queue");
            } else {
                println!("No counter to serve the non priority client");
            }
        }
        _ => println!("The queue is empty"),
    }
    console::pause();
    Ok(())
}

fn open_counter(demo_state: &mut DemoState) -> MenuResult {
    demo_state.counters.push(Counter::new());
    println!("Added counter");
    console::pause();
    Ok(())
}

fn close_counter(demo_state: &mut DemoState) -> MenuResult {
    if demo_state.counters.len() > MIN_COUNTERS {
        let index = demo_state
            .counters
            .iter()
            .enumerate()
            .find(|(_, c)| !c.in_use())
            .map(|(i, _)| i);
        if let Some(index) = index {
            demo_state.counters.remove(index);
            println!("Removed counter {}", index);
        } else {
            println!("All counters are in use");
        }
    } else {
        println!("The min number of counters is {}", MIN_COUNTERS);
    }
    console::pause();
    Ok(())
}

fn emit_ticket(demo_state: &mut DemoState) -> MenuResult {
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
