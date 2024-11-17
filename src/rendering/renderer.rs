use super::texture::Texture;
use crate::camera::Camera;

pub struct Renderer {
    ground_texture: Texture,
    screen_width: u32,
    screen_height: u32,
}

impl Renderer {
    pub fn new(screen_width: u32, screen_height: u32, ground_texture: Texture) -> Self {
        Self {
            ground_texture,
            screen_width,
            screen_height,
        }
    }

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

    pub fn render(&self, frame: &mut [u8], camera: &Camera) {
        self.render_ground(frame, camera);
    }

    // Render ground texture
    pub fn render_ground(&self, frame: &mut [u8], camera: &Camera) {
        let height = (frame.len() / (self.screen_width as usize * 4)) as u32;
        debug_assert_eq!(frame.len(), (self.screen_width * height * 4) as usize);

        for y in 0..self.screen_height {
            for x in 0..self.screen_width {
                let screen_x = x as f32;
                let screen_y = y as f32;

                let color = if let Some((world_x, world_y)) = self.transform(screen_x, screen_y, camera) {
                    self.ground_texture.sample_octinear(world_x, world_y, /* hotpink */ [255, 105, 180, 255])
                } else {
                    // Horizon/background is hotpink
                    [255, 0, 255, 255]
                };

                let idx = ((y * self.screen_width + x) * 4) as usize;
                frame[idx..idx + 4].copy_from_slice(&color);
            }
        }
    }
}
