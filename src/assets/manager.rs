use crate::assets::Texture;
use include_assets::{include_dir, NamedArchive};
use std::collections::HashMap;

/// Asset management system with compile-time loading and constant-time lookups.
///
/// # Implementation
/// - Assets are embedded in binary at compile time from `assets` directory
/// - File names are checked at compile time via build.rs constants
/// - All operations using generated constants are guaranteed safe
pub struct AssetManager {
    assets: NamedArchive,
    textures: HashMap<String, Texture>,
}

impl AssetManager {
    /// Creates a new empty AssetManager instance.
    ///
    /// Initializes the internal archive with assets embedded at compile time.
    pub fn new() -> Self {
        Self {
            assets: NamedArchive::load(include_dir!("assets")),
            textures: HashMap::new(),
        }
    }

    /// Gets a cached texture by name.
    ///
    /// # Arguments
    /// * `name` - Asset name matching a compile-time generated constant
    ///
    /// # Returns
    /// - Some(Texture) if the asset is loaded
    /// - None if the asset is not loaded
    pub fn get(&self, name: &str) -> Option<&Texture> {
        self.textures.get(name)
    }

    /// Loads or retrieves a cached texture.
    ///
    /// # Arguments
    /// * `name` - Asset name matching a compile-time generated constant
    ///
    /// # Returns
    /// - Some(Texture) if the asset is loaded
    /// - None if the asset is not loaded
    ///
    /// # Panics (should never happen)
    /// - If the asset is not found in the archive
    pub fn load(&mut self, name: &str) -> &Texture {
        if self.textures.contains_key(name) {
            return self.textures.get(name).unwrap();
        }

        let image = image::load_from_memory(self.assets.get(name).expect("Texture not found"))
            .expect("Failed to load texture");
        let texture = Texture::from_image(image);

        self.textures.insert(name.to_string(), texture);
        self.textures.get(name).unwrap()
    }
}
