//! Memory range type representing min/max byte values
//!
//! A memory range represents a span of possible memory usage values,
//! with a minimum and maximum. Supports arithmetic operations for
//! combining and transforming ranges.

use std::fmt;

use super::estimate::Estimate;

/// Represents a range of positive byte values
///
/// The range can span 0 bytes when min and max are identical.
/// Supports arithmetic operations for memory estimation composition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub struct MemoryRange {
    min: usize,
    max: usize,
}

impl MemoryRange {
    fn checked_add(a: usize, b: usize) -> usize {
        a.checked_add(b).expect("MemoryRange overflow while adding")
    }

    fn checked_mul(a: usize, b: usize) -> usize {
        a.checked_mul(b)
            .expect("MemoryRange overflow while multiplying")
    }

    fn checked_sub(a: usize, b: usize) -> usize {
        a.checked_sub(b)
            .expect("MemoryRange underflow while subtracting")
    }

    /// Creates a memory range with identical min and max values
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::MemoryRange;
    ///
    /// let range = MemoryRange::of(1024);
    /// assert_eq!(range.min(), 1024);
    /// assert_eq!(range.max(), 1024);
    /// ```
    pub fn of(value: usize) -> Self {
        Self {
            min: value,
            max: value,
        }
    }

    /// Creates a memory range with different min and max values
    ///
    /// # Panics
    ///
    /// Panics if max < min
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::MemoryRange;
    ///
    /// let range = MemoryRange::of_range(1024, 2048);
    /// assert_eq!(range.min(), 1024);
    /// assert_eq!(range.max(), 2048);
    /// ```
    pub fn of_range(min: usize, max: usize) -> Self {
        assert!(max >= min, "Max must be >= min");
        Self { min, max }
    }

    /// Returns an empty memory range (0 bytes)
    pub fn empty() -> Self {
        Self { min: 0, max: 0 }
    }

    /// Returns the minimum value
    pub fn min(&self) -> usize {
        self.min
    }

    /// Returns the maximum value
    pub fn max(&self) -> usize {
        self.max
    }

    /// Checks if the range is empty (both min and max are 0)
    pub fn is_empty(&self) -> bool {
        self.min == 0 && self.max == 0
    }

    /// Adds a scalar value to both min and max
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::MemoryRange;
    ///
    /// let range = MemoryRange::of_range(100, 200);
    /// let result = range.add_scalar(50);
    /// assert_eq!(result.min(), 150);
    /// assert_eq!(result.max(), 250);
    /// ```
    pub fn add_scalar(&self, value: usize) -> Self {
        Self {
            min: Self::checked_add(self.min, value),
            max: Self::checked_add(self.max, value),
        }
    }

    /// Adds another memory range
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::MemoryRange;
    ///
    /// let r1 = MemoryRange::of_range(100, 200);
    /// let r2 = MemoryRange::of_range(50, 150);
    /// let result = r1.add(&r2);
    /// assert_eq!(result.min(), 150);
    /// assert_eq!(result.max(), 350);
    /// ```
    pub fn add(&self, other: &Self) -> Self {
        Self {
            min: Self::checked_add(self.min, other.min),
            max: Self::checked_add(self.max, other.max),
        }
    }

    /// Multiplies by a scalar
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::MemoryRange;
    ///
    /// let range = MemoryRange::of_range(100, 200);
    /// let result = range.times(3);
    /// assert_eq!(result.min(), 300);
    /// assert_eq!(result.max(), 600);
    /// ```
    pub fn times(&self, count: usize) -> Self {
        Self {
            min: Self::checked_mul(self.min, count),
            max: Self::checked_mul(self.max, count),
        }
    }

    /// Subtracts a scalar value
    ///
    /// # Panics
    ///
    /// Panics if subtracting would underflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::MemoryRange;
    ///
    /// let range = MemoryRange::of_range(100, 200);
    /// let result = range.subtract(50);
    /// assert_eq!(result.min(), 50);
    /// assert_eq!(result.max(), 150);
    /// ```
    pub fn subtract(&self, value: usize) -> Self {
        Self {
            min: Self::checked_sub(self.min, value),
            max: Self::checked_sub(self.max, value),
        }
    }

    /// Element-wise subtraction of another range
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::MemoryRange;
    ///
    /// let r1 = MemoryRange::of_range(200, 300);
    /// let r2 = MemoryRange::of_range(50, 100);
    /// let result = r1.element_wise_subtract(&r2);
    /// assert_eq!(result.min(), 150);
    /// assert_eq!(result.max(), 200);
    /// ```
    pub fn element_wise_subtract(&self, other: &Self) -> Self {
        Self {
            min: Self::checked_sub(self.min, other.min),
            max: Self::checked_sub(self.max, other.max),
        }
    }

