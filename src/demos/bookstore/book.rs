use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub original_title: String,
    pub author: String,
    pub year: i32,
    pub country: String,
    pub note: String,
}

impl Book {
    pub fn read(path: &Path) -> Result<Book, Box<dyn Error>> {
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

    pub fn write(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path)?;
        writeln!(file, "{}", self.title)?;
        writeln!(file, "{}", self.original_title)?;
        writeln!(file, "{}", self.author)?;
        writeln!(file, "{}", self.year)?;
        writeln!(file, "{}", self.country)?;
        writeln!(file, "{}", self.note)?;
        Ok(())
    }
}
