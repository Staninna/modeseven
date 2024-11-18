use super::texture::Texture;
use crate::camera::Camera;

/// An SNES Mode 7-style renderer that performs perspective-correct texture mapping
///
/// This renderer implements a style of rendering similar to the Super Nintendo's Mode 7,
/// which was famously used in games like F-Zero and Super Mario Kart. It performs
/// perspective-correct texture mapping of a ground plane using a movable camera.
///
/// The renderer transforms screen coordinates to world coordinates using the camera's
/// position, angle, and scale, then samples a ground texture using bilinear filtering.
/// Areas above the horizon are rendered in a solid color.
///
/// # Example
///
/// ```rust
/// let texture = Texture::load("ground.png");
/// let renderer = Renderer::new(640, 480, texture);
/// let camera = Camera::new();
/// let mut frame_buffer = vec![0u8; 640 * 480 * 4];
///
/// renderer.render(&mut frame_buffer, &camera);
/// ```
pub struct Renderer {
    ground_texture: Texture,
    screen_width: u32, // TODO: Rename to view_port_width (keep in mind the docstrings)
    screen_height: u32, // TODO: Rename to view_port_height (keep in mind the docstrings)
}

impl Renderer {
    /// Creates a new renderer with the specified screen dimensions and ground texture
    ///
    /// # Arguments
    ///
    /// * `screen_width` - The width of the output frame in pixels
    /// * `screen_height` - The height of the output frame in pixels
    /// * `ground_texture` - The texture to use for the ground plane
    ///
    /// # Returns
    ///
    /// A new `Renderer` instance configured with the provided parameters
    pub fn new(screen_width: u32, screen_height: u32, ground_texture: Texture) -> Self {
        Self {
            ground_texture,
            screen_width,
            screen_height,
        }
    }

    /// Transform screen coordinates to world coordinates
    ///
    /// This function performs perspective transformation from screen space to world space
    /// using the camera's parameters. Points above the horizon or outside the view frustum
    /// return None.
    ///
    /// The transformation process:
    /// 1. Converts screen coordinates to normalized device coordinates (-1 to 1)
    /// 2. Calculates the intersection with the ground plane
    /// 3. Applies the camera's rotation, scale, and translation
    ///
    /// # Arguments
    ///
    /// * `screen_x` - The x-coordinate in screen space (pixels from left)
    /// * `screen_y` - The y-coordinate in screen space (pixels from top)
    /// * `camera` - The camera defining the view transformation
    ///
    /// # Returns
    ///
    /// * `Some((x, y))` - The transformed world coordinates if the point is visible
    /// * `None` - If the point is above the horizon or outside the view frustum
    fn transform(&self, screen_x: f32, screen_y: f32, camera: &Camera) -> Option<(f32, f32)> {
        // Convert screen coordinates to normalized device coordinates (-1 to 1)
        let x = (screen_x - self.screen_width as f32 / 2.0) / self.screen_width as f32 * 2.0;
        let y = (screen_y - (self.screen_height as f32 / 2.0)) / self.screen_height as f32 * 2.0;

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

        let sin_angle = camera.angle.sin();
        let cos_angle = camera.angle.cos();

        let rotated_x = world_x * cos_angle - world_z * sin_angle;
        let rotated_z = world_x * sin_angle + world_z * cos_angle;

        Some((
            rotated_x + camera.x,
            rotated_z + camera.y,
        ))
    }

    /// Transform world coordinates to screen coordinates
    ///
    /// This function performs the inverse of the transform function, converting world
    /// coordinates back to screen space using the camera's parameters. Points outside
    /// the view frustum return None.
    ///
    /// The transformation process:
    /// TODO: Write this
    ///
    /// # Arguments
    ///
    /// * `world_x` - The x-coordinate in world space
    /// * `world_y` - The y-coordinate in world space
    /// * `camera` - The camera defining the view transformation
    ///
    /// # Returns
    ///
    /// * `Some((x, y))` - The transformed screen coordinates if the point is visible
    /// * `None` - If the point is outside the view frustum
    fn untransform(&self, world_x: f32, world_y: f32, camera: &Camera) -> Option<(f32, f32)> {
        todo!()
    }

    /// Renders a 'complete' frame using the current camera settings
    ///
    /// This method renders the entire scene, including the ground plane, from the perspective
    /// of the provided camera. The frame buffer is updated with the rendered pixels in RGBA format.
    ///
    /// # Arguments
    ///
    /// * `frame` - A mutable slice containing the pixel buffer to render into. Must be exactly
    ///             screen_width * screen_height * 4 bytes in size (RGBA format)
    /// * `camera` - The camera defining the viewpoint and perspective transformation parameters
    ///
    /// # Panics
    ///
    /// Panics if the frame buffer size does not match screen_width * screen_height * 4 bytes
    pub fn render(&self, frame: &mut [u8], camera: &Camera) {
        assert_eq!(frame.len(), (self.screen_width * self.screen_height * 4) as usize);

        self.render_ground(frame, camera);
    }

    /// Renders the ground plane using Mode 7-style perspective transformation
    ///
    /// This method performs perspective-correct texture mapping of the ground plane texture,
    /// implementing an SNES Mode 7-style renderer. For each screen pixel, it:
    /// 1. Transforms the screen coordinates to world space using the camera parameters
    /// 2. Samples the ground texture at the transformed coordinates
    /// 3. Writes the sampled color to the frame buffer
    ///
    /// # Details
    ///
    /// The rendering process:
    /// - Points above the horizon are rendered in pink ([255, 0, 255, 255])
    /// - Ground plane pixels are sampled from the texture using bilinear filtering
    /// - Default color for out-of-bounds texture samples is hotpink ([255, 105, 180, 255])
    /// - The output is written as RGBA pixels (4 bytes per pixel)
    ///
    /// # Arguments
    ///
    /// * `frame` - A mutable slice containing the pixel buffer to render into. Must be exactly
    ///             screen_width * screen_height * 4 bytes in size (RGBA format)
    /// * `camera` - The camera defining the viewpoint and perspective transformation parameters
    pub fn render_ground(&self, frame: &mut [u8], camera: &Camera) {
        for y in 0..self.screen_height {
            for x in 0..self.screen_width {
                let screen_x = x as f32;
                let screen_y = y as f32;

                let color = if let Some((world_x, world_y)) = self.transform(screen_x, screen_y, camera) {
                    self.ground_texture.sample_bilinear(world_x, world_y, /* hotpink */ [255, 105, 180, 255])
                } else {
                    // Horizon/background is hotpink
                    [255, 0, 255, 255]
                };

                let idx = ((y * self.screen_width + x) * 4) as usize;
                frame[idx..idx + 4].copy_from_slice(&color);
            }
        }
    }

    // TODO: Render cars/other objects
}
