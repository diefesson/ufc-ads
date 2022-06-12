mod demos;
mod structures;

use demos::bookstore_demo;
use std::{io::stdin, process::exit};

fn bye() {
    exit(0);
}

fn main() {
    let demos: Vec<(&str, fn() -> ())> = vec![("Bookstore", bookstore_demo), ("Exit", bye)];
    let mut buffer = String::new();
    for (index, (name, _)) in demos.iter().enumerate() {
        println!("{} - {}", index, name);
    }
    println!("Enter demo number");
    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();
    let option = buffer.trim().parse::<usize>();
    match option {
        Ok(index) if index < demos.len() => {
            demos[index].1();
        }
        _ => {
            println!("Invalid demo number: {}", buffer.trim());
        }
    }
}
