use super::book::{read_book, write_book, Book};
use crate::structures::BPlusMap;
use std::{
    error::Error,
    path::{Path, PathBuf},
};

pub struct BookRepository {
    current_id: usize,
    root: PathBuf,
    mapping: BPlusMap,
}

impl BookRepository {
    pub fn new(root: PathBuf) -> Self {
        Self {
            current_id: 0,
            root: root,
            mapping: BPlusMap::new(2),
        }
    }

    pub fn add(&mut self, book: Book) -> Result<(), Box<dyn Error>> {
        let filename = book_name(self.current_id, &book);
        let path = book_path(&self.root, &filename);
        self.mapping
            .insert(self.current_id, filename.to_str().unwrap().to_string());
        self.current_id += 1;
        write_book(&path, &book)?;
        Ok(())
    }

    pub fn find(&self, id: usize) -> Result<Option<Book>, Box<dyn Error>> {
        let filename = self.mapping.get(id);
        if let Some(filename) = filename {
            let path = book_path(&self.root, PathBuf::from(filename).as_path());
            let book = read_book(&path)?;
            Ok(Some(book))
        } else {
            Ok(None)
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Result<Book, Box<dyn Error>>> + '_ {
        self.mapping.iter().map(|filename| {
            let path = book_path(self.root.as_path(), PathBuf::from(filename).as_path());
            read_book(&path)
        })
    }
}

fn book_name(id: usize, book: &Book) -> PathBuf {
    let path = format!("{}-{}-{}.txt", id, book.author, book.title);
    path.into()
}

fn book_path(root: &Path, book_filename: &Path) -> PathBuf {
    PathBuf::from(root).join(book_filename)
}
