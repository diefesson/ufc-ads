use super::book::{write_book, Book};
use crate::structures::BPlusMap;

pub struct BookRepository {
    current_id: usize,
    root: String,
    mapping: BPlusMap,
}

impl BookRepository {
    pub fn new(root: String) -> Self {
        Self {
            current_id: 0,
            root: root,
            mapping: BPlusMap::new(2),
        }
    }

    pub fn add(&mut self, book: Book) {
        let filename = generate_filename(self.current_id, &book);
        let path = format!("{}/books/{}", self.root, filename);
        self.mapping.insert(self.current_id, filename);
        write_book(&path, &book).unwrap();
    }
}

fn generate_filename(id: usize, book: &Book) -> String {
    format!("{}-{}-{}.txt", id, book.author, book.title)
}
