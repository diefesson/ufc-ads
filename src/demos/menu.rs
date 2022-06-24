use crate::demos::console;
use crate::demos::DemoResult;

pub type MenuFunction<S> = fn(&mut S) -> DemoResult;

pub type MenuOption<S> = (&'static str, MenuFunction<S>);
pub struct Menu<S> {
    state: S,
    options: Vec<MenuOption<S>>,
}

impl<S> Menu<S> {
    pub fn new(state: S, options: Vec<MenuOption<S>>) -> Self {
        Self { state, options }
    }

    pub fn show(&mut self) -> DemoResult {
        loop {
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
