use alloc::vec::Vec;

pub struct Combinations {
    indices: Vec<usize>,
    n: usize,
    k: usize,
    first: bool,
}

impl Combinations {
    pub fn new(n: usize, k: usize) -> Self {
        Self {
            indices: (0..k).collect(),
            n,
            k,
            first: true,
        }
    }
}

impl Iterator for Combinations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            if self.k <= self.n {
                return Some(self.indices.clone());
            }
            return None;
        }

        let mut i = self.k;
        while i > 0 {
            i -= 1;
            if self.indices[i] != i + self.n - self.k {
                self.indices[i] += 1;
                for j in (i + 1)..self.k {
                    self.indices[j] = self.indices[j - 1] + 1;
                }
                return Some(self.indices.clone());
            }
        }
        None
    }
}