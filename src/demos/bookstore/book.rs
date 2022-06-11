use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Debug)]
pub struct Book {
    name: String,
    original_name: String,
    author: String,
    year: i32,
    country: String,
    note: String,
}

impl Book {
    pub fn new(
        name: String,
        original_name: String,
        author: String,
        year: i32,
        country: String,
        note: String,
    ) -> Self {
        Self {
            name,
            original_name,
            author,
            year,
            country,
            note,
        }
    }
}

pub fn read_book(path: &str) -> Result<Book, Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut buffer = String::new();

    reader.read_line(&mut buffer)?;
    let name = buffer.trim().to_string();
    buffer.clear();
    reader.read_line(&mut buffer)?;
    let original_name = buffer.trim().to_string();
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
        name,
        original_name,
        author,
        year,
        country,
        note,
    })
}

pub fn write_book(path: &str, book: &Book) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    writeln!(file, "{}", book.name)?;
    writeln!(file, "{}", book.original_name)?;
    writeln!(file, "{}", book.author)?;
    writeln!(file, "{}", book.year)?;
    writeln!(file, "{}", book.country)?;
    writeln!(file, "{}", book.note)?;
    Ok(())
}
