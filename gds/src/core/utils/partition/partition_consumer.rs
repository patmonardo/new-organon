//! Consumer trait for partitions.

use super::Partition;

/// Consumer for partitions that processes each partition.
pub trait PartitionConsumer<P: AsRef<Partition>> {
    /// Process a single partition.
    ///
    /// # Arguments
    /// * `partition` - The partition to process
    fn consume(&mut self, partition: P);
}

// Implement PartitionConsumer for closures
impl<F, P> PartitionConsumer<P> for F
where
    F: FnMut(P),
    P: AsRef<Partition>,
{
    fn consume(&mut self, partition: P) {
        self(partition);
    }
}
