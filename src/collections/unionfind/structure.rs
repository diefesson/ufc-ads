pub struct UnionFind {
    data: Vec<isize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![-1; size],
        }
    }

    pub fn representative(&mut self, mut index: usize) -> (usize, usize) {
        while !self.is_representative(index) {
            let parent = self.data[index] as usize;
            if self.is_representative(parent) {
                return (parent, (-self.data[parent]) as usize);
            }
            let grand_parent = self.data[parent];
            self.data[index] = grand_parent;
            index = grand_parent as usize;
        }
        (index, (-self.data[index]) as usize)
    }

    pub fn is_representative(&self, index: usize) -> bool {
        self.data[index] < 0
    }

    pub fn join(&mut self, a: usize, b: usize) {
        let (a_representative, a_size) = self.representative(a);
        let (b_representative, b_size) = self.representative(b);
        if a_representative == b_representative {
            return;
        }
        if a_size <= b_size {
            self.data[a_representative] = b_representative as isize;
            self.data[b_representative] -= a_size as isize;
        } else {
            self.data[b_representative] = a_representative as isize;
            self.data[a_representative] -= b_size as isize;
        }
    }
}
