use crate::core::scene::{GameScene, SceneTransition};
use lerp::Lerp;
use macroquad::prelude::*;

pub struct TitleScene {
    hold_down_tick: u128,
}

impl GameScene for TitleScene {
    fn init(&mut self) {}

    fn update(&mut self) -> Result<SceneTransition, i32> {
        if is_key_down(KeyCode::Down) {
            self.hold_down_tick += crate::ONE_FRAME.as_millis();
            self.hold_down_tick = self.hold_down_tick.min(500);
        } else {
            self.hold_down_tick = match self
                .hold_down_tick
                .checked_sub(crate::ONE_FRAME.as_millis())
            {
                Some(v) => v,
                None => 0,
            };
        }

        if is_key_pressed(KeyCode::Space) {
            let mut scene = crate::scenes::gameplay::GameplayScene::new();
            scene.init();
            return Ok(SceneTransition::Replace(Box::new(scene)));
        }

        Ok(SceneTransition::None)
    }

    fn draw(&mut self) {
        clear_background(RED);

        crate::core::utils::draw_text_center("TITLE", 48, 100.0);

        crate::core::utils::draw_text_center(
            "Press `Space` to start",
            24,
            crate::GAME_HEIGHT * 0.6,
        );

        let percentage = self.hold_down_tick as f32 / 500.0;
        let pos_y = crate::GAME_HEIGHT.lerp(crate::GAME_HEIGHT * 0.7, percentage);
        self.draw_list(pos_y);
    }
}

impl TitleScene {
    pub fn new() -> Self {
        Self { hold_down_tick: 0 }
    }

    fn draw_list(&self, pos_y: f32) {
        crate::core::utils::draw_text_center("hold `down` button", 24, pos_y - 14.0);

        crate::core::utils::draw_text_center("Unlocks:", 24, pos_y + 40.0);
        crate::core::utils::draw_text_center("[ ] ending: 1", 24, pos_y + 100.0);
        crate::core::utils::draw_text_center("[ ] ending: 2", 24, pos_y + 140.0);
        crate::core::utils::draw_text_center("[ ] ending: 3", 24, pos_y + 180.0);
    }
}
