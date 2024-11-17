use anyhow::Result;
use log::info;
use pix_win_loop::{App, Context, Pixels};
use std::time::Instant;

use crate::{camera::Camera,
            consts::{PIXELS_HEIGHT, PIXELS_WIDTH},
            input::Controls,
            physics::Car,
            rendering::{Renderer, Texture},
            utils::FpsCounter};

pub struct Application {
    renderer: Renderer,
    camera1: Camera,
    camera2: Camera,
    controls: Controls,
    fps_counter: FpsCounter,
    car: Car,
    last_update: Instant,
}

impl Application {
    pub fn new() -> Result<Self> {
        let ground_texture = Texture::from_image("assets/track.png")?;
        let renderer = Renderer::new(PIXELS_WIDTH, PIXELS_HEIGHT / 2, ground_texture);

        // Main camera: car
        let camera1 = Camera::new(0.0, 0.0, 10.0, 0.0);
        
        // Second camera (for nothing for now) later player 2
        let camera2 = Camera::new(1024.0 / 2.0, 1024.0 / 2.0, 500.0, 0.0);

        Ok(Self {
            renderer,
            camera1,
            camera2,
            controls: Controls::new(),
            fps_counter: FpsCounter::new(1.0),
            car: Car::new(1024.0 / 3.0, 1024.0 / 3.0),
            last_update: Instant::now(),
        })
    }
}

impl App for Application {
    fn update(&mut self, ctx: &mut Context) -> Result<()> {
        // Update controls
        self.controls.update(ctx);

        // Calculate delta time
        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        // Get input values
        let input = self.controls.get_car_input();

        // Update car physics
        self.car.update(dt, input.throttle, input.brake, input.turn);

        // Update main camera to follow car
        self.camera1.follow_car(&self.car, dt);

        if let Some(fps) = self.fps_counter.update() {
            info!("FPS: {:.2}", fps);
        }

        Ok(())
    }

    fn render(&mut self, pixels: &mut Pixels, _blending_factor: f64) -> Result<()> {
        let frame = pixels.frame_mut();
        let half_height = PIXELS_HEIGHT / 2;
        let row_size = PIXELS_WIDTH * 4;
        let view_size = (PIXELS_WIDTH * half_height * 4) as usize;

        // First camera (following car)
        let top_view = &mut frame[0..view_size];
        self.renderer.render(top_view, &self.camera1);

        // Second camera (for nothing)
        let bottom_view = &mut frame[view_size..];
        self.renderer.render(bottom_view, &self.camera2);

        // Render separator line
        let separator_row = view_size - row_size as usize;
        for x in 0..PIXELS_WIDTH as usize {
            let pixel_idx = separator_row + x * 4;
            frame[pixel_idx..pixel_idx + 4].copy_from_slice(&[255, 0, 0, 255]);
        }

        pixels.render()?;
        Ok(())
    }
}