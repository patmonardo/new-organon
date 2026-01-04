//! Progress value representing task completion state.

use super::UNKNOWN_VOLUME;

/// Immutable progress value combining current progress and total volume.
///
/// Java parity target: a small value object with `relative_progress()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Progress {
    progress: usize,
    volume: usize,
}

impl Progress {
    /// Create a new Progress value.
    pub fn of(progress: usize, volume: usize) -> Self {
        Self { progress, volume }
    }

    /// Get current progress value.
    pub fn progress(&self) -> usize {
        self.progress
    }

    /// Get total volume.
    pub fn volume(&self) -> usize {
        self.volume
    }

    /// Calculate relative progress.
    ///
    /// Java parity: returns `Task.UNKNOWN_VOLUME` when volume is unknown.
    pub fn relative_progress(&self) -> f64 {
        if self.volume == UNKNOWN_VOLUME {
            return UNKNOWN_VOLUME as f64;
        }

        // Progress can be larger if volume was estimated too low
        if self.progress >= self.volume {
            1.0
        } else {
            self.progress as f64 / self.volume as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_creation() {
        let p = Progress::of(50, 100);
        assert_eq!(p.progress(), 50);
        assert_eq!(p.volume(), 100);
    }

    #[test]
    fn test_relative_progress() {
        let p = Progress::of(50, 100);
        assert!((p.relative_progress() - 0.5).abs() < 0.001);

        let complete = Progress::of(100, 100);
        assert!((complete.relative_progress() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_over_completion() {
        // Volume estimated too low
        let p = Progress::of(150, 100);
        assert!((p.relative_progress() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_unknown_volume() {
        let p = Progress::of(42, UNKNOWN_VOLUME);
        assert_eq!(p.relative_progress(), UNKNOWN_VOLUME as f64);
    }
}
