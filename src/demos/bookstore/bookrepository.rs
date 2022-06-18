use super::book::Book;
use crate::structures::BPlusMap;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

pub struct BookRepository {
    current_id: usize,
    root: PathBuf,
    mapping: BPlusMap,
}

impl BookRepository {
    pub fn new(root: PathBuf) -> Result<Self, Box<dyn Error>> {
        let mut new = Self {
            current_id: 0,
            root: root,
            mapping: BPlusMap::new(2),
        };
        new.load_index()?;
        Ok(new)
    }

    pub fn add(&mut self, book: Book) -> Result<(), Box<dyn Error>> {
        let filename = book_name(self.current_id, &book);
        let path = book_path(&self.root, &filename);
        self.mapping
            .insert(self.current_id, filename.to_str().unwrap().to_string());
        self.current_id += 1;
        book.write(&path)?;
        self.save_index()?;
        Ok(())
    }

    pub fn find(&self, id: usize) -> Result<Option<Book>, Box<dyn Error>> {
        let filename = self.mapping.get(id);
        if let Some(filename) = filename {
            let path = book_path(&self.root, PathBuf::from(filename).as_path());
            let book = Book::read(&path)?;
            Ok(Some(book))
        } else {
            Ok(None)
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Result<Book, Box<dyn Error>>> + '_ {
        self.mapping.iter().map(|(_, filename)| {
            let path = book_path(self.root.as_path(), PathBuf::from(filename).as_path());
            Book::read(&path)
        })
    }

    fn save_index(&self) -> Result<(), Box<dyn Error>> {
        let path = self.root.join("index.txt");
        let file = File::create(&path)?;
        let mut writer = BufWriter::new(file);
        writeln!(&mut writer, "{}", self.current_id)?;
        for (id, filename) in self.mapping.iter() {
            writeln!(&mut writer, "{}|{}", id, filename)?;
        }
        Ok(())
    }

    fn load_index(&mut self) -> Result<(), Box<dyn Error>> {
        if let Ok(file) = File::open(&self.root.join("index.txt")) {
            let mut reader = BufReader::new(file);
            let mut buffer = String::new();
            reader.read_line(&mut buffer)?;
            self.current_id = buffer.trim().parse()?;
            for line in reader.lines() {
                if let Some((id, filename)) = line?.split_once("|") {
                    self.mapping.insert(id.parse()?, filename.to_string());
                } else {
                    Err("incorrectly formated line in index file")?
                }
            }
        }
        Ok(())
    }
}

fn book_name(id: usize, book: &Book) -> PathBuf {
    let path = format!("{}-{}-{}.txt", id, book.author, book.title);
    path.into()
}

fn book_path(root: &Path, book_filename: &Path) -> PathBuf {
    PathBuf::from(root).join("books").join(book_filename)
}
