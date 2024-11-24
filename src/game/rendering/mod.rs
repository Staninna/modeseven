//! Game world rendering and graphics systems
//!
//! Implements an SNES Mode 7-style renderer with perspective-correct
//! texture mapping. Handles all visual aspects.

mod renderable;
mod renderer;

pub use renderable::Renderable;
pub use renderer::Renderer;
