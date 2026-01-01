//! RandomWalk Storage
//!
//! Stores generated walk paths during computation.

use std::sync::Mutex;

/// Storage for random walk computation
pub struct RandomWalkStorageRuntime {
    /// Generated walks (each walk is a sequence of node IDs)
    pub walks: Mutex<Vec<Vec<u64>>>,
}

impl Default for RandomWalkStorageRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomWalkStorageRuntime {
    pub fn new() -> Self {
        Self {
            walks: Mutex::new(Vec::new()),
        }
    }

    pub fn add_walk(&self, walk: Vec<u64>) {
        self.walks.lock().unwrap().push(walk);
    }

    pub fn take_walks(&self) -> Vec<Vec<u64>> {
        let mut walks = self.walks.lock().unwrap();
        std::mem::take(&mut *walks)
    }
}