    /// Union of two ranges (takes min of mins, max of maxs)
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::MemoryRange;
    ///
    /// let r1 = MemoryRange::of_range(100, 200);
    /// let r2 = MemoryRange::of_range(150, 300);
    /// let result = r1.union(&r2);
    /// assert_eq!(result.min(), 100);
    /// assert_eq!(result.max(), 300);
    /// ```
    pub fn union(&self, other: &Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    /// Maximum of two ranges (takes max of mins, max of maxs)
    pub fn maximum(range1: &Self, range2: &Self) -> Self {
        Self {
            min: range1.min.max(range2.min),
            max: range1.max.max(range2.max),
        }
    }

    /// Apply a function to both min and max
    pub fn apply<F>(&self, f: F) -> Self
    where
        F: Fn(usize) -> usize,
    {
        Self {
            min: f(self.min),
            max: f(self.max),
        }
    }
}

impl fmt::Display for MemoryRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.min == self.max {
            write!(f, "{}", Estimate::human_readable(self.min))
        } else {
            write!(
                f,
                "[{}, {}]",
                Estimate::human_readable(self.min),
                Estimate::human_readable(self.max)
            )
        }
    }
}

impl std::ops::Add for MemoryRange {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            min: Self::checked_add(self.min, other.min),
            max: Self::checked_add(self.max, other.max),
        }
    }
}

impl std::ops::Mul<usize> for MemoryRange {
    type Output = Self;

    fn mul(self, count: usize) -> Self {
        self.times(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_of() {
        let range = MemoryRange::of(1024);
        assert_eq!(range.min(), 1024);
        assert_eq!(range.max(), 1024);
    }

    #[test]
    fn test_of_range() {
        let range = MemoryRange::of_range(100, 200);
        assert_eq!(range.min(), 100);
        assert_eq!(range.max(), 200);
    }

    #[test]
    #[should_panic(expected = "Max must be >= min")]
    fn test_of_range_invalid() {
        MemoryRange::of_range(200, 100);
    }

    #[test]
    fn test_empty() {
        let range = MemoryRange::empty();
        assert!(range.is_empty());
        assert_eq!(range.min(), 0);
        assert_eq!(range.max(), 0);
    }

    #[test]
    fn test_add_scalar() {
        let range = MemoryRange::of_range(100, 200);
        let result = range.add_scalar(50);
        assert_eq!(result.min(), 150);
        assert_eq!(result.max(), 250);
    }

    #[test]
    fn test_add_range() {
        let r1 = MemoryRange::of_range(100, 200);
        let r2 = MemoryRange::of_range(50, 150);
        let result = r1.add(&r2);
        assert_eq!(result.min(), 150);
        assert_eq!(result.max(), 350);
    }

    #[test]
    fn test_times() {
        let range = MemoryRange::of_range(100, 200);
        let result = range.times(3);
        assert_eq!(result.min(), 300);
        assert_eq!(result.max(), 600);
    }

    #[test]
    fn test_subtract() {
        let range = MemoryRange::of_range(100, 200);
        let result = range.subtract(50);
        assert_eq!(result.min(), 50);
        assert_eq!(result.max(), 150);
    }

    #[test]
    #[should_panic(expected = "MemoryRange underflow while subtracting")]
    fn test_underflow_panics() {
        let range = MemoryRange::of(10);
        let _ = range.subtract(20);
    }

    #[test]
    fn test_union() {
        let r1 = MemoryRange::of_range(100, 200);
        let r2 = MemoryRange::of_range(150, 300);
        let result = r1.union(&r2);
        assert_eq!(result.min(), 100);
        assert_eq!(result.max(), 300);
    }

    #[test]
    fn test_maximum() {
        let r1 = MemoryRange::of_range(100, 200);
        let r2 = MemoryRange::of_range(150, 250);
        let result = MemoryRange::maximum(&r1, &r2);
        assert_eq!(result.min(), 150);
        assert_eq!(result.max(), 250);
    }

    #[test]
    fn test_apply() {
        let range = MemoryRange::of_range(100, 200);
        let result = range.apply(|x| x * 2);
        assert_eq!(result.min(), 200);
        assert_eq!(result.max(), 400);
    }

    #[test]
    fn test_display() {
        let fixed = MemoryRange::of(1024);
        assert_eq!(format!("{}", fixed), "1.00 KiB");

        let range = MemoryRange::of_range(1024, 2048);
        assert_eq!(format!("{}", range), "[1.00 KiB, 2.00 KiB]");
    }

    #[test]
    fn test_operators() {
        let r1 = MemoryRange::of(100);
        let r2 = MemoryRange::of(50);

        let sum = r1 + r2;
        assert_eq!(sum.min(), 150);

        let scaled = r1 * 3;
        assert_eq!(scaled.min(), 300);
    }
}
