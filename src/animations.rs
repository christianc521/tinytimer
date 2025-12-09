use embedded_graphics::{prelude::Point, primitives::{line::Line, Rectangle}};
use embedded_graphics::prelude::*;
use embedded_graphics::geometry::AnchorPoint;
use crate::constants::MAX_ANIMATIONS;

#[derive(Debug, Copy, Clone)]
pub struct AnimationState {
    pub queue: [Animation; MAX_ANIMATIONS],
}

impl Default for AnimationState {
    fn default() -> Self {
        AnimationState {
            queue: [Animation::Empty; MAX_ANIMATIONS]
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Animation {
    Cursor(CursorMove),
    Empty
}

pub trait AnimationEvent {
    fn get_frame(&mut self) -> FrameType;
    fn frame_data(&self) -> &FrameData;
}

impl AnimationEvent for Animation {
    fn get_frame(&mut self) -> FrameType {
        match self {
            Self::Cursor(cursor_data) => cursor_data.get_frame(),
            Self::Empty => FrameType::Empty
        }
    }

    fn frame_data(&self) -> &FrameData {
        match self {
            Self::Cursor(cursor_data) => &cursor_data.frame_data,
            Self::Empty => {
                &FrameData {
                    frame_index: 0,
                    frame_count: 0
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FrameType {
    Rectangle(Rectangle),
    Empty
}

#[derive(Debug, Clone, Copy)]
pub struct FrameData {
    pub frame_index: usize,
    pub frame_count: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct CursorMove {
    pub start: Point,
    pub end: Point,
    pub frame_data: FrameData,
    pub cursor_rect: Rectangle,
    pub path: Line
}

impl CursorMove {
    pub fn initialize(start_pos: Point, end_pos: Point) -> Self {
        let cursor = Rectangle::with_corners(Point::zero(), start_pos);
        let path = Line::new(start_pos, end_pos);
        let frame_count = path.points().count();
        let frame_data = FrameData {
            frame_index: 0,
            frame_count
        };

        Self {
            start: start_pos,
            end: end_pos,
            cursor_rect: cursor,
            frame_data,
            path
        }
    }

    pub fn get_frame(&mut self) -> FrameType {
        let mut frame = FrameType::Empty;
        if self.frame_data.frame_index >= self.frame_data.frame_count {
            let position = self.path.points()
                .nth(self.frame_data.frame_index as usize)
                .unwrap();

            let x = position.x as u32;
            let y = position.y as u32;
            self.cursor_rect = self.cursor_rect.resized(Size::new(x, y), AnchorPoint::TopLeft);
            
            self.frame_data.frame_index += 1;
            frame = FrameType::Rectangle(self.cursor_rect.clone())
        }
        frame
    }
}

pub struct AnimatedSprite {
    frames: [u16; 30],
    frame_index: usize,
}

impl AnimatedSprite {
    pub fn new(frames: [u16; 30]) -> Self {
        Self {
            frames,
            frame_index: 0
        }        
    }

    pub fn get_frame(&mut self) -> u16 {
        let frame = self.frames[self.frame_index];
        self.frame_index += 1;
        frame
    }
}
