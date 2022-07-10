use std::error;
use std::fmt::Display;

use crate::demos::console;

pub type MenuResult = Result<(), Box<dyn error::Error>>;

pub type MenuFunction<S> = Box<dyn Fn(&mut S) -> MenuResult>;

pub type MenuOption<S> = (&'static str, MenuFunction<S>);

pub type MenuDisplayer<S> = Box<dyn Fn(&S)>;
pub struct Menu<S> {
    displayer: MenuDisplayer<S>,
    state: S,
    options: Vec<MenuOption<S>>,
}

impl<S> Menu<S> {
    pub fn new(displayer: MenuDisplayer<S>, state: S, options: Vec<MenuOption<S>>) -> Self {
        Self {
            state,
            displayer,
            options,
        }
    }

    pub fn show(&mut self) -> MenuResult {
        loop {
            (self.displayer)(&self.state);
            println!("");
            for (index, (name, _)) in self.options.iter().enumerate() {
                println!("{}. {}", index, name);
            }
            println!("{}. Exit", self.options.len());
            let index = console::parse_line::<usize>();
            match index {
                Ok(index) if index < self.options.len() => self.options[index].1(&mut self.state)?,
                Ok(index) if index == self.options.len() => return Ok(()),
                _ => println!("Invalid option"),
            };
        }
    }
}

pub fn title<S>(title: &'static str) -> MenuDisplayer<S> {
    Box::new(move |_| {
        println!("===== {} =====", title);
    })
}

pub fn display_state<S: Display>() -> MenuDisplayer<S> {
    Box::new(|state| print!("{}", state))
}

pub fn menu_option<S>(
    name: &'static str,
    handler: impl Fn(&mut S) -> MenuResult + 'static,
) -> MenuOption<S> {
    (name, Box::new(handler))
}

pub fn simple_option<S>(name: &'static str, handler: impl Fn(&mut S) + 'static) -> MenuOption<S> {
    (
        name,
        Box::new(move |state| {
            handler(state);
            Ok(())
        }),
    )
}
