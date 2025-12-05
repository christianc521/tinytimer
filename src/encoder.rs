use embassy_futures::select::select;
use rotary_encoder_hal::{DefaultPhase, Direction, Rotary};
use esp_hal::gpio::Input;

pub struct Encoder<'a>(Rotary<Input<'a>, Input<'a>, DefaultPhase>);

impl<'a> Encoder<'a> {
    pub fn new(pin_a: Input<'a>, pin_b: Input<'a>) -> Self {
        let encoder = Rotary::new(pin_a, pin_b);
        Encoder(encoder)
    }

    pub async fn wait_for_edge(&mut self) -> &mut Self {
        let (pin_a, pin_b) = self.0.pins();

        // Wait for either pin to change pull
        select(pin_a.wait_for_any_edge(), pin_b.wait_for_any_edge()).await;
        match self.0.update().unwrap() {
            Direction::Clockwise => {
                esp_println::println!("Moved Clockwise!")
            }
            Direction::CounterClockwise => {
                esp_println::println!("Moved Counter Clockwise!")
            }
            Direction::None => {}
        };
        self
    }
}
