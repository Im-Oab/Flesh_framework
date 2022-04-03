use macroquad::texture::{load_texture, Texture2D};
use std::collections::HashMap;

pub struct TextureManager {
    counter: u128,
    pub textures: HashMap<u128, Texture2D>,
    names: HashMap<String, u128>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            counter: 1,
            textures: HashMap::new(),
            names: HashMap::new(),
        }
    }

    pub async fn load_from_file(&mut self, name: &str, path: &str) -> u128 {
        match load_texture(path).await {
            Ok(texture) => {
                let texture_id = self.counter;
                self.textures.insert(texture_id, texture);
                self.names.insert(name.to_owned(), texture_id);
                self.counter += 1;

                println!("Loaded texture: {}, {}, {}", texture_id, name, path);
                texture_id
            }
            Err(e) => {
                println!("Load texture error: {}", e);
                0
            }
        }
    }

    pub fn get_texture_by_id(&self, texture_id: u128) -> Option<&Texture2D> {
        self.textures.get(&texture_id)
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        match self.names.get(name) {
            Some(id) => self.textures.get(id),
            None => None,
        }
    }

    pub fn get_texture_id(&self, name: &str) -> Option<&u128> {
        self.names.get(name)
    }
}
