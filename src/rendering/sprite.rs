
use std::rc::Rc;
use crate::physics::Vec2;
use super::texture::Texture;

#[derive(Debug, Clone)]
pub struct Sprite {
    // Use Rc to share texture data between sprites
    texture: Rc<Texture>,
    // World position
    pub position: Vec2,
    // Scale factor (1.0 = original size)
    pub scale: f32,
    // Rotation in radians
    pub rotation: f32,
    // Layer for rendering order (higher = in front)
    pub layer: i32,
    // Sprite dimensions in world units
    pub width: f32,
    pub height: f32,
}

impl Sprite {
    pub fn new(texture: Rc<Texture>, position: Vec2, width: f32, height: f32) -> Self {
        Self {
            texture,
            position,
            scale: 1.0,
            rotation: 0.0,
            layer: 0,
            width,
            height,
        }
    }

    // Helper to create sprite with uniform scale based on desired world size
    pub fn new_world_scaled(texture: Rc<Texture>, position: Vec2, world_size: f32) -> Self {
        let aspect_ratio = texture.width as f32 / texture.height as f32;
        Self::new(
            texture,
            position,
            world_size * aspect_ratio,
            world_size,
        )
    }

    // Get color at relative coordinates (0,0 = top-left, 1,1 = bottom-right)
    pub fn sample(&self, rel_x: f32, rel_y: f32) -> [u8; 4] {
        let x = rel_x * self.texture.width as f32;
        let y = rel_y * self.texture.height as f32;
        self.texture.sample_bilinear(x, y, [0, 0, 0, 0])
    }

    // Get sprite bounds in world space
    pub fn bounds(&self) -> (Vec2, Vec2) {
        let half_width = self.width * self.scale / 2.0;
        let half_height = self.height * self.scale / 2.0;

        let min = Vec2::new(
            self.position.x - half_width,
            self.position.y - half_height
        );
        let max = Vec2::new(
            self.position.x + half_width,
            self.position.y + half_height
        );

        (min, max)
    }
}