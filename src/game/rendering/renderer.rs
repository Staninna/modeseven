use super::super::camera::Camera;
use super::super::rendering::Renderable;
use crate::assets::{AssetManager, Texture};
use crate::game::world::World;

/// A Mode 7-style renderer for perspective-correct texture mapping
///
/// Implements an SNES-inspired renderer that provides:
/// * Ground plane perspective transformation
/// * Configurable camera with position, angle, and scale
/// * Bilinear texture sampling for ground plane and sprites
/// * Horizon rendering with solid background // TODO: Make background pretty
/// * Screen-to-world and back coordinate mapping
/// * Texture-mapped sprite rendering with rotation
///
/// Uses a camera height-based projection similar to F-Zero and Mario Kart.
pub struct Renderer {
    /// Texture used for the ground plane mapping
    ground_texture: Texture,
    /// Output viewport width in pixels
    viewport_width: u32,
    /// Output viewport height in pixels
    viewport_height: u32,
}

impl Renderer {
    /// Creates a new renderer with given dimensions and ground texture
    ///
    /// # Arguments
    ///
    /// * `viewport_width` - Output width in pixels
    /// * `viewport_height` - Output height in pixels
    /// * `ground_texture` - Ground plane texture for mapping
    ///
    /// # Returns
    ///
    /// Configured renderer for the specified dimensions
    pub fn new(viewport_width: u32, viewport_height: u32, ground_texture: Texture) -> Self {
        Self {
            ground_texture,
            viewport_width,
            viewport_height,
        }
    }

    /// Maps screen coordinates to world space
    ///
    /// Performs perspective projection using:
    /// * Camera height for z-depth calculation
    /// * Pitch angle for horizon determination
    /// * View angle for world rotation
    /// * Scale for world space sizing
    ///
    /// # Arguments
    ///
    /// * `screen_x` - X position in screen space
    /// * `screen_y` - Y position in screen space
    /// * `camera` - View transformation parameters
    ///
    /// # Returns
    ///
    /// World space coordinates if visible, None if occluded
    fn transform(&self, screen_x: f32, screen_y: f32, camera: &Camera) -> Option<(f32, f32)> {
        let x = (screen_x - self.viewport_width as f32 / 2.0) / self.viewport_width as f32 * 2.0;
        let y =
            (screen_y - (self.viewport_height as f32 / 2.0)) / self.viewport_height as f32 * 2.0;

        let horizon = camera.pitch.tan() * 0.5;
        if y < horizon {
            return None;
        }

        let z = camera.height / (y - horizon + 0.00001);
        if z <= camera.near || z >= camera.far {
            return None;
        }

        let world_x = x * z * camera.scale;
        let world_z = z;

        let (sin_angle, cos_angle) = camera.angle.sin_cos();
        let rotated_x = world_x * cos_angle - world_z * sin_angle;
        let rotated_z = world_x * sin_angle + world_z * cos_angle;

        Some((rotated_x + camera.x, rotated_z + camera.y))
    }

    /// Maps world space coordinates to screen space
    ///
    /// Performs inverse perspective projection:
    /// 1. Untranslate from camera position
    /// 2. Unrotate by camera angle
    /// 3. Project to screen space using camera parameters
    ///
    /// # Arguments
    ///
    /// * `world_x` - X position in world space
    /// * `world_y` - Y position in world space
    /// * `camera` - View transformation parameters
    ///
    /// # Returns
    ///
    /// Screen space coordinates if visible, None if occluded
    fn untransform(&self, world_x: f32, world_y: f32, camera: &Camera) -> Option<(f32, f32)> {
        let untranslated_x = world_x - camera.x;
        let untranslated_y = world_y - camera.y;

        let (sin_angle, cos_angle) = camera.angle.sin_cos();
        let unrotated_x = untranslated_x * cos_angle + untranslated_y * sin_angle;
        let unrotated_y = -untranslated_x * sin_angle + untranslated_y * cos_angle;

        let z = unrotated_y;
        if z <= camera.near || z >= camera.far {
            return None;
        }

        let scaled_x = unrotated_x / (z * camera.scale);
        let horizon = camera.pitch.tan() * 0.5;
        let projected_y = horizon + camera.height / z;

        if projected_y < horizon {
            return None;
        }

        let screen_x = (scaled_x + 1.0) * self.viewport_width as f32 / 2.0;
        let screen_y = (projected_y + 1.0) * self.viewport_height as f32 / 2.0;

        if screen_x < 0.0
            || screen_x >= self.viewport_width as f32
            || screen_y < 0.0
            || screen_y >= self.viewport_height as f32
        {
            return None;
        }

        Some((screen_x, screen_y))
    }

