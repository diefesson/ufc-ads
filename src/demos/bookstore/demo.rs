use crate::demos::DemoResult;

use super::book::Book;
use super::bookrepository::BookRepository;
use super::utils::{read_line, read_selection, read_year};

const ROOT: &str = "data";

fn show_options() {
    let options = vec!["Add book", "Update book", "List books", "Exit"];
    for (index, name) in options.iter().enumerate() {
        println!("{} - {}", index, name);
    }
}

pub fn bookstore_demo() -> DemoResult {
    let mut book_repository = BookRepository::new(ROOT.into());
    loop {
        show_options();
        match read_selection() {
            Ok(0) => {
                add_book(&mut book_repository);
            }
            Ok(1) => {
                // TODO: impl book update
                todo!("Not implemented");
            }
            Ok(2) => {
                list_books(&book_repository)?;
            }
            Ok(3) => return Ok(()),
            _ => {
                println!("Invalid option:");
            }
        }
    }
}

fn add_book(book_repository: &mut BookRepository) {
    println!("Title:");
    let title = read_line();
    if title.is_empty() {
        println!("Title cannot be empty");
        return;
    }
    println!("Original title:");
    let original_title = read_line();
    if original_title.is_empty() {
        println!("Original title should not be empty");
        return;
    }
    println!("Author:");
    let author = read_line();
    if author.is_empty() {
        println!("Author should not be empty");
        return;
    }
    println!("Year:");
    let year = read_year();
    if year.is_err() {
        println!("Year should not be empty");
        return;
    }
    let year = year.unwrap();
    println!("Country:");
    let country = read_line();
    if country.is_empty() {
        println!("Country should not be empty");
        return;
    }
    println!("Note (optional):");
    let note = read_line();
    let book = Book {
        title,
        original_title,
        author,
        year,
        country,
        note,
    };
    book_repository.add(book);
}

fn list_books(book_repository: &BookRepository) -> DemoResult {
    for book in book_repository.iter() {
        println!("{:?}", book?);
    }
    Ok(())
}
