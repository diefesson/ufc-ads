use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub original_title: String,
    pub author: String,
    pub year: i32,
    pub country: String,
    pub note: String,
}

pub fn read_book(path: &str) -> Result<Book, Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut buffer = String::new();

    reader.read_line(&mut buffer)?;
    let title = buffer.trim().to_string();
    buffer.clear();
    reader.read_line(&mut buffer)?;
    let original_title = buffer.trim().to_string();
    buffer.clear();
    reader.read_line(&mut buffer)?;
    let author = buffer.trim().to_string();
    buffer.clear();
    reader.read_line(&mut buffer)?;
    let year = buffer.trim().parse()?;
    buffer.clear();
    reader.read_line(&mut buffer)?;
    let country = buffer.trim().to_string();
    buffer.clear();
    reader.read_line(&mut buffer)?;
    let note = buffer.trim().to_string();
    buffer.clear();

    Ok(Book {
        title,
        original_title,
        author,
        year,
        country,
        note,
    })
}

pub fn write_book(path: &str, book: &Book) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    writeln!(file, "{}", book.title)?;
    writeln!(file, "{}", book.original_title)?;
    writeln!(file, "{}", book.author)?;
    writeln!(file, "{}", book.year)?;
    writeln!(file, "{}", book.country)?;
    writeln!(file, "{}", book.note)?;
    Ok(())
}
