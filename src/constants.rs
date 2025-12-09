use embassy_time::Duration;

pub const DEBOUNCE_DELAY: Duration = Duration::from_millis(10);
pub const MAX_ANIMATIONS: usize = 6;
pub const FRAME_RATE: usize = 30;
