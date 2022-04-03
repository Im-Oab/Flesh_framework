use crate::core::scene::{GameScene, SceneTransition};
use macroquad::prelude::*;
use lerp::Lerp;


pub struct EndingScene
{
    tick: u128,
}

impl EndingScene{
    pub fn new() -> Self
    {
        Self
        {
            tick: 0,
        }
    }
}

impl GameScene for EndingScene
{
    fn init(&mut self)
    {

    }

    fn update(&mut self) -> Result<SceneTransition, i32>
    {
        self.tick += crate::ONE_FRAME.as_millis();
        if self.tick > 1000
        {
            let mut title_scene = crate::scenes::title::TitleScene::new();
            title_scene.init();
            return Ok(SceneTransition::Replace(Box::new(title_scene)));
        }

        Ok(SceneTransition::None)
    }

    fn draw(&mut self)
    {
        clear_background(BLACK);
    }
}