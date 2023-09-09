//! Time-related operations.

pub use core::time::Duration;

/// A measurement of the system clock.
///
/// Currently, it reuses the [`core::time::Duration`] type. But it does not
/// represent a duration, but a clock time.
pub type TimeValue = Duration;

#[cfg(feature = "irq")]
pub use crate::platform::irq::TIMER_IRQ_NUM;
#[cfg(feature = "irq")]
pub use crate::platform::time::set_oneshot_timer;
#[cfg(feature = "rtc")]
pub use crate::platform::time::rtc_read_time;
pub use crate::platform::time::{current_ticks, nanos_to_ticks, ticks_to_nanos};

/// Number of milliseconds in a second.
pub const MILLIS_PER_SEC: u64 = 1_000;
/// Number of microseconds in a second.
pub const MICROS_PER_SEC: u64 = 1_000_000;
/// Number of nanoseconds in a second.
pub const NANOS_PER_SEC: u64 = 1_000_000_000;
/// Number of nanoseconds in a millisecond.
pub const NANOS_PER_MILLIS: u64 = 1_000_000;
/// Number of nanoseconds in a microsecond.
pub const NANOS_PER_MICROS: u64 = 1_000;

/// Returns the current clock time in nanoseconds.
pub fn current_time_nanos() -> u64 {
    ticks_to_nanos(current_ticks())
}

/// Returns the current clock time in [`TimeValue`].
pub fn current_time() -> TimeValue {
    let nanos = current_time_nanos();
    #[cfg(feature = "rtc")]
    //let x = Duration::new(rtc_read_time(),0);
    return Duration::new((nanos / (NANOS_PER_SEC as u64)) + rtc_read_time(), (nanos % (NANOS_PER_SEC as u64)) as u32);
    TimeValue::from_nanos(current_time_nanos())
}

/// Busy waiting for the given duration.
pub fn busy_wait(dur: Duration) {
    busy_wait_until(current_time() + dur);
}

/// Busy waiting until reaching the given deadline.
pub fn busy_wait_until(deadline: TimeValue) {
    while current_time() < deadline {
        core::hint::spin_loop();
    }
}
