use lerp::Lerp;
use macroquad::prelude::*;
use std::collections::VecDeque;

use crate::core::scene::{GameScene, SceneTransition};

use crate::game::collectable::{Collctable, CollectableType};
use crate::game::obstacle::Obstacle;
use crate::game::player::Player;

const MAX_SPEED: f32 = 32.0;

enum GameplayState {
    Start,
    Playing,
    GameOver,
}

pub struct GameplayScene {
    state: GameplayState,
    tick: u128,

    surface_distance: f32,
    player: Player,
    obstacles: VecDeque<Obstacle>,
    item_tick: u128,
    items: VecDeque<Collctable>,
    speed: f32,
    spawn_cooldown: i32,
}

impl GameplayScene {
    pub fn new() -> Self {
        Self {
            state: GameplayState::Start,
            tick: 3000,
            surface_distance: 1000.0,

            player: Player::new(),
            obstacles: VecDeque::new(),
            item_tick: 2000,
            items: VecDeque::new(),
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
        match self.state {
            GameplayState::Start => match self.tick.checked_sub(crate::ONE_FRAME.as_millis()) {
                Some(v) => {
                    self.tick = v;
                }
                None => {
                    self.state = GameplayState::Playing;
                }
            },
            GameplayState::Playing => {
                return self.update_playing();
            }
            GameplayState::GameOver => {
                self.tick += crate::ONE_FRAME.as_millis();

                if is_key_down(KeyCode::Space) && self.tick > 300 {
                    let mut scene = GameplayScene::new();
                    scene.init();
                    return Ok(SceneTransition::Replace(Box::new(scene)));
                }
            }
        }

        Ok(SceneTransition::None)
    }

    fn draw(&mut self) {
        clear_background(WHITE);

        for obstacle in self.obstacles.iter() {
            obstacle.draw();
        }

        for item in self.items.iter_mut() {
            item.draw();
        }

        self.player.draw();

        self.draw_ui();

        if matches!(self.state, GameplayState::Start) {
            let seconds = (self.tick as f32 / 1000.0).ceil() as u128;
            crate::core::utils::draw_text_center(
                format!("{}", seconds).as_str(),
                96,
                crate::GAME_HEIGHT / 2.0,
            );
        } else if matches!(self.state, GameplayState::GameOver) {
            crate::core::utils::draw_text_center("GAME OVER", 72, crate::GAME_HEIGHT / 2.0);
            crate::core::utils::draw_text_center("`Space` for retry", 32, crate::GAME_HEIGHT * 0.6);
        }
    }
}

impl GameplayScene {
    fn update_playing(&mut self) -> Result<SceneTransition, i32> {
        let speed = self.speed;
        if rand::RandomRange::gen_range(0, 20) as i32 == 0 && self.spawn_cooldown <= 0 {
            self.spawn_cooldown = self.spawn_obstacle() * 2 / (self.speed as i32).max(1);
        }

        self.spawn_item();

        if self.spawn_cooldown > 0 {
            self.spawn_cooldown -= 1;
        }

        for obstacle in self.obstacles.iter_mut() {
            obstacle.update(speed);
            if self.player.is_invincible() == false && obstacle.is_hit(self.player.position, 8.0) {
                obstacle.hit();

                if self.speed > MAX_SPEED / 2.0 {
                    self.speed = -self.speed * 0.6;
                } else {
                    self.speed *= 0.1;
                }

                let direction = {
                    if self.player.position.x < obstacle.position.x {
                        -1
                    } else {
                        1
                    }
                };

                self.player.external_force = direction * 30;
            }
        }

        let mut removed_list = vec![];

        for index in 0..self.items.len() {
            let mut item = self.items.get_mut(index).unwrap();
            item.update(speed, self.player.position);

            if item.collected == true {
                if let CollectableType::Water(value) = item.collectable_type {
                    self.player.water += value;
                    self.player.water = self.player.water.min(5000);
                }
                removed_list.push(index);
            }
        }

        for index in removed_list.iter().rev() {
            self.items.remove(*index);
        }

        self.player.update();

        self.speed += 0.4;
        self.speed = self.speed.min(MAX_SPEED);

        self.surface_distance -= (self.speed) * 0.01;

        if self.player.is_dead() {
            self.state = GameplayState::GameOver;
            self.tick = 0;
        } else if self.surface_distance <= 0.0 {
            let mut scene = crate::scenes::ending::EndingScene::new();
            scene.init();

            return Ok(SceneTransition::Replace(Box::new(scene)));
        }

        Ok(SceneTransition::None)
    }
    fn spawn_obstacle(&mut self) -> i32 {
        let radius: i32 = rand::RandomRange::gen_range(10, 80);
        let pos_x = rand::RandomRange::gen_range(-radius, crate::GAME_WIDTH as i32 + radius * 2);

        self.obstacles.push_front(Obstacle::new(
            Vec2::new(pos_x as f32, -10.0 - radius as f32),
            radius as f32,
        ));

        radius
    }

    fn spawn_item(&mut self) {
        match self.item_tick.checked_sub(crate::ONE_FRAME.as_millis()) {
            Some(v) => {
                self.item_tick = v;
            }
            None => {
                self.item_tick = rand::RandomRange::gen_range(1000i32, 1500i32) as u128;
                let pos_x =
                    rand::RandomRange::gen_range(10 + 32, crate::GAME_WIDTH as i32 - (10 + 32));

                let mut item = Collctable::water(5000);
                item.position = Vec2::new(pos_x as f32, 10.0 - item.radius);
                self.items.push_back(item);
            }
        }
    }
}

impl GameplayScene {
    fn draw_ui(&self) {
        let metre = self.surface_distance.ceil() as i32;
        crate::core::utils::draw_text_center(format!("{} cm", metre).as_str(), 72, 100.0);

        let percentage = self.player.water as f32 / 5000.0;
        let width = crate::GAME_WIDTH - 20.0;
        let current_width = 0f32.lerp(width, percentage);
        draw_line(
            10.0,
            crate::GAME_HEIGHT - 20.0,
            crate::GAME_WIDTH - 10.0,
            crate::GAME_HEIGHT - 20.0,
            8.0,
            BLACK,
        );
        draw_line(
            10.0,
            crate::GAME_HEIGHT - 20.0,
            10.0 + current_width,
            crate::GAME_HEIGHT - 20.0,
            8.0,
            SKYBLUE,
        );
    }
}
