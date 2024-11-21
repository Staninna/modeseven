//! Asset management and loading
//!
//! This module provides a centralized asset management system for the game.
//! It compiles all assets within the binary and loads them into memory at startup.

mod manager;
pub use manager::AssetManager;

pub mod texture;
pub use texture::Texture;
