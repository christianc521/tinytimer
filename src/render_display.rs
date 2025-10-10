use embassy_executor::{SpawnError, Spawner};
use embassy_futures::select::{select, Either};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{Duration, Timer};

use crate::tft::TFT;
use crate::draw_panels::Panel;

#[derive(Debug)]
pub enum Never {
}

pub struct TFTRender<'a>(&'a TFTNotifier);
pub type TFTNotifier = Signal<CriticalSectionRawMutex, Panel>;

impl TFTRender<'_> {
    #[must_use]
    pub const fn notifier() -> TFTNotifier {
        Signal::new()
    }

    pub fn new(
        tft: TFT<'static>,
        notifier: &'static TFTNotifier,
        spawner: Spawner
        ) -> Result<Self, SpawnError> {
        spawner.spawn(render_loop(tft, notifier))?;
        Ok(Self(notifier))
    }

    // called by Session
    pub fn render(&self, frame: Panel) {
       self.0.signal(frame); 
    }
}

#[embassy_executor::task]
async fn render_loop(
    tft: TFT<'static>,
    notifier: &'static TFTNotifier
) -> ! {
    // safely start state loop
    let err = inner_render_loop(tft, notifier).await;
}

// final step; draws to the display
async fn inner_render_loop(
    mut tft: TFT<'static>,
    notifier: &'static TFTNotifier
) -> ! {
    let mut panel = Panel::default();
    'outer: loop {
        // pass off the state information to the hardware wrapper
        tft.handle_payload(&panel);

        // either wait for a new payload or sleep for 4 seconds
        let sleep_or_signal = 
            select(
                Timer::after_secs(4), 
                notifier.wait()
            ).await;

        // if a new payload was recieved before the sleep, 
        // start loop with new payload
        if let Either::Second(notification) = sleep_or_signal {
            panel = notification;
            continue 'outer
        }
    }
}
