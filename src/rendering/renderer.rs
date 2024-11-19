use super::texture::Texture;
use crate::camera::Camera;

/// A Mode 7-style renderer for perspective-correct texture mapping
///
/// Implements an SNES-inspired renderer that provides:
/// * Ground plane perspective transformation
/// * Configurable camera with position, angle, and scale
/// * Bilinear texture sampling for ground plane
/// * Horizon rendering with solid background // TODO: Make background pretty
/// * Screen-to-world and back coordinate mapping
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
        // Convert to NDC space (-1 to 1)
        let x = (screen_x - self.viewport_width as f32 / 2.0) / self.viewport_width as f32 * 2.0;
        let y =
            (screen_y - (self.viewport_height as f32 / 2.0)) / self.viewport_height as f32 * 2.0;

        // Check if point is above horizon line
        let horizon = camera.pitch.tan() * 0.5;
        if y < horizon {
            return None;
        }

        // Calculate z-depth from y position
        let z = camera.height / (y - horizon + 0.00001);
        if z <= camera.near || z >= camera.far {
            return None;
        }

        // Project point to world space
        let world_x = x * z * camera.scale;
        let world_z = z;

        // Rotate by camera angle
        let (sin_angle, cos_angle) = camera.angle.sin_cos();
        let rotated_x = world_x * cos_angle - world_z * sin_angle;
        let rotated_z = world_x * sin_angle + world_z * cos_angle;

        // Translate by camera position
        Some((rotated_x + camera.x, rotated_z + camera.y))
    }

    /// Maps world space coordinates to screen space
    ///
    /// Performs perspective projection using:
    /// * TODO
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
        todo!()
    }

    /// Renders a complete frame with ground plane and horizon
    ///
    /// # Arguments
    ///
    /// * `frame` - RGBA pixel buffer (width * height * 4 bytes)
    /// * `camera` - Current camera parameters
    ///
    /// # Panics
    ///
    /// If frame buffer size doesn't match viewport dimensions
    pub fn render(&self, frame: &mut [u8], camera: &Camera) {
        assert_eq!(
            frame.len(),
            (self.viewport_width * self.viewport_height * 4) as usize
        );

        self.render_ground(frame, camera);
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
    pub fn render_ground(&self, frame: &mut [u8], camera: &Camera) {
        for y in 0..self.viewport_height {
            for x in 0..self.viewport_width {
                let screen_x = x as f32;
                let screen_y = y as f32;

                // Transform point and sample texture or use horizon color
                let color =
                    if let Some((world_x, world_y)) = self.transform(screen_x, screen_y, camera) {
                        // Sample ground texture with hotpink background
                        self.ground_texture.sample_bilinear(
                            world_x,
                            world_y,
                            [255, 105, 180, 255], // Hotpink for out-of-bounds
                        )
                    } else {
                        [255, 0, 255, 255] // Magenta for horizon
                    };

                // Write color to frame buffer
                let idx = ((y * self.viewport_width + x) * 4) as usize;
                frame[idx..idx + 4].copy_from_slice(&color);
            }
        }
    }
}
