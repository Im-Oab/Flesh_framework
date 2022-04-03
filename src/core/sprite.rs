use crate::core::animation::Animation;
use std::time::Duration;

use macroquad::prelude::*;

pub struct Sprite {
    pause: bool,

    is_loop: bool,

    frame_index: usize,

    animation: Animation,

    frame_duration: Duration,
}

impl Sprite {
    pub fn new() -> Self {
        Self {
            pause: false,
            is_loop: true,
            frame_index: 0,
            animation: Animation::new(),
            frame_duration: Duration::from_millis(0),
        }
    }

    pub fn reset(&mut self) {
        self.is_loop = true;
        self.pause = false;
        self.frame_index = 0;
        self.frame_duration = Duration::from_millis(0);
    }

    pub fn get_current_animation_name(&self) -> &String {
        &self.animation.name
    }

    pub fn is_loop(&self) -> bool {
        self.is_loop
    }

    pub fn is_end_of_animation(&self) -> bool {
        if self.pause && self.frame_index == self.animation.frames.len() - 1 {
            return true;
        }

        if self.animation.frames.is_empty() == false
            && self.frame_index > self.animation.frames.len() - 1
        {
            return true;
        }

        false
    }

    fn continue_loop(&mut self) {
        let frames = self.animation.frames.len();
        self.frame_index = self.frame_index % frames;
    }

    pub fn play(&mut self, new_animation: &Animation) {
        self.pause = false;
        self.frame_index = 0;
        self.frame_duration = Duration::from_millis(0);
        self.animation = new_animation.clone();
        println!("Sprite: play(): self.animation {:?}", self.animation.frames);
        println!("Sprite: play(): new_animation {:?}", new_animation.frames);
    }

    pub fn restart_animation(&mut self) {
        self.frame_duration = Duration::from_millis(0);
        self.frame_index = 0;
        self.pause = false;
    }

    fn advance(&mut self) -> bool {
        let frame_length = self.animation.frame_length;

        match self.frame_duration.checked_add(crate::ONE_FRAME) {
            Some(v) => self.frame_duration = v,
            None => self.frame_duration = Duration::from_millis(0),
        };

        if self.frame_duration >= frame_length && self.pause == false {
            while self.frame_duration >= frame_length {
                self.frame_duration -= frame_length;
                self.frame_index += 1;
            }

            if self.is_end_of_animation() == true {
                if self.is_loop == true {
                    self.continue_loop();
                } else {
                    self.frame_index = self.animation.frames.len() - 1;
                    self.pause = true;
                }

                return true;
            }
        }

        false
    }

    pub fn update(&mut self) -> bool {
        self.advance()
    }

    pub fn draw(&self, position: Vec2, rotation: f32) {
        let frame_index = self.frame_index;
        match self.animation.frames.get(frame_index) {
            Some(frame) => {
                let texture_manager = crate::TEXTURE_MANAGER.lock().unwrap();

                match texture_manager.get_texture_by_id(*frame) {
                    Some(texture) => {
                        let mut params = DrawTextureParams::default();
                        params.rotation = (rotation * 360.0).to_radians();
                        draw_texture_ex(
                            texture.clone(),
                            position.x - texture.width() / 2.0,
                            position.y - texture.height() / 2.0,
                            WHITE,
                            params,
                        );
                    }
                    None => {
                        // println!("Sprite: can't find texture by id: {}", frame);
                    }
                };
            }
            None => {
                // println!("Sprite: Animation:  can't find texture_id from index: {:?} {}", self.animation.frames, frame_index);
            }
        };
    }
}
