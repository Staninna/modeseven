use crate::assets::Texture;
use crate::consts::{ALL_ASSET_FILES, HALO_DEK_FONT_FILE};
use include_assets::{include_dir, NamedArchive};
use rusttype::Font;
use std::collections::HashMap;

/// Asset management system with compile-time loading and constant-time lookups.
///
/// # Implementation
/// - Assets are embedded in binary at compile time from `assets` directory
/// - File names are checked at compile time via build.rs constants
/// - All operations using generated constants are guaranteed safe
pub struct AssetManager {
    textures: HashMap<String, Texture>,
    font: Font<'static>,
}

impl AssetManager {
    /// Creates a new empty AssetManager instance.
    ///
    /// Initializes the internal archive with assets embedded at compile time.
    pub fn new() -> Self {
        let assets = NamedArchive::load(include_dir!("assets"));

        let mut textures: HashMap<String, Texture> = HashMap::new();
        for asset in ALL_ASSET_FILES {
            if asset.ends_with(".ttf") {
                continue;
            }

            let texture = Texture::from_image(
                image::load_from_memory(assets.get(asset).expect("Texture not found"))
                    .expect("Failed to load texture"),
            );

            textures.insert(asset.to_string(), texture);
        }

        // Convert the font data to a static slice (black magic)
        let font_data = assets.get(HALO_DEK_FONT_FILE).expect("Font not found");
        let font_data_static: &'static [u8] = Box::leak(font_data.to_vec().into_boxed_slice());

        let font =
            Font::try_from_bytes(font_data_static).expect("error constructing a Font from bytes");

        Self { textures, font }
    }

    /// Gets a cached texture by name.
    ///
    /// # Arguments
    /// * `name` - Asset name matching a compile-time generated constant
    ///
    /// # Returns
    /// - Some(Texture) if the asset is loaded
    /// - None if the asset is not loaded
    pub fn get_texture(&self, name: &str) -> &Texture {
        self.textures.get(name).expect("Texture not found")
    }

    /// Gets a cached font by name.
    ///
    /// # Arguments
    /// * `name` - Asset name matching a compile-time generated constant
    ///
    /// # Returns
    /// - Some(Font) if the asset is loaded
    /// - None if the asset is not loaded
    pub fn get_font(&self) -> &Font {
        &self.font
    }
}
