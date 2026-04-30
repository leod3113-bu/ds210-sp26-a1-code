// Imports
use std::time::Instant;

// Typedef
pub type Timeout = (Instant, u128);

// Helpers
pub fn begin_timeout(duration: u128) -> Timeout {
    (Instant::now(), duration)
}

pub fn get_epoch(timeout: &Timeout) -> Instant {
    timeout.0
}

pub fn get_duration(timeout: &Timeout) -> u128 {
    timeout.1
}

pub fn has_timed_out(timeout: &Timeout) -> bool {
    Instant::now().duration_since(get_epoch(timeout)).as_millis() > get_duration(timeout)
}
