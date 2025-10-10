use core::{ops::AddAssign, str::Utf8Chunk};

use embassy_time::{Duration, Instant};
use heapless::{String, Vec};

// Duration to be rendered on display
pub struct Time {
    offset: Duration
}

impl Default for Time {
    fn default() -> Self {
        Self {
            offset: Duration::from_millis(0)
        }
    }
}

impl Time {
    #[inline]
    pub fn now(&self) -> Duration {
        let ms = Instant::now().as_millis() + self.offset.as_millis();
        Duration::from_millis(ms)
    }

    #[inline]
    pub fn sleep_100ms(&self) -> ([u8; 20], Duration) {
        let now = self.now();
        let sleep_duration = Self::until_next(now, Duration::from_secs(1));
        let seconds_now = now.as_secs();
        let hours = ((seconds_now / 3600) + 11) % 12 + 1; // 1-12 instead of 0-11
        let minutes = (seconds_now % 3600) / 60;
        let seconds = seconds_now % 60;

        let time_arr = format_time(hours, minutes, seconds);

        (time_arr, sleep_duration)
    }

    pub const fn until_next(now: Duration, next: Duration) -> Duration {
        let next_ticks = next.as_ticks();
        Duration::from_ticks(next_ticks - now.as_ticks() % next_ticks)
    }
}

impl AddAssign<Duration> for Time {
    fn add_assign(&mut self, rhs: Duration) {
        let ms = self.offset.as_millis() + rhs.as_millis();
        self.offset = Duration::from_millis(ms)
    }
}

fn format_time(hours: u64, mins: u64, seconds: u64) -> [u8; 20] {

    let mut buffer = [b' '; 20]; // Initialize with spaces
    
    // Convert numbers to digits
    let h1 = (hours / 10) as u8 + b'0';
    let h2 = (hours % 10) as u8 + b'0';
    let m1 = (mins / 10) as u8 + b'0';
    let m2 = (mins % 10) as u8 + b'0';
    let s1 = (seconds / 10) as u8 + b'0';
    let s2 = (seconds % 10) as u8 + b'0';
    let c1 = (mins / 10) as u8 + b'0';
    let c2 = (mins % 10) as u8 + b'0';
    
    // Format: "HH:MM:SS.hh"
    buffer[0] = h1;
    buffer[1] = h2;
    buffer[2] = b':';
    buffer[3] = m1;
    buffer[4] = m2;
    buffer[5] = b':';
    buffer[6] = s1;
    buffer[7] = s2;
    buffer[8] = b'.';
    buffer[9] = c1;
    buffer[10] = c2;
    buffer
}
