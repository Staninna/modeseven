use anyhow::Result;
use image::GenericImageView;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

impl Texture {
    pub fn from_image<P: AsRef<Path>>(path: P) -> Result<Self> {
        let image = image::open(path)?;
        let (width, height) = image.dimensions();
        let bytes_per_pixel = image.color().bytes_per_pixel();

        let pixels: Vec<u8> = match bytes_per_pixel {
            3 => {
                // rgb to rgba conversion
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
                // already rgba
                image.as_bytes().to_vec()
            }
            _ => anyhow::bail!("Unsupported image format with {} bytes per pixel", bytes_per_pixel),
        };

        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    pub fn sample(&self, x: f32, y: f32, bg_color: [u8; 4]) -> [u8; 4] {
        // Check if coordinates are within bounds
        if x < 0.0 || x >= self.width as f32 || y < 0.0 || y >= self.height as f32 {
            return bg_color;
        }

        // Convert to integers
        let x = x as u32;
        let y = y as u32;

        // Get pixel index
        let idx = ((y * self.width + x) * 4) as usize;

        [
            self.pixels[idx],
            self.pixels[idx + 1],
            self.pixels[idx + 2],
            self.pixels[idx + 3],
        ]
    }

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

        // Bilinear interpolation
        let mut result = [0; 4];
        for i in 0..4 {
            let top = c00[i] as f32 * (1.0 - fx) + c10[i] as f32 * fx;
            let bottom = c01[i] as f32 * (1.0 - fx) + c11[i] as f32 * fx;
            result[i] = (top * (1.0 - fy) + bottom * fy) as u8;
        }

        result
    }
}