//! Compressed storage for random walks.

#[derive(Debug)]
pub struct CompressedRandomWalks {
    walks: Vec<Vec<i64>>,
    max_walk_length: usize,
    size: usize,
}

impl CompressedRandomWalks {
    pub fn new(max_walk_count: usize) -> Self {
        Self {
            walks: Vec::with_capacity(max_walk_count),
            max_walk_length: 0,
            size: 0,
        }
    }

    pub fn add(&mut self, _index: usize, walk: &[i64]) {
        self.walks.push(walk.to_vec());
        self.max_walk_length = self.max_walk_length.max(walk.len());
    }

    pub fn set_max_walk_length(&mut self, len: usize) {
        self.max_walk_length = len;
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn walks(&self) -> &[Vec<i64>] {
        &self.walks
    }

    pub fn max_walk_length(&self) -> usize {
        self.max_walk_length
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
