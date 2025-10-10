use embedded_graphics::{prelude::{Point, Size}, primitives::Rectangle};

pub enum Payload {
    Time([u8; 20]),
    Empty
}

pub enum PanelPosition {
    Top,
    Middle,
    BottomLeft,
    BottomRight,
}

impl PanelPosition {
    pub fn get_rect(&self) -> Rectangle {
        match self {
            PanelPosition::Top => {
                    Rectangle::new(Point::new(0, 0), Size::new(320, 64))
            },
            PanelPosition::Middle => {
                    Rectangle::new(Point::new(0, 64), Size::new(320, 96))
            },
            PanelPosition::BottomLeft => {
                    Rectangle::new(Point::new(0, 160), Size::new(160, 80))
            },
            PanelPosition::BottomRight => {
                    Rectangle::new(Point::new(160, 160), Size::new(160, 80))
            },
        }
    }
}

pub struct Panel(pub Rectangle, pub Payload);

impl Default for Panel {
    fn default() -> Self {
        let default_frame = Rectangle::new(Point::zero(), Size::new(320, 240));
        let default_payload = Payload::Empty;
        Panel(default_frame, default_payload)
    }
}

impl Panel {
    pub fn from_time(time: [u8; 20], position: PanelPosition) -> Self {
        let rectangle = position.get_rect();
        let payload = Payload::Time(time);
        Panel(rectangle, payload)
    }
}

