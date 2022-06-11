use super::{
    book::{read_book, write_book},
    Book,
};

pub fn bookstore_demo() {
    let book = Book::new(
        "Livro 1".to_string(),
        "Book 1".to_string(),
        "Author 1".to_string(),
        2003,
        "USA".to_string(),
        "algo".to_string(),
    );
    write_book("data/books/0.txt", &book).expect("Error writing book");
    let readed_book = read_book("data/books/0.txt").expect("Error reading book");
    println!("{:?}", readed_book);
}
