#![no_std]
#![no_main]

use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal::gpio;
use esp_hal::{clock::CpuClock, gpio::Input};
use esp_hal::timer::timg::TimerGroup;
use embedded_graphics::{
    prelude::*,
    pixelcolor::Rgb565
};
use pitft_async::clock_util::DoubleTimerSession;
use pitft_async::{button::Button, clock_util::SessionState, tft::TFT};
use log::info;


#[derive(Debug)]
pub enum Never {}

#[embassy_executor::task]
async fn run() {
    loop {
        esp_println::println!("im in da embussy :3")
    }
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.2.2

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let cs = peripherals.GPIO2;
    let dc = peripherals.GPIO4;
    let mosi = peripherals.GPIO7;
    let miso = peripherals.GPIO5;
    let sclk = peripherals.GPIO6;
    let rst = peripherals.GPIO3;
    let input = Input::new(peripherals.GPIO1, gpio::Pull::Down);

    // create TFT struct with direct display control
    let mut tft = TFT::new(
        peripherals.SPI2, 
        sclk, 
        miso, 
        mosi, 
        cs, 
        rst, 
        dc);
    let mut button = Button::new(input);
    let mut state = SessionState::default();

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    tft.clear(Rgb565::BLACK);
    tft.draw_image();
    tft.render_border();
    tft.draw_focus_time("im a new soul :3");
    tft.draw_unfocused_time("test");

    let mut session = DoubleTimerSession::new(tft, spawner);
    loop {
        state = state.execute(&mut session, &mut button).await;
    }
}
