use anyhow::Result;
use image::GenericImageView;
use std::path::Path;

/// A 2D texture with RGBA pixels and sampling support
///
/// Texture provides:
/// * RGBA pixel storage (4 bytes per pixel)
/// * Nearest-neighbor and bilinear sampling
/// * Image file loading with format conversion
/// * Debug checkerboard pattern generation
///
/// Non-RGBA images are automatically converted during loading.
#[derive(Debug, Clone)]
pub struct Texture {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Raw RGBA pixel data
    pub pixels: Vec<u8>,
    /// Source identifier
    pub path: String,
}

impl Texture {
    /// Creates a test checkerboard pattern texture
    ///
    /// # Arguments
    ///
    /// * `width` - Texture width in pixels
    /// * `height` - Texture height in pixels
    /// * `checker_size` - Size of each checker square
    ///
    /// # Returns
    ///
    /// A new black and white checkerboard texture
    pub fn checkerboard(width: u32, height: u32, checker_size: u32) -> Self {
        let mut pixels = Vec::with_capacity((width * height * 4) as usize);

        for y in 0..height {
            for x in 0..width {
                let checker = ((x / checker_size + y / checker_size) % 2) as u8;
                let color = if checker == 0 { 255 } else { 0 };
                pixels.extend_from_slice(&[color, color, color, 255]);
            }
        }

        Self {
            width,
            height,
            pixels,
            path: format!(
                "checkerboard_{}x{}-{}x{}.png",
                width, height, checker_size, checker_size
            ),
        }
    }

    /// Loads a texture from an image file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to image file
    ///
    /// # Returns
    ///
    /// A new texture from the loaded image, or an error
    pub fn from_image<P: AsRef<Path> + ToString>(path: P) -> Result<Self> {
        let image = image::open(&path)?;
        let (width, height) = image.dimensions();
        let bytes_per_pixel = image.color().bytes_per_pixel();

        let pixels: Vec<u8> = /* TODO: This is really hacky */ match bytes_per_pixel {
            3 => {
                let pixels_rgb = image.as_bytes().to_vec();
                pixels_rgb
                    .chunks_exact(3)
                    .flat_map(|chunk| vec![chunk[0], chunk[1], chunk[2], 255])
                    .collect()
            }
            4 => image.as_bytes().to_vec(),
            _ => anyhow::bail!(
                "Unsupported image format with {} bytes per pixel",
                bytes_per_pixel
            ),
        };

        Ok(Self {
            width,
            height,
            pixels,
            path: path.to_string(),
        })
    }

    /// Samples a pixel using nearest-neighbor interpolation
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate (0 to width)
    /// * `y` - Y coordinate (0 to height)
    /// * `bg_color` - Color for out-of-bounds samples
    ///
    /// # Returns
    ///
    /// RGBA color array at the sampled position
    pub fn sample(&self, x: f32, y: f32, bg_color: [u8; 4]) -> [u8; 4] {
        // Check if coordinates are within bounds
        if x < 0.0 || x >= self.width as f32 || y < 0.0 || y >= self.height as f32 {
            return bg_color;
        }

        // Convert to integers for nearest-neighbor sampling
        let x = x as u32;
        let y = y as u32;

        // Calculate pixel index in RGBA array
        let idx = ((y * self.width + x) * 4) as usize;

        [
            self.pixels[idx],
            self.pixels[idx + 1],
            self.pixels[idx + 2],
            self.pixels[idx + 3],
        ]
    }

    /// Samples a pixel using bilinear interpolation
    ///
    /// Smoothly interpolates between four adjacent pixels based
    /// on the fractional coordinate values.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate (0 to width)
    /// * `y` - Y coordinate (0 to height)
    /// * `bg_color` - Color for out-of-bounds samples
    ///
    /// # Returns
    ///
    /// Interpolated RGBA color array
    pub fn sample_bilinear(&self, x: f32, y: f32, bg_color: [u8; 4]) -> [u8; 4] {
        // Check if we're completely outside the texture
        if x < 0.0 || x >= self.width as f32 || y < 0.0 || y >= self.height as f32 {
            return bg_color;
        }

        // Get integer and fractional parts
        let ix = x.floor();
        let iy = y.floor();
        let fx = x - ix;
        let fy = y - iy;

        // Get the four nearest texel coordinates
        let x1 = ix as u32;
        let y1 = iy as u32;
        let x2 = (x1 + 1).min(self.width - 1);
        let y2 = (y1 + 1).min(self.height - 1);

        // Sample the four corners
        let c00 = self.sample(x1 as f32, y1 as f32, bg_color);
        let c10 = self.sample(x2 as f32, y1 as f32, bg_color);
        let c01 = self.sample(x1 as f32, y2 as f32, bg_color);
        let c11 = self.sample(x2 as f32, y2 as f32, bg_color);

        // Perform bilinear interpolation for each color channel
        let mut result = [0; 4];
        for i in 0..4 {
            let top = c00[i] as f32 * (1.0 - fx) + c10[i] as f32 * fx;
            let bottom = c01[i] as f32 * (1.0 - fx) + c11[i] as f32 * fx;
            result[i] = (top * (1.0 - fy) + bottom * fy) as u8;
        }

        result
    }
}
