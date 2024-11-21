use crate::world::World;
use glam::Vec2;

/// Trait for objects that can be rendered in the game world
pub trait Renderable {
    /// Get the position of the entity in world space
    fn position(&self) -> Vec2;
    /// Get the rotation angle in radians
    fn angle(&self) -> f32;
    /// Get the base size for rendering
    fn base_size(&self) -> f32;
    /// Get the texture filename for this entity
    fn texture_file(&self, world: &World) -> &str;
}
