use std::time::Duration;
use pix_win_loop::{PhysicalSize, WindowBuilder};
use anyhow::Result;
use log::LevelFilter;

mod app;
mod camera;
mod rendering;
mod input;
mod utils;
mod consts;
mod physics;

use crate::consts::{FPS, MAX_LAG_TIME, PIXELS_HEIGHT, PIXELS_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::app::Application;

fn main() -> Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let window_builder = WindowBuilder::new()
        .with_title("modeseven")
        .with_inner_size(PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));

    let pixel_buffer_size = PhysicalSize::new(PIXELS_WIDTH, PIXELS_HEIGHT);

    let target_frame_time = Duration::from_secs_f32(1. / FPS);
    let max_frame_time = Duration::from_secs_f32(MAX_LAG_TIME);

    pix_win_loop::start(
        window_builder,
        Application::new()?,
        pixel_buffer_size,
        target_frame_time,
        max_frame_time,
    )
}