use std::collections::BTreeMap;
use std::rc::Rc;
use anyhow::Result;
use super::{Sprite, Texture};

/// SpriteManager is a simple manager for sprites
/// It keeps track of loaded textures and sorts sprites by layer
/// It also provides a method to load textures
///
/// # Example
/// ```
/// let mut sprite_manager = SpriteManager::new();
/// let texture = sprite_manager.load_texture("assets/sprite.png").unwrap();
/// let sprite = Sprite::new(texture, 0, 0, 100, 100);
/// sprite_manager.add_sprite(sprite);
/// ...
/// ...
/// // Somewhere else in the code
/// for sprite in sprite_manager.get_sprites() {
///     // Render sprite
/// }
/// ```
pub struct SpriteManager {
    // Use BTreeMap to automatically sort sprites by layer
    sprites: BTreeMap<i32, Vec<Sprite>>,
    // Cache of loaded textures
    textures: Vec<Rc<Texture>>,
}

impl SpriteManager {
    pub fn new() -> Self {
        Self {
            sprites: BTreeMap::new(),
            textures: Vec::new(),
        }
    }

    // Load a texture and return an Rc to it
    pub fn load_texture(&mut self, path: &str) -> Result<Rc<Texture>> {
        // check if texture is already loaded
        for texture in self.textures.iter() {
            if texture.path == path {
                return Ok(Rc::clone(texture));
            }
        }

        let texture = Rc::new(Texture::from_image(path)?);
        self.textures.push(Rc::clone(&texture));
        Ok(texture)
    }

    // Add a sprite to be rendered
    pub fn add_sprite(&mut self, sprite: Sprite) {
        let layer = sprite.layer;
        self.sprites.entry(layer).or_default().push(sprite);
    }

    // Get all sprites sorted by layer
    pub fn get_sprites(&self) -> impl Iterator<Item = &Sprite> {
        self.sprites.values().flatten()
    }

    // Clear all sprites (but keep loaded textures)
    pub fn clear_sprites(&mut self) {
        self.sprites.clear();
    }
}