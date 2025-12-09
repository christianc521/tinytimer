use core::{default, ops::Index};

use embedded_graphics::{pixelcolor::Rgb565, prelude::{PixelColor, RgbColor}, primitives::Rectangle, Drawable};
use heapless::Vec;
use crate::{animations::{Animation, AnimationEvent, FrameType}, clickable::ClickableElement};

#[derive(Default)]
pub enum Scene {
    #[default]
    ConfigTaro,
    ConfigTaroPlus,
    ConfigCountingUp,
}

pub trait UINode {
    fn get_position(&self) -> &Rectangle;

    fn handle_action(&mut self, scene: &mut SceneData, action: UIAction);
}

pub enum UIType {
    Menu(),
    Clickable(ClickableElement),
    Digits(DigitsElement),
    TextBox
}

pub enum UIAction {
    Back,
    Select,
    MoveBack,
    MoveNext
}

pub struct SceneManager {
    pub current_scene: SceneData,
    pub playing_animation: bool,
    pub animation_queue: [Animation; 6]
}

impl Default for SceneManager {
    fn default() -> Self {
        SceneManager {
            current_scene: SceneData::default(),
            playing_animation: false,
            animation_queue: [const {Animation::Empty}; 6]
        }
    }
}

impl SceneManager {
    pub fn play_next(&mut self) -> [FrameType; 6] {
        let mut frames = Vec::<FrameType, 6>::new();
        for ( index, animation ) in self.animation_queue
            .iter_mut()
            .enumerate() {
            match animation {
                // If no animation queued here, do nothing
                Animation::Empty => {
                    ()
                }
                _ => {
                    let next_frame = animation.get_frame();
                    match next_frame {
                        // If the next frame is empty, 
                        // set self.animation_queue block to empty
                        FrameType::Empty => {
                            *animation = Animation::Empty;
                            frames[index] = FrameType::Empty;
                        }
                        // Else, populate animation frame queue with frame data
                        // TODO: to be handled in tft.rs 
                        _ => {
                            frames[index] = next_frame;
                        }
                    }
                }
            }
        }
        frames.into_array().unwrap()
    }
}

pub struct SceneData {
    pub scene: Scene,
    pub elements: [UIType; 10],
    pub cursor_index: u8,
}

impl Default for SceneData {
    fn default() -> Self {
        TARO_CONFIG_SCENE
    }
}

const TARO_CONFIG_SCENE: SceneData = SceneData {
    scene: Scene::ConfigTaro,
    elements: [const {UIType::TextBox}; 10],
    cursor_index: 0
};

pub struct DigitsElement {
    pub position: Rectangle,
    pub current_digit: u8,
    next_element: u8,
    prev_element: u8
}

impl UINode for DigitsElement 
{
   fn get_position(&self) -> &Rectangle {
        &self.position
   } 

   fn handle_action(&mut self, scene: &mut SceneData, action: UIAction) {
       match action {
            UIAction::MoveBack => {
                if self.current_digit == 0 {
                    self.current_digit = 9;
                }
                self.current_digit -= 1;
            }
            UIAction::MoveNext => {
                if self.current_digit == 9 {
                    self.current_digit = 0;
                }
                self.current_digit += 1;
            }
            UIAction::Select => {
                scene.cursor_index = self.next_element;
            }
            UIAction::Back => {
                scene.cursor_index = self.prev_element;
            }
       }
   }
}

impl Drawable for DigitsElement 
{
    type Color = Rgb565;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
        where
            D: embedded_graphics::prelude::DrawTarget<Color = Self::Color> {
                todo!()
        
    }

}

