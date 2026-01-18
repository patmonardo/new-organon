//! Mapped batch implementation for ML in GDS.
//!
//! Translated from Java GDS ml-core MappedBatch.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::{Batch, BatchTransformer};
use std::sync::Arc;

/// A batch that applies a transformation to element IDs from a delegate batch.
pub struct MappedBatch<B> {
    delegate: B,
    transformer: Arc<dyn BatchTransformer>,
}

impl<B> MappedBatch<B>
where
    B: Batch,
{
    /// Create a new MappedBatch with a delegate batch and transformer.
    pub fn new(delegate: B, transformer: Arc<dyn BatchTransformer>) -> Self {
        Self {
            delegate,
            transformer,
        }
    }
}

impl<B> Batch for MappedBatch<B>
where
    B: Batch,
{
    type ElementIdsIter = MappedIterator<B::ElementIdsIter>;

    fn element_ids(&self) -> Self::ElementIdsIter {
        MappedIterator::new(self.delegate.element_ids(), Arc::clone(&self.transformer))
    }

    fn size(&self) -> usize {
        self.delegate.size()
    }
}

/// Iterator that applies a transformation to element IDs.
pub struct MappedIterator<I> {
    iter: I,
    transformer: Arc<dyn BatchTransformer>,
}

impl<I> MappedIterator<I>
where
    I: Iterator<Item = u64>,
{
    fn new(iter: I, transformer: Arc<dyn BatchTransformer>) -> Self {
        Self { iter, transformer }
    }
}

impl<I> Iterator for MappedIterator<I>
where
    I: Iterator<Item = u64>,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|id| self.transformer.apply(id))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::core::batch::{IdentityBatchTransformer, ListBatch};

    #[test]
    fn test_mapped_batch_with_identity() {
        let delegate = ListBatch::new(vec![1, 2, 3, 4, 5]);
        let transformer = Arc::new(IdentityBatchTransformer);
        let mapped = MappedBatch::new(delegate, transformer);

        assert_eq!(mapped.size(), 5);
        let ids: Vec<u64> = mapped.element_ids().collect();
        assert_eq!(ids, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_mapped_batch_with_offset() {
        struct OffsetTransformer {
            offset: u64,
        }

        impl BatchTransformer for OffsetTransformer {
            fn apply(&self, index: u64) -> u64 {
                index + self.offset
            }
        }

        let delegate = ListBatch::new(vec![1, 2, 3]);
        let transformer = Arc::new(OffsetTransformer { offset: 10 });
        let mapped = MappedBatch::new(delegate, transformer);

        assert_eq!(mapped.size(), 3);
        let ids: Vec<u64> = mapped.element_ids().collect();
        assert_eq!(ids, vec![11, 12, 13]);
    }

    #[test]
    fn test_mapped_iterator_size_hint() {
        let delegate = ListBatch::new(vec![1, 2, 3, 4, 5]);
        let transformer = Arc::new(IdentityBatchTransformer);
        let mapped = MappedBatch::new(delegate, transformer);

        let mut iter = mapped.element_ids();
        assert_eq!(iter.size_hint(), (5, Some(5)));

        // Consume one element
        let _ = iter.next();
        assert_eq!(iter.size_hint(), (4, Some(4)));
    }
}
