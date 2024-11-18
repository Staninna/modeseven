#![allow(clippy::module_inception)]

// TODO: Remove magic numbers etc by wrapping in types ThingId(usize)

mod app;
mod camera;
mod rendering;
mod input;
mod utils;
mod consts;
mod world;

use anyhow::Result;
use log::LevelFilter;
use pix_win_loop::{PhysicalSize, WindowBuilder};
use std::time::Duration;

use crate::{app::Application,
            consts::{FPS, MAX_LAG_TIME, PIXELS_HEIGHT, PIXELS_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH}};

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