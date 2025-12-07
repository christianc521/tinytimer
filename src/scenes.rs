use embedded_graphics::{pixelcolor::Rgb565, prelude::{PixelColor, RgbColor}, primitives::Rectangle, Drawable};
use crate::clickable::ClickableElement;

pub enum Scene {
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

pub struct SceneData {
    pub scene: Scene,
    pub elements: [UIType; 20],
    pub cursor_index: u8,
}

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

