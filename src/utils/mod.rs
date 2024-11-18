//! Utility functions and helper types
//!
//! Collection of general-purpose utilities including vector math,
//! FPS counting, and other helper functions used throughout the
//! game. Provides common functionality shared across modules.

mod fps;
mod vec2;

pub use fps::FpsCounter;
pub use vec2::Vec2;