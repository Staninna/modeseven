//! Game world rendering and graphics systems
//!
//! Implements an SNES Mode 7-style renderer with perspective-correct
//! texture mapping. Handles all visual aspects including texture
//! management, and visual effects.

mod renderer;
mod texture;

pub use renderer::Renderer;
pub use texture::Texture;
