use core::ops::AddAssign;

use embassy_time::{Duration, Instant};

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
    pub fn now(&mut self) -> Duration {
        let ms = Instant::now().as_millis() + self.offset.as_millis();
        Duration::from_millis(ms)
    }
}

impl AddAssign<Duration> for Time {
    fn add_assign(&mut self, rhs: Duration) {
        let ms = self.offset.as_millis() + rhs.as_millis();
        self.offset = Duration::from_millis(ms)
    }
}
