use crate::core::sprite::Sprite;
use macroquad::prelude::*;

pub struct Obstacle {
    sprite: Sprite,
    radius: f32,
    pub position: Vec2,
    velocity: Vec2,
    hit_cooldown: i32,
}

impl Obstacle {
    pub fn new(position: Vec2, radius: f32) -> Self {
        Self {
            sprite: Sprite::new(),
            radius: radius,
            position: position,
            velocity: Vec2::zero(),
            hit_cooldown: 0,
        }
    }

    pub fn update(&mut self, speed: f32) {
        self.sprite.update();

        self.position += self.velocity;
        self.position.y += speed;

        if self.hit_cooldown > 0 {
            self.hit_cooldown -= 1;
        }
    }

    pub fn draw(&self) {
        self.sprite.draw(self.position, 0.0);

        draw_circle(self.position.x, self.position.y, self.radius, RED);
    }

    pub fn is_hit(&self, other: Vec2, other_radius: f32) -> bool {
        let dist = self.position.distance_squared(other);
        let size = self.radius + other_radius;
        if dist <= size * size && self.hit_cooldown == 0 {
            true
        } else {
            false
        }
    }

    pub fn hit(&mut self) {
        self.hit_cooldown = 120;
    }
}
