//! Memory reservation exception.
//!
//! Java parity: mirrors `org.neo4j.gds.mem.MemoryReservationExceededException`.

use std::error::Error;
use std::fmt;

/// Error indicating that a memory reservation attempt failed.
///
/// Thrown when requested memory exceeds available memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryReservationExceededException {
    bytes_required: u64,
    bytes_available: u64,
}

impl MemoryReservationExceededException {
    /// Creates a new memory reservation exception
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::MemoryReservationExceededException;
    ///
    /// let err = MemoryReservationExceededException::new(1024 * 1024 * 100, 1024 * 1024 * 50);
    /// assert_eq!(err.bytes_required(), 1024 * 1024 * 100);
    /// assert_eq!(err.bytes_available(), 1024 * 1024 * 50);
    /// ```
    pub fn new(bytes_required: u64, bytes_available: u64) -> Self {
        Self {
            bytes_required,
            bytes_available,
        }
    }

    /// Returns the number of bytes that were required
    pub fn bytes_required(&self) -> u64 {
        self.bytes_required
    }

    /// Returns the number of bytes that were available
    pub fn bytes_available(&self) -> u64 {
        self.bytes_available
    }
}

impl fmt::Display for MemoryReservationExceededException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Memory reservation failed. Required: {} bytes, Available: {} bytes.",
            self.bytes_required, self.bytes_available
        )
    }
}

impl Error for MemoryReservationExceededException {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let err = MemoryReservationExceededException::new(1000, 500);

        assert_eq!(err.bytes_required(), 1000);
        assert_eq!(err.bytes_available(), 500);
    }

    #[test]
    fn test_display_default() {
        let err = MemoryReservationExceededException::new(1000, 500);
        let display = format!("{}", err);

        assert!(display.contains("1000"));
        assert!(display.contains("500"));
        assert!(display.contains("Required"));
        assert!(display.contains("Available"));
    }
}
