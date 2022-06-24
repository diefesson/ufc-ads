use crate::demos::bookstore::{Book, BookRepository};
use crate::demos::console::{parse_line, read_line};
use crate::demos::DemoResult;

const ROOT: &str = "data";

fn show_options() {
    let options = vec![
        "Add book",
        "Find book",
        "Update book note",
        "List books",
        "Exit",
    ];
    for (index, name) in options.iter().enumerate() {
        println!("{} - {}", index, name);
    }
}

pub fn bookstore_demo() -> DemoResult {
    let mut book_repository = BookRepository::new(ROOT.into())?;
    loop {
        show_options();
        match parse_line() {
            Ok(0) => add_book(&mut book_repository)?,
            Ok(1) => find_book(&book_repository)?,
            Ok(2) => {
                update_book_note(&book_repository)?;
            }
            Ok(3) => list_books(&book_repository)?,
            Ok(4) => return Ok(()),
            _ => println!("Invalid option:"),
        }
    }
}

fn add_book(book_repository: &mut BookRepository) -> DemoResult {
    println!("Title:");
    let title = read_line();
    if title.is_empty() {
        println!("Title cannot be empty");
        return Ok(());
    }
    println!("Original title:");
    let original_title = read_line();
    if original_title.is_empty() {
        println!("Original title should not be empty");
        return Ok(());
    }
    println!("Author:");
    let author = read_line();
    if author.is_empty() {
        println!("Author should not be empty");
        return Ok(());
    }
    println!("Year:");
    let year = parse_line();
    if year.is_err() {
        println!("Year should not be empty");
        return Ok(());
    }
    let year = year.unwrap();
    println!("Country:");
    let country = read_line();
    if country.is_empty() {
        println!("Country should not be empty");
        return Ok(());
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
    book_repository.add(book)?;
    Ok(())
}

fn find_book(book_repository: &BookRepository) -> DemoResult {
    println!("Book id:");
    let id = parse_line()?;
    let book = book_repository.find(id)?;
    if let Some(book) = book {
        println!("Book {}", book);
    } else {
        println!("Book not found");
    }
    Ok(())
}

fn list_books(book_repository: &BookRepository) -> DemoResult {
    for entry in book_repository.iter() {
        let (id, book) = entry?;
        println!("Book {}: {}", id, book);
    }
    Ok(())
}

fn update_book_note(book_repository: &BookRepository) -> DemoResult {
    println!("Book id:");
    let id = parse_line()?;
    let book = book_repository.find(id)?;
    if let Some(mut book) = book {
        println!("Book: {}", book);
        println!("New note:");
        let note = read_line();
        book.note = note;
        book_repository.update(id, book)?;
    } else {
        println!("Book not found");
    }
    Ok(())
}
