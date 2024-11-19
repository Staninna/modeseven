use anyhow::Result;
use image::GenericImageView;
use std::path::Path;

/// A 2D texture that supports sampling and bilinear interpolation
///
/// The Texture struct represents a 2D image in RGBA format that can be sampled
/// for rendering. It supports both nearest-neighbor and bilinear interpolation
/// sampling methods, and can be created either from an image file or generated
/// as a checkerboard pattern for testing.
///
/// All texture data is stored in RGBA format (4 bytes per pixel) regardless of
/// the source image format. Non-RGBA images are automatically converted during loading.
///
/// # Example
///
/// ```rust
/// // Load a texture from a file
/// let texture = Texture::from_image("ground.png")?;
///
/// // Sample a pixel using bilinear interpolation
/// let color = texture.sample_bilinear(10.5, 20.7, [0, 0, 0, 255]);
///
/// // Create a test checkerboard texture
/// let checker = Texture::checkerboard(256, 256, 32);
/// ```
#[derive(Debug, Clone)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    /// Raw pixel data in RGBA format (4 bytes per pixel)
    pub pixels: Vec<u8>,
    /// Path or identifier of the texture source
    pub path: String,
}

impl Texture {
    /// Creates a new checkerboard texture for testing and debugging purposes
    ///
    /// Generates a black and white checkerboard pattern with configurable dimensions
    /// and checker size. The resulting texture is always in RGBA format with full
    /// opacity (alpha = 255).
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the texture in pixels
    /// * `height` - The height of the texture in pixels
    /// * `checker_size` - The size of each checker square in pixels
    ///
    /// # Returns
    ///
    /// A new `Texture` instance containing the checkerboard pattern
    ///
    /// # Example
    ///
    /// ```rust
    /// // Create a 256x256 texture with 32x32 checker squares
    /// let checker = Texture::checkerboard(256, 256, 32);
    /// ```
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

    /// Creates a new texture by loading and processing an image file
    ///
    /// Loads an image from the specified path and converts it to RGBA format if necessary.
    /// Supports both RGB and RGBA source images. Other formats will return an error.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the image file to load. Must implement both `AsRef<Path>` and ToString
    ///
    /// # Returns
    ///
    /// * `Ok(Texture)` - A new texture containing the loaded image data
    /// * `Err(Error)` - If the image cannot be loaded or has an unsupported format
    ///
    /// # Example
    ///
    /// ```rust
    /// match Texture::from_image("assets/ground.png") {
    ///     Ok(texture) => println!("Loaded {}x{} texture", texture.width, texture.height),
    ///     Err(e) => eprintln!("Failed to load texture: {}", e),
    /// }
    /// ```
    pub fn from_image<P: AsRef<Path> + ToString>(path: P) -> Result<Self> {
        let image = image::open(&path)?;
        let (width, height) = image.dimensions();
        let bytes_per_pixel = image.color().bytes_per_pixel();

        let pixels: Vec<u8> = match bytes_per_pixel {
            3 => {
                // Convert RGB to RGBA by adding full opacity
                let pixels_rgb = image.as_bytes().to_vec();
                pixels_rgb
                    .chunks_exact(3)
                    .flat_map(|chunk| {
                        let r = chunk[0];
                        let g = chunk[1];
                        let b = chunk[2];
                        vec![r, g, b, 255]
                    })
                    .collect()
            }
            4 => {
                // Already in RGBA format
                image.as_bytes().to_vec()
            }
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

    /// Samples a single pixel from the texture using nearest-neighbor interpolation
    ///
    /// Converts the floating-point coordinates to integer pixel coordinates and returns
    /// the color at that position. If the coordinates are outside the texture bounds,
    /// returns the specified background color.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate in texture space (0.0 to width)
    /// * `y` - Y coordinate in texture space (0.0 to height)
    /// * `bg_color` - RGBA color to return for out-of-bounds coordinates
    ///
    /// # Returns
    ///
    /// An array of 4 bytes containing the RGBA color values
    ///
    /// # Example
    ///
    /// ```rust
    /// let color = texture.sample(15.0, 30.0, [0, 0, 0, 255]);
    /// ```
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

    /// Samples a pixel from the texture using bilinear interpolation
    ///
    /// Performs smooth interpolation between neighboring pixels to provide
    /// higher quality sampling for texture mapping. Uses four adjacent pixels
    /// to calculate a weighted average based on the fractional coordinates.
    ///
    /// The interpolation process:
    /// 1. Finds the four nearest pixels to the sample point
    /// 2. Calculates the fractional distance to each pixel
    /// 3. Performs linear interpolation in both x and y directions
    /// 4. Returns the weighted average color
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate in texture space (0.0 to width)
    /// * `y` - Y coordinate in texture space (0.0 to height)
    /// * `bg_color` - RGBA color to return for out-of-bounds coordinates
    ///
    /// # Returns
    ///
    /// An array of 4 bytes containing the interpolated RGBA color values
    ///
    /// # Example
    ///
    /// ```rust
    /// let color = texture.sample_bilinear(15.7, 30.2, [0, 0, 0, 255]);
    /// ```
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
