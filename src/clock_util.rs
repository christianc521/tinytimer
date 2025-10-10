use embassy_executor::{SpawnError, Spawner};
use embassy_futures::select::{select, Either};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::{ Duration, Ticker, Timer };
use crate::{button::{Button, PressDuration}, draw_panels::{Panel, PanelPosition, Payload}, render_display::{TFTNotifier, TFTRender}, tft::TFT, time_util::Time};


/*
 * Represents a single Ticker that increments 'run_duration' every tenth of a second
 */
pub struct SingleClock {
    run_duration: Duration,
}

impl SingleClock {
    pub fn new() -> Self {
        SingleClock { 
            run_duration: Duration::from_ticks(0), 
        }
    }

    pub async fn run_clock(&mut self) {
        let mut ticker = Ticker::every(Duration::from_millis(100));
        loop {
            self.run_duration += Duration::from_millis(100);
            ticker.next().await;
        }
    }
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub enum SessionState {
    #[default]
    Working,
    Break,
    Paused
}

impl SessionState {
    pub async fn execute(
        self, 
        session: &mut DoubleTimerSession<'_>, 
        button: &mut Button<'_>) -> Self 
    {
        match self {
            SessionState::Working => self.execute_working(session, button).await,
            SessionState::Break => todo!(),
            SessionState::Paused => todo!(),
        }
    }

    pub(crate) fn render(self, time: &Time) -> (Panel, Duration) {
        match self {
            Self::Working => Self::render_working(time),
            _ => todo!()
        }
    }

    async fn execute_working(self, session: &mut DoubleTimerSession<'_>, button: &mut Button<'_>) -> Self {
        session.set_state(self).await;
        match button.press_duration().await {
            PressDuration::Short => {
                todo!()
            }
            PressDuration::Long => {
                todo!()
            }
        }
    }

    fn render_working(time: &Time) -> (Panel, Duration) {
        let (display_time, sleep_dur) = time.sleep_100ms();
        let panel = Panel::from_time(display_time, PanelPosition::Middle);
        (panel, sleep_dur)
    }

}

pub type SessionNotifier = (SessionOuterNotifier, TFTNotifier);
pub type SessionOuterNotifier = Channel<CriticalSectionRawMutex, SessionNotice, 4>;

pub struct DoubleTimerSession<'spi>(&'spi SessionOuterNotifier);
//{
//    tft: TFT<'spi>,
//    work_clock: Duration,
//    break_clock: Duration,
//    session_state: SessionState,
//    spawner: Spawner
//}

impl<'spi> DoubleTimerSession<'spi> {
    pub fn new(
        tft: TFT<'static>,
        spawner: Spawner,
        notifier: &'static SessionNotifier,
    ) -> Result<Self, SpawnError> {
        let (outer_notifier, tft_notifier) = notifier;
        let tft = TFTRender::new(tft, tft_notifier, spawner)?;
        spawner.spawn(device_loop(outer_notifier, tft))?;
        Ok(Self(outer_notifier))
    }

    pub(crate) async fn set_state(&self, new_state: SessionState) {
        self.0.send(SessionNotice::SetState(new_state)).await;
    }

    #[must_use]
    pub const fn notifier() -> SessionNotifier {
        (Channel::new(), TFTRender::notifier())
    }

}

pub enum SessionNotice {
    SetState(SessionState),
    AdjustTimer(Duration)
}

impl SessionNotice {
    pub(crate) fn apply(self, time: &mut Time, state: &mut SessionState) {
        match self {
            Self::AdjustTimer(delta) => {
                *time += delta
            }
            Self::SetState(new_state) => {
                *state = new_state
            }
        }
    }
}

#[embassy_executor::task]
async fn device_loop(session_notifier: &'static SessionOuterNotifier, tft_renderer: TFTRender<'static>) -> ! {
    let mut time = Time::default();
    let mut session_state = SessionState::default();

    loop {
        let (panel, sleep_dur) = session_state.render(&time);
        tft_renderer.render(panel);
        if let Either::First(notification) = select(session_notifier.receive(), Timer::after(sleep_dur)).await
        {
            notification.apply(&mut time, &mut session_state);
        }
    }
}
