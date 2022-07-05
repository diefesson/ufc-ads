use crate::collections::UnionFind;

use super::InterestArea;

pub struct Group {
    representative_id: usize,
    interest_area: InterestArea,
    students: UnionFind,
}

impl Group {
    pub fn new(representative_id: usize, interest_area: InterestArea, size: usize) -> Self {
        Self {
            representative_id,
            interest_area,
            students: UnionFind::new(size),
        }
    }

    pub fn is_in(&mut self, student_id: usize) -> bool {
        self.students.representative(student_id) == self.representative_id
    }

    pub fn add(&mut self, student_id: usize) {
        self.students.join(self.representative_id, student_id)
    }
}
