use crate::demos::bookstore::{Book, BookRepository};
use crate::demos::console;
use crate::demos::menu::{menu_option, title, Menu, MenuResult};

const ROOT: &str = "data";

pub fn bookstore_demo() -> MenuResult {
    let mut menu = Menu::new(
        title("Bookstore Demo"),
        BookRepository::new(ROOT.into())?,
        vec![
            menu_option("Add book", |s| add_book(s)),
            menu_option("Find book", |s| find_book(s)),
            menu_option("Update book note", |s| update_book_note(s)),
            menu_option("List books", |s| list_books(s)),
        ],
    );
    menu.show()
}

fn add_book(book_repository: &mut BookRepository) -> MenuResult {
    println!("Title:");
    let title = console::read_line();
    if title.is_empty() {
        println!("Title cannot be empty");
        return Ok(());
    }
    println!("Original title:");
    let original_title = console::read_line();
    if original_title.is_empty() {
        println!("Original title should not be empty");
        return Ok(());
    }
    println!("Author:");
    let author = console::read_line();
    if author.is_empty() {
        println!("Author should not be empty");
        return Ok(());
    }
    println!("Year:");
    let year = console::parse_line();
    if year.is_err() {
        println!("Year should not be empty");
        return Ok(());
    }
    let year = year.unwrap();
    println!("Country:");
    let country = console::read_line();
    if country.is_empty() {
        println!("Country should not be empty");
        return Ok(());
    }
    println!("Note (optional):");
    let note = console::read_line();
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

fn find_book(book_repository: &BookRepository) -> MenuResult {
    println!("Book id:");
    let id = console::parse_line()?;
    let book = book_repository.find(id)?;
    if let Some(book) = book {
        println!("Book {}", book);
    } else {
        println!("Book not found");
    }
    Ok(())
}

fn list_books(book_repository: &BookRepository) -> MenuResult {
    for entry in book_repository.iter() {
        let (id, book) = entry?;
        println!("Book {}: {}", id, book);
    }
    Ok(())
}

fn update_book_note(book_repository: &BookRepository) -> MenuResult {
    println!("Book id:");
    let id = console::parse_line()?;
    let book = book_repository.find(id)?;
    if let Some(mut book) = book {
        println!("Book: {}", book);
        println!("New note:");
        let note = console::read_line();
        book.note = note;
        book_repository.update(id, book)?;
    } else {
        println!("Book not found");
    }
    Ok(())
}
