use crate::assets::Texture;
use crate::camera::Camera;
use crate::world::{Car, World};

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
        // Untranslate by camera position
        let untranslated_x = world_x - camera.x;
        let untranslated_y = world_y - camera.y;

        // Unrotate by camera angle
        let (sin_angle, cos_angle) = camera.angle.sin_cos();
        let unrotated_x = untranslated_x * cos_angle + untranslated_y * sin_angle;
        let unrotated_y = -untranslated_x * sin_angle + untranslated_y * cos_angle;

        // Calculate z-depth (distance from camera)
        let z = unrotated_y;

        // Check if point is within view frustum
        if z <= camera.near || z >= camera.far {
            return None;
        }

        // Project to NDC space
        let scaled_x = unrotated_x / (z * camera.scale);

        // Calculate y position based on z and horizon
        let horizon = camera.pitch.tan() * 0.5;
        let projected_y = horizon + camera.height / z;

        // Check if point is above horizon
        if projected_y < horizon {
            return None;
        }

        // Convert from NDC to screen space
        let screen_x = (scaled_x + 1.0) * self.viewport_width as f32 / 2.0;
        let screen_y = (projected_y + 1.0) * self.viewport_height as f32 / 2.0;

        // Check if point is within viewport bounds
        if screen_x < 0.0
            || screen_x >= self.viewport_width as f32
            || screen_y < 0.0
            || screen_y >= self.viewport_height as f32
        {
            return None;
        }

        Some((screen_x, screen_y))
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
    pub fn render(&self, frame: &mut [u8], world: &World, camera: &Camera) {
        assert_eq!(
            frame.len(),
            (self.viewport_width * self.viewport_height * 4) as usize
        );

        // First render the ground
        self.render_ground(frame, camera);

        // Then render the cars
        self.render_car(frame, &world.cars[0], camera);
        self.render_car(frame, &world.cars[1], camera);
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

    // TODO: Unify this function to render all kinds of entities
    /// Renders the car sprite onto the frame buffer with distance-based scaling
    ///
    /// # Arguments
    ///
    /// * `frame` - RGBA pixel buffer for output
    /// * `car` - The car to render
    /// * `camera` - View transformation parameters
    fn render_car(&self, frame: &mut [u8], car: &Car, camera: &Camera) {
        // Get car position in world space
        let car_pos = car.position();

        // Calculate distance from camera to car
        let dx = car_pos.x - camera.x;
        let dy = car_pos.y - camera.y;
        let distance = (dx * dx + dy * dy).sqrt();

        // Base size and scaling factor
        let base_size = 60.0; // Base size when at reference distance
        let reference_distance = 100.0; // Distance at which car is at base size
        let min_size = 5.0; // Minimum size to prevent car from disappearing

        // Calculate scaled size based on distance
        // Using inverse relationship with distance, clamped to minimum size
        let scale_factor = (reference_distance / distance).min(4.0).max(0.25);
        let car_size = (base_size * scale_factor).max(min_size) as u32;

        // Transform car position to screen space
        if let Some((screen_x, screen_y)) = self.untransform(car_pos.x, car_pos.y, camera) {
            // Calculate bounds for the car sprite
            let start_x = (screen_x - car_size as f32 / 2.0).max(0.0) as u32;
            let start_y = (screen_y - car_size as f32 / 2.0).max(0.0) as u32;
            let end_x = (start_x + car_size).min(self.viewport_width);
            let end_y = (start_y + car_size).min(self.viewport_height);

            // Choose color based on which car (assuming index 0 is red, 1 is blue)
            let car_color = if car.speed() > 1.0 {
                [255, 0, 0, 255] // Moving - red
            } else {
                [0, 0, 255, 255] // Stationary - blue
            };

            // Draw the car as a scaled square
            for y in start_y..end_y {
                for x in start_x..end_x {
                    let idx = ((y * self.viewport_width + x) * 4) as usize;
                    frame[idx..idx + 4].copy_from_slice(&car_color);
                }
            }
        }
    }
}
