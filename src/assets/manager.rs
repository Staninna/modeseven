use crate::assets::Texture;
use crate::consts::ALL_ASSET_FILES;
use include_assets::{include_dir, NamedArchive};
use std::collections::HashMap;

/// Asset management system with compile-time loading and constant-time lookups.
///
/// # Implementation
/// - Assets are embedded in binary at compile time from `assets` directory
/// - File names are checked at compile time via build.rs constants
/// - All operations using generated constants are guaranteed safe
pub struct AssetManager {
    textures: HashMap<String, Texture>,
}

impl AssetManager {
    /// Creates a new empty AssetManager instance.
    ///
    /// Initializes the internal archive with assets embedded at compile time.
    pub fn new() -> Self {
        let assets = NamedArchive::load(include_dir!("assets"));
        // loop over all assets of
        let mut textures: HashMap<String, Texture> = HashMap::new();
        for asset in ALL_ASSET_FILES {
            let texture = Texture::from_image(
                image::load_from_memory(assets.get(asset).expect("Texture not found"))
                    .expect("Failed to load texture"),
            );

            textures.insert(asset.to_string(), texture);
        }

        Self { textures }
    }

    /// Gets a cached texture by name.
    ///
    /// # Arguments
    /// * `name` - Asset name matching a compile-time generated constant
    ///
    /// # Returns
    /// - Some(Texture) if the asset is loaded
    /// - None if the asset is not loaded
    pub fn get(&self, name: &str) -> &Texture {
        self.textures.get(name).expect("Texture not found")
    }
}
