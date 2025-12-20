use super::compressed_random_walks::CompressedWalkIterator;
use crate::collections::HugeDoubleArray;
use crate::concurrency::parallel_util::BatchUtil;
use crate::core::utils::progress::ProgressTracker;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

const FILTERED_NODE_MARKER: i64 = -2;

/// Produces positive (center, context) samples from random walks.
///
/// Java: `PositiveSampleProducer`
pub struct PositiveSampleProducer<'a> {
    walks: CompressedWalkIterator<'a>,
    sampling_probabilities: &'a HugeDoubleArray,
    prefix_window_size: usize,
    postfix_window_size: usize,

    current_walk: Vec<i64>,
    center_word_index: isize,
    current_center_word: i64,
    context_word_index: isize,
    current_window_start: isize,
    current_window_end: isize,

    rng: ChaCha8Rng,
    progress_tracker: ProgressTracker,
    attempted_sampling_walks: bool,
}

impl<'a> PositiveSampleProducer<'a> {
    pub fn new(
        walks: CompressedWalkIterator<'a>,
        sampling_probabilities: &'a HugeDoubleArray,
        window_size: usize,
        random_seed: u64,
        progress_tracker: ProgressTracker,
    ) -> Self {
        let prefix_window_size = BatchUtil::ceil_div(window_size.saturating_sub(1), 2);
        let postfix_window_size = (window_size.saturating_sub(1)) / 2;

        Self {
            walks,
            sampling_probabilities,
            prefix_window_size,
            postfix_window_size,
            current_walk: Vec::new(),
            center_word_index: -1,
            current_center_word: -1,
            context_word_index: 1,
            current_window_start: 0,
            current_window_end: 0,
            rng: ChaCha8Rng::seed_from_u64(random_seed),
            progress_tracker,
            attempted_sampling_walks: false,
        }
    }

    /// Writes the next (center, context) pair into `buffer`.
    pub fn next(&mut self, buffer: &mut [i64; 2]) -> bool {
        if self.next_context_word() {
            buffer[0] = self.current_center_word;
            buffer[1] = self.current_walk[self.context_word_index as usize];
            return true;
        }
        false
    }

    fn next_walk(&mut self) -> bool {
        if self.attempted_sampling_walks {
            // a walk has been exhausted
            let _ = &self.progress_tracker; // placeholder
        }
        self.attempted_sampling_walks = true;

        let Some(walk_slice) = self.walks.next_walk() else {
            return false;
        };

        // Copy walk so that we can mutate/filter it without relying on iterator buffer lifetimes.
        let mut walk = walk_slice.to_vec();

        // Skip walks that filter down to < 2 valid nodes.
        while self.skip_walk(&mut walk) {
            if let Some(next) = self.walks.next_walk() {
                walk = next.to_vec();
            } else {
                return false;
            }
        }

        self.current_walk = walk;
        self.center_word_index = -1;
        self.next_center_word()
    }

    fn skip_walk(&mut self, walk: &mut [i64]) -> bool {
        let filtered_len = self.filter(walk);
        let skip = filtered_len < 2;
        if skip {
            let _ = &self.progress_tracker; // placeholder
        }
        skip
    }

    fn next_center_word(&mut self) -> bool {
        loop {
            self.center_word_index += 1;
            let idx = self.center_word_index as usize;
            if self.current_walk.is_empty() {
                return self.next_walk();
            }

            if idx >= self.current_walk.len() || self.current_walk[idx] == -1 {
                return self.next_walk();
            }

            if self.current_walk[idx] == FILTERED_NODE_MARKER {
                continue;
            }

            self.current_center_word = self.current_walk[idx];
            self.set_context_boundaries();
            self.context_word_index = self.current_window_start - 1;
            return self.next_context_word();
        }
    }

    fn next_context_word(&mut self) -> bool {
        loop {
            if self.current_walk.is_empty() {
                return self.next_center_word();
            }

            self.context_word_index += 1;
            if self.context_word_index <= self.current_window_end
                && self.context_word_index != self.center_word_index
            {
                let val = self.current_walk[self.context_word_index as usize];
                if val >= 0 {
                    return true;
                }
            }

            if self.context_word_index > self.current_window_end {
                return self.next_center_word();
            }
        }
    }

    fn filter(&mut self, walk: &mut [i64]) -> usize {
        let mut filtered_len = 0usize;
        for node in walk.iter_mut() {
            if *node == -1 {
                break;
            }
            if *node >= 0 && self.should_pick_node(*node) {
                filtered_len += 1;
            } else if *node >= 0 {
                *node = FILTERED_NODE_MARKER;
            }
        }
        filtered_len
    }

    fn should_pick_node(&mut self, node_id: i64) -> bool {
        let p = self.sampling_probabilities.get(node_id as usize);
        self.rng.gen_range(0.0..1.0) < p
    }

    fn set_context_boundaries(&mut self) {
        let mut current_prefix = self.prefix_window_size as isize;
        self.current_window_start = self.center_word_index;
        while current_prefix > 0 && self.current_window_start > 0 {
            self.current_window_start -= 1;
            let idx = self.current_window_start as usize;
            if self.current_walk[idx] > 0 {
                current_prefix -= 1;
            }
        }

        let mut current_postfix = self.postfix_window_size as isize;
        self.current_window_end = self.center_word_index;
        while current_postfix > 0
            && (self.current_window_end as usize) < self.current_walk.len().saturating_sub(1)
            && self.current_walk[self.current_window_end as usize] != -1
        {
            self.current_window_end += 1;
            let idx = self.current_window_end as usize;
            if self.current_walk[idx] > 0 {
                current_postfix -= 1;
            }
        }
    }
}


