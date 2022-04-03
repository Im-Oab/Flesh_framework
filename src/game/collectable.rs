use crate::core::sprite::Sprite;
use macroquad::prelude::*;

pub enum CollectableType {
    Water(i32),
}

pub struct Collctable {
    pub radius: f32,
    pub position: Vec2,
    pub collectable_type: CollectableType,
    collected_tick: u128,
    pub collected: bool,
}

impl Collctable {
    pub fn water(value: i32) -> Self {
        Self {
            radius: 32.0,
            position: Vec2::zero(),
            collectable_type: CollectableType::Water(value),
            collected_tick: 0,
            collected: false,
        }
    }
}

impl Collctable {
    pub fn update(&mut self, speed: f32, player_position: Vec2) {
        let distance = self.position.distance_squared(player_position);
        if self.collected_tick > 0 {
            self.collected_tick += crate::ONE_FRAME.as_millis();

            let percentage = self.collected_tick as f32 / 500.0;
            self.position = self.position.lerp(player_position, percentage);

            if self.collected_tick >= 500 || distance < self.radius * self.radius {
                self.collected = true;
            }
        } else {
            self.position.y += speed;

            let radius = self.radius * 3.0;
            if distance < (radius * radius) {
                self.closeby();
            }
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, GREEN);
    }

    pub fn closeby(&mut self) {
        if self.collected_tick == 0 {
            self.collected_tick = 300;
        }
    }
}
