use embedded_graphics::{prelude::{Point, Size}, primitives::Rectangle};

use crate::{animations::Animation, scenes::SceneData};

#[derive(Debug, Clone, Copy)]
pub enum Payload {
    Time([u8; 20]),
    Animate(Animation),
    NewScene(SceneData),
    Empty
}

#[derive(Debug, Clone, Copy)]
pub enum PanelPosition {
    Top,
    Middle,
    Bottom,
    FullScreen
}

impl PanelPosition {
    pub fn get_rect(&self) -> Rectangle {
        match self {
            PanelPosition::Top => {
                    Rectangle::new(Point::new(10, 20), Size::new(300, 50))
            },
            PanelPosition::Middle => {
                    Rectangle::new(Point::new(60, 95), Size::new(200, 50))
            },
            PanelPosition::Bottom => {
                    Rectangle::new(Point::new(10, 160), Size::new(300, 50))
            },
            PanelPosition::FullScreen => {
                    Rectangle::new(Point::zero(), Size::new(320, 240))
            },
        }
    }
}

pub struct Panel(pub PanelPosition, pub Payload);

impl Default for Panel {
    fn default() -> Self {
        let empty_time = Payload::Time([0; 20]);
        Panel(PanelPosition::Top, empty_time)
    }
}

impl Panel {
    pub fn from_time(time: [u8; 20], position: PanelPosition) -> Self {
        let payload = Payload::Time(time);
        Panel(position, payload)
    }
}

