use std::{cell::RefCell, fmt::Display};

use super::InterestArea;
use crate::collections::UnionFind;

pub struct Group {
    representative_id: usize,
    name: String,
    interest_area: InterestArea,
    students: RefCell<UnionFind>,
}

impl Group {
    pub fn new(
        representative_id: usize,
        name: String,
        interest_area: InterestArea,
        size: usize,
    ) -> Self {
        Self {
            representative_id,
            name,
            interest_area,
            students: RefCell::new(UnionFind::new(size)),
        }
    }

    pub fn representative_id(&self) -> usize {
        self.representative_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn interest_area(&self) -> &InterestArea {
        &self.interest_area
    }

    pub fn len(&self) -> usize {
        self.students
            .borrow_mut()
            .representative(self.representative_id)
            .1
    }

    pub fn contains(&self, student_id: usize) -> bool {
        self.students.borrow_mut().representative(student_id).0 == self.representative_id
    }

    pub fn add(&mut self, student_id: usize) {
        self.students
            .borrow_mut()
            .join(self.representative_id, student_id)
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} [{}]: {}",
            self.name,
            self.len(),
            self.interest_area()
        )
    }
}
