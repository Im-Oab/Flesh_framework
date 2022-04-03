use macroquad::prelude::*;
use std::collections::VecDeque;

use crate::core::scene::{GameScene, SceneTransition};
use crate::core::sprite::Sprite;

use crate::game::player::Player;

const MAX_SPEED: f32 = 24.0;

pub struct GameplayScene {
    player: Player,
    obstacles: VecDeque<Obstacle>,
    speed: f32,
    spawn_cooldown: i32,
}

impl GameplayScene {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            obstacles: VecDeque::new(),
            speed: 0.0,
            spawn_cooldown: 0,
        }
    }
}

impl GameScene for GameplayScene {
    fn init(&mut self) {
        self.player.position = Vec2::new(screen_width() / 2.0, screen_height() - 100.0);
    }

    fn update(&mut self) -> Result<SceneTransition, i32> {
        let speed = self.speed;
        if rand::RandomRange::gen_range(0, 20) as i32 == 0 && self.spawn_cooldown <= 0 {
            self.spawn_cooldown = self.spawn_obstacle() * 2 / (self.speed as i32).max(1);
        }

        if self.spawn_cooldown > 0
        {
            self.spawn_cooldown -= 1;
        }

        for obstacle in self.obstacles.iter_mut() {
            obstacle.update(speed);
            if obstacle.is_hit(self.player.position, 8.0) {
                self.speed *= 0.5;
                obstacle.hit();

                let direction = {
                    if self.player.position.x < obstacle.position.x {
                        -1
                    } else {
                        1
                    }
                };

                self.player.external_force = direction * 20;
            }
        }

        self.player.update();

        self.speed += 0.4;
        self.speed = self.speed.min(MAX_SPEED);

        Ok(SceneTransition::None)
    }

    fn draw(&mut self) {
        clear_background(WHITE);

        for obstacle in self.obstacles.iter() {
            obstacle.draw();
        }

        self.player.draw();
    }
}

impl GameplayScene {
    fn spawn_obstacle(&mut self) -> i32 {
        let radius: i32 = rand::RandomRange::gen_range(10, 80);
        let pos_x = rand::RandomRange::gen_range(
            -radius,
            crate::GAME_WIDTH as i32 + radius * 2,
        );

        self.obstacles.push_front(Obstacle::new(
            Vec2::new(pos_x as f32, -10.0 - radius as f32),
            radius as f32,
        ));

        radius
    }
}


struct Obstacle {
    sprite: Sprite,
    radius: f32,
    position: Vec2,
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

    fn is_hit(&self, other: Vec2, other_radius: f32) -> bool {
        let dist = self.position.distance_squared(other);
        let size = self.radius + other_radius;
        if dist <= size * size && self.hit_cooldown == 0 {
            true
        } else {
            false
        }
    }

    fn hit(&mut self) {
        self.hit_cooldown = 120;
    }
}
