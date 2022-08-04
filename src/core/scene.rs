use macroquad::prelude::*;

/// None: It will keep using the same scene. Other than this. It mean changing to the new scene.
pub enum SceneTransition {
    // Dont change scene
    None,
    // Pop current scene from active stack
    Pop,
    // Push scene on active stack
    Push(Box<dyn GameScene>),
    // Replace current active scene on active stack
    Replace(Box<dyn GameScene>),
}

pub trait GameScene {
    fn init(&mut self);
    fn update(&mut self) -> Result<SceneTransition, i32>;
    fn draw(&mut self);
}
