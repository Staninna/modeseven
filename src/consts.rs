//! Global constants and configuration values
//!
//! Contains all game-wide constant definitions including screen dimensions,
//! physics parameters, and other configurable values.

/// Width of the application window in pixels.
pub const WINDOW_WIDTH: u32 = 800;

/// Height of the application window in pixels.
pub const WINDOW_HEIGHT: u32 = 1000;

/// Scaling factor that determines the ratio between logical and physical pixels.
/// A value of 1.0 means one logical pixel equals one physical pixel.
pub const PIXELS_PER_PIXEL: f32 = 1.0;

/// Width of the actual frame buffer in pixels.
/// This determines the number of pixels available for rendering in the horizontal direction,
/// independent of the window's physical display size.
pub const PIXELS_WIDTH: u32 = (WINDOW_WIDTH as f32 / PIXELS_PER_PIXEL) as u32;

/// Height of the actual frame buffer in pixels.
/// This determines the number of pixels available for rendering in the vertical direction,
/// independent of the window's physical display size.
pub const PIXELS_HEIGHT: u32 = (WINDOW_HEIGHT as f32 / PIXELS_PER_PIXEL) as u32;

/// Target frames per second for the application.
/// Controls how frequently the game loop updates and renders.
pub const FPS: f32 = 144.0;

/// Maximum allowed time delta between frames in seconds.
/// Prevents spiral of death in case of major lag spikes by capping
/// the time step used for physics/game logic updates.
pub const MAX_LAG_TIME: f32 = 0.1;

// Include generated constants
include!(concat!(env!("OUT_DIR"), "/filename_consts.rs"));
