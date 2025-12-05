use embedded_graphics::prelude::Point;


pub enum Scene {
    ConfigTaro,
    ConfigTaroPlus,
    ConfigCountingUp,
}

enum UIElement {
    Menu(u8),
    Clickable(),
    Scrollable,
    Empty
}

enum UIAction {
    Back,
    Select,
    MoveBack,
    MoveNext
}

struct SceneData {
    scene: Scene,
    cursor_index: u8,
    curr_level: u8,
}

struct UINode<'a> {
    parent: Option<&'a UINode<'a>>,
    child: Option<&'a UINode<'a>>,
    element_type: UIElement,
    position: Point
}

pub struct SceneManager {
    pub curr_scene: Scene
}

impl SceneManager {
    pub fn initialize(scene: Scene) -> Self {
        let menu_config = match scene {
            Scene::ConfigTaro => {

            }
            Scene::ConfigTaroPlus => {

            }
            Scene::ConfigCountingUp => {

            }
        };

        SceneManager {
            curr_scene: scene
        }
    }

    pub fn init_config_taro() {

    }

}
