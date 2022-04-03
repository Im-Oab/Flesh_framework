use crate::core::sprite::Sprite;
use macroquad::prelude::*;

pub struct Player {
    sprite: Sprite,
    pub position: Vec2,
    pub rotation: f32,

    // iFrame
    iframe: i32,
    // Life
    pub water: i32,

    // Control
    pub direction: i32,
    pub bouncing_force: i32,
    pub external_force: i32,
}

impl Player {
    pub fn new() -> Self {
        let mut sprite = Sprite::new();
        let animation_manager = crate::ANIMATION_MANAGER.lock().unwrap();
        if let Some(anim) = animation_manager.get("player-idle") {
            sprite.play(anim);
        }

        Self {
            sprite: sprite,
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,

            iframe: 0,
            direction: 1,
            bouncing_force: 0,
            external_force: 0,
            water: 5000,
        }
    }

    pub fn update(&mut self) {
        self.sprite.update();

        if is_key_pressed(KeyCode::Space) {
            self.switching_direction();
        }

        let speed = 8.0;
        let next_x = self.position.x + (speed * self.direction as f32);

        self.bouncing(next_x);
        self.position.x = (next_x + self.bouncing_force as f32 + self.external_force as f32)
            .max(0.0)
            .min(crate::GAME_WIDTH);
        if self.bouncing_force > 0 {
            self.bouncing_force -= 1;
        } else if self.bouncing_force < 0 {
            self.bouncing_force += 1;
        }

        if self.external_force > 0 {
            self.external_force -= 5;
            self.external_force = self.external_force.max(0);
        } else if self.external_force < 0 {
            self.external_force += 5;
            self.external_force = self.external_force.min(0);
        }

        self.water -= crate::ONE_FRAME.as_millis() as i32;
        if self.iframe > 0 {
            self.iframe -= 1;
        }
    }

    pub fn draw(&self) {
        // self.sprite.draw(self.position, self.rotation);
        if self.water > 0 {
            draw_circle(self.position.x, self.position.y, 15.0, BLACK);
        } else {
            draw_circle_lines(self.position.x, self.position.y, 15.0, 2.0, BLACK);
        }
    }

    pub fn bouncing(&mut self, next_x: f32) {
        if next_x >= crate::GAME_WIDTH {
            self.bouncing_force = -rand::RandomRange::gen_range(10, 15);
        } else if next_x <= 0.0 {
            self.bouncing_force = rand::RandomRange::gen_range(10, 15);
        }
    }

    pub fn switching_direction(&mut self) {
        self.direction *= -1;
        self.bouncing_force = self.direction * 10;

        self.iframe = self.iframe.max(10);
    }

    pub fn hit(&mut self) {
        self.water -= 100;
        self.iframe = 30;
    }

    pub fn is_invincible(&self) -> bool {
        self.iframe > 0
    }

    pub fn is_dead(&self) -> bool {
        self.water <= 0
    }
}
