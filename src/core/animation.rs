use std::collections::HashMap;
use std::time::Duration;

/// Contain texture_id (u128) in `frames` and use it for playing animation.
/// texture_id will link to texture on crate::TEXTURE_MANAGER
/// The Animation is member of `Sprite`.
pub struct Animation {
    pub frame_length: Duration,
    pub frames: Vec<u128>,
    pub name: String,
}

impl Clone for Animation {
    fn clone(&self) -> Animation {
        Animation {
            frame_length: self.frame_length,
            frames: self.frames.clone(),
            name: self.name.clone(),
        }
    }
}

impl Animation {
    pub fn new() -> Self {
        Self {
            frame_length: Duration::from_millis(1000 / 18),
            frames: vec![],
            name: "".to_owned(),
        }
    }

    pub fn from_frames(name: &str, frame_names: Vec<&str>) -> Self {
        let mut frames = vec![];
        let texture_manager = crate::TEXTURE_MANAGER.lock().unwrap();
        for name in frame_names.iter() {
            if let Some(id) = texture_manager.get_texture_id(name) {
                frames.push(*id);
            } else {
                println!(
                    "Animation:from_frames(): can't find texture_id by name: {}",
                    name
                );
            }
        }

        Self {
            frame_length: Duration::from_millis(1000 / 18),
            frames: frames,
            name: name.to_owned(),
        }
    }
}

pub struct AnimationManager {
    pub animations: HashMap<String, Animation>,
}

impl AnimationManager {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, frame_names: Vec<&str>) {
        let animation = Animation::from_frames(name, frame_names);
        if animation.frames.len() > 0 {
            self.animations.insert(name.to_owned(), animation);
            println!("AnimationManger: add(): success: {}", name);
        } else {
            println!("AnimationManger: add(): failed: {}", name);
        }
    }

    pub fn get(&self, name: &str) -> Option<&Animation> {
        self.animations.get(name)
    }
}