    /// Generic render function for any renderable entity
    ///
    /// Handles perspective projection and texture mapping for any
    /// object implementing the Renderable trait.
    ///
    /// # Arguments
    ///
    /// * `frame` - RGBA pixel buffer for output
    /// * `entity` - Any type implementing Renderable
    /// * `world` - Game world state for context
    /// * `camera` - View transformation parameters
    /// * `assets` - Asset manager for texture loading
    fn render_entity<T: Renderable>(
        &self,
        frame: &mut [u8],
        entity: &T,
        world: &World,
        camera: &Camera,
        assets: &AssetManager,
    ) {
        let pos = entity.position();

        // Calculate distance and scaling
        let dx = pos.x - camera.x;
        let dy = pos.y - camera.y;
        let distance = (dx * dx + dy * dy).sqrt();

        let reference_distance = 100.0;
        let min_size = 5.0;

        let scale_factor = (reference_distance / distance).min(4.0).max(0.25);
        let entity_size = (entity.base_size() * scale_factor).max(min_size) as u32;

        if let Some((screen_x, screen_y)) = self.untransform(pos.x, pos.y, camera) {
            let start_x = (screen_x - entity_size as f32 / 2.0).max(0.0) as u32;
            let start_y = (screen_y - entity_size as f32 / 2.0).max(0.0) as u32;
            let end_x = (start_x + entity_size).min(self.viewport_width);
            let end_y = (start_y + entity_size).min(self.viewport_height);

            let texture = assets.get_texture(entity.texture_file(world));

            for y in start_y..end_y {
                for x in start_x..end_x {
                    let tex_x =
                        ((x - start_x) as f32 / entity_size as f32) * texture.width() as f32;
                    let tex_y =
                        ((y - start_y) as f32 / entity_size as f32) * texture.height() as f32;

                    let color = texture.sample_bilinear(tex_x, tex_y, [0, 0, 0, 0]);

                    if color[3] > 0 {
                        let idx = ((y * self.viewport_width + x) * 4) as usize;
                        frame[idx..idx + 4].copy_from_slice(&color);
                    }
                }
            }
        }
    }

    /// Renders the perspective-mapped ground plane
    ///
    /// Implements Mode 7-style rendering:
    /// * Maps screen pixels to texture coordinates
    /// * Uses bilinear filtering for texture sampling
    /// * Renders horizon in solid color
    ///
    /// # Arguments
    ///
    /// * `frame` - RGBA pixel buffer for output
    /// * `camera` - View transformation parameters
    fn render_ground(&self, frame: &mut [u8], camera: &Camera) {
        for y in 0..self.viewport_height {
            for x in 0..self.viewport_width {
                let screen_x = x as f32;
                let screen_y = y as f32;

                let color =
                    if let Some((world_x, world_y)) = self.transform(screen_x, screen_y, camera) {
                        self.ground_texture.sample_bilinear(
                            world_x,
                            world_y,
                            [255, 105, 180, 255], // Hotpink for out-of-bounds
                        )
                    } else {
                        [255, 0, 255, 255] // Magenta for horizon
                    };

                let idx = ((y * self.viewport_width + x) * 4) as usize;
                frame[idx..idx + 4].copy_from_slice(&color);
            }
        }
    }

    /// Renders a complete frame with ground plane, horizon, and entities
    ///
    /// # Arguments
    ///
    /// * `frame` - RGBA pixel buffer (width * height * 4 bytes)
    /// * `world` - Game world containing entities to render
    /// * `camera` - Current camera parameters
    /// * `assets` - Asset manager for texture loading
    ///
    /// # Panics
    ///
    /// If frame buffer size doesn't match viewport dimensions
    pub fn render(&self, frame: &mut [u8], world: &World, camera: &Camera, assets: &AssetManager) {
        assert_eq!(
            frame.len(),
            (self.viewport_width * self.viewport_height * 4) as usize
        );

        self.render_ground(frame, camera);

        // Render all cars using the generic render_entity function
        for car in &world.cars {
            self.render_entity(frame, car, world, camera, assets);
        }
    }
}
