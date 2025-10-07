use embassy_executor::{SpawnError, Spawner};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};

use crate::tft::{DoubleTimerRectangles, TFT};

#[derive(Debug)]
pub enum Never {
}

pub struct TFTRender<'a>(&'a TFTNotifier);
pub type TFTNotifier = Signal<CriticalSectionRawMutex, DoubleTimerRectangles>;

impl TFTRender<'_> {
    #[must_use]
    pub const fn notifier() -> TFTNotifier {
        Signal::new()
    }

    pub fn new(
        tft: TFT,
        notifier: &'static TFTNotifier,
        spawner: Spawner
        ) -> Result<Self, SpawnError> {

    }
}

#[embassy_executor::task]
async fn render_loop(
    tft: TFT<'static>,
    notifier: &'static TFTNotifier
) -> ! {
    let err = inner_render_loop(tft, notifier).await;
}

async fn inner_render_loop(
    mut tft: TFT<'static>,
    mut notifier: &'static TFTNotifier
) -> ! {
    loop {
        todo!()
    }
}
