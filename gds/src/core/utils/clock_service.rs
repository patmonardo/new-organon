use std::cell::Cell;
use std::time::{SystemTime, UNIX_EPOCH};

/// Trait representing a clock that provides the current time in milliseconds.
/// This is the Rust equivalent of Java's java.time.Clock.
pub trait Clock: Send + Sync {
    /// Returns the current millisecond instant of the clock.
    /// Equivalent to java.time.Clock.millis().
    fn millis(&self) -> u64;
}

/// A default implementation of Clock that uses the system's UTC time.
#[derive(Debug)]
struct SystemUTCClock;

impl Clock for SystemUTCClock {
    fn millis(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before UNIX epoch")
            .as_millis() as u64
    }
}

/// Provides a globally accessible clock, which can be overridden for testing purposes.
/// This is a Rust translation of GDS's org.neo4j.gds.core.utils.ClockService.
pub struct ClockService;

static SYSTEM_CLOCK: SystemUTCClock = SystemUTCClock;

thread_local! {
    static CLOCK_OVERRIDE: Cell<Option<&'static dyn Clock>> = const { Cell::new(None) };
}

impl ClockService {
    /// Sets the clock to be used by the service.
    pub fn set_clock(clock: &'static dyn Clock) {
        CLOCK_OVERRIDE.with(|slot| slot.set(Some(clock)));
    }

    /// Returns the currently configured clock.
    /// Defaults to the system UTC clock.
    pub fn clock() -> &'static dyn Clock {
        CLOCK_OVERRIDE.with(|slot| slot.get().unwrap_or(&SYSTEM_CLOCK))
    }

    /// Executes a given closure with a temporarily specified clock.
    /// The original clock is restored after the closure completes,
    /// regardless of whether it completes normally or panics.
    pub fn run_with_clock<T: Clock, F, R>(temp_clock: &'static T, runnable: F) -> R
    where
        F: FnOnce(&'static T) -> R,
    {
        let previous = CLOCK_OVERRIDE.with(|slot| {
            let prev = slot.get();
            slot.set(Some(temp_clock));
            prev
        });

        let result =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| runnable(temp_clock)));

        CLOCK_OVERRIDE.with(|slot| slot.set(previous));

        match result {
            Ok(r) => r,
            Err(e) => std::panic::resume_unwind(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    struct MockClock {
        current_time: AtomicU64,
    }

    impl MockClock {
        const fn new(initial_time: u64) -> Self {
            Self {
                current_time: AtomicU64::new(initial_time),
            }
        }

        fn advance_millis(&self, ms: u64) {
            self.current_time.fetch_add(ms, Ordering::SeqCst);
        }

        fn set_millis(&self, ms: u64) {
            self.current_time.store(ms, Ordering::SeqCst);
        }
    }

    impl Clock for MockClock {
        fn millis(&self) -> u64 {
            self.current_time.load(Ordering::SeqCst)
        }
    }

    #[test]
    fn test_system_clock() {
        let clock = ClockService::clock();
        let time1 = clock.millis();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let time2 = clock.millis();
        assert!(time2 >= time1);
    }

    #[test]
    fn test_mock_clock() {
        static MOCK: MockClock = MockClock::new(1000);

        ClockService::set_clock(&MOCK);
        assert_eq!(ClockService::clock().millis(), 1000);

        MOCK.advance_millis(500);
        assert_eq!(ClockService::clock().millis(), 1500);

        MOCK.set_millis(2000);
        assert_eq!(ClockService::clock().millis(), 2000);
    }

    #[test]
    fn test_run_with_clock() {
        static TEMP_MOCK: MockClock = MockClock::new(5000);

        let original_time = ClockService::clock().millis();

        let result = ClockService::run_with_clock(&TEMP_MOCK, |clock| {
            assert_eq!(clock.millis(), 5000);
            clock.millis() + 100
        });

        assert_eq!(result, 5100);

        // Original clock should be restored
        let restored_time = ClockService::clock().millis();
        assert!(restored_time >= original_time);
    }
}
