// allow to have for example world.rs in module `world` `/src/world/world.rs`
#![allow(clippy::module_inception)]
// #![forbid(missing_docs)] // TODO: Enable this when we have documentation

//! modeseven
//!
//! A 2D racing game inspired by Super Nintendo's Mode 7.
//!
//! The game is a two-player split-screen racing game with a split-screen
//! view of the 2 players' cars. The game is rendered in split-screen mode
//! with a top view and a bottom view. The top view is player 1's view, the
//! bottom view is player 2's view.

// TODO: Remove magic numbers etc by wrapping in types ThingId(usize)
pub mod app;
pub mod assets;
pub mod consts;
pub mod game;
pub mod menu;
mod state;

use anyhow::Result;
use log::LevelFilter;
use pix_win_loop::{PhysicalSize, WindowBuilder};
use std::time::Duration;

use crate::{
    app::Application,
    consts::{FPS, MAX_LAG_TIME, PIXELS_HEIGHT, PIXELS_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH},
};

fn main() -> Result<()> {
    // Init logging
    env_logger::builder().filter_level(LevelFilter::Info).init();

    // Create window
    let window_builder = WindowBuilder::new()
        .with_title("modeseven")
        .with_inner_size(PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));

    // Get pixel buffer size
    let pixel_buffer_size = PhysicalSize::new(PIXELS_WIDTH, PIXELS_HEIGHT);

    // Set target frame times
    let target_frame_time = Duration::from_secs_f32(1. / FPS);
    let max_frame_time = Duration::from_secs_f32(MAX_LAG_TIME);

    // Start game loop
    pix_win_loop::start(
        window_builder,
        Application::new()?,
        pixel_buffer_size,
        target_frame_time,
        max_frame_time,
    )
}
