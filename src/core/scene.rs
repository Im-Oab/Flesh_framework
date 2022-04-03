use macroquad::prelude::*;
pub enum SceneTransition {
    None,
    Pop,
    Push(Box<dyn GameScene>),
    Replace(Box<dyn GameScene>),
}

pub trait GameScene {
    fn init(&mut self);
    fn update(&mut self) -> Result<SceneTransition, i32>;
    fn draw(&mut self);
}
