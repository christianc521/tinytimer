use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::{ Duration, Ticker };
use crate::{button::Button, tft::TFT, time_util::Time};


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
            SessionState::Working => todo!(),
            SessionState::Break => todo!(),
            SessionState::Paused => todo!(),

        }
    }

    async fn execute_working(self, session: &mut DoubleTimerSession<'_>, button: &mut Button<'_>) -> Self {
        

    }
}

pub type SessionNotifier = (SessionOuterNotifier, TFTNotifier);
pub type SessionOuterNotifier = Channel<CriticalSectionRawMutex, SessionNotice, 4>;

pub struct DoubleTimerSession<'spi> //(&'spi SessionOuterNotifier);
{
    tft: TFT<'spi>,
    work_clock: Duration,
    break_clock: Duration,
    session_state: SessionState,
    spawner: Spawner
}

impl<'spi> DoubleTimerSession<'spi> {
    pub fn new(
        tft: TFT<'spi>,
        spawner: Spawner,
        notifier: &'static SessionNotifier,
    ) -> Result<Self, Spawn> {
        DoubleTimerSession {
            tft,
            work_clock: Duration::from_millis(0),
            break_clock: Duration::from_millis(0),
            session_state: SessionState::Paused,
            spawner
        }
    }

    pub async fn run_work_clock(&mut self) {
        self.session_state = SessionState::Working;
        let mut ticker = Ticker::every(Duration::from_millis(100));

        while self.session_state == SessionState::Working {
            self.work_clock += Duration::from_millis(100);
            ticker.next().await;
        }
    }

    pub async fn run_break_clock(&mut self) {
        self.session_state = SessionState::Break;
        let mut ticker = Ticker::every(Duration::from_millis(100));

        while self.session_state == SessionState::Break {
            self.break_clock += Duration::from_millis(100);
            ticker.next().await;
        }
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
async fn device_loop(session_notifier: )
