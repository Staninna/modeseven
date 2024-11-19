//! Main application and game loop management
//!
//! This module contains the core Application struct that manages the game's
//! lifecycle, including initialization, update loop, and rendering.

use crate::{
    camera::Camera,
    consts::{PIXELS_HEIGHT, PIXELS_WIDTH},
    input::Inputs,
    rendering::{Renderer, Texture},
    utils::FpsCounter,
    world::World,
};
use anyhow::Result;
use log::info;
use pix_win_loop::{App, Context, Pixels};
use std::time::Instant;

/// Main game application managing state, rendering, and game loop
///
/// The Application struct serves as the central coordinator for the game,
/// implementing a split-screen two-player racing game. It manages:
/// * Game state and world simulation
/// * Dual camera views for split-screen rendering
/// * Input handling for both players
/// * Performance monitoring and frame timing
/// * Asset loading and resource management
///
/// The game renders in split-screen mode with:
/// * Player 1's view in the top half
/// * Player 2's view in the bottom half
/// * A separator line between views
///
/// # Example
///
/// ```rust
/// match Application::new() {
///     Ok(mut app) => {
///         // Start game loop...
///     }
///     Err(e) => eprintln!("Failed to initialize game: {}", e),
/// }
/// ```
pub struct Application {
    /// Renderer instance for drawing the game world
    renderer: Renderer,
    /// Game world containing all game entities
    world: World,
    /// Camera for player 1's view (top screen)
    camera1: Camera,
    /// Camera for player 2's view (bottom screen)
    camera2: Camera,
    /// Input handler for both players
    controls: Inputs,
    /// FPS counter for performance monitoring
    fps_counter: FpsCounter,
    /// Timestamp of last update for delta time calculation
    last_update: Instant,
}

impl Application {
    /// Creates and initializes a new game application
    ///
    /// This method performs the complete initialization sequence:
    /// 1. Loads the ground texture from disk
    /// 2. Creates the renderer with split-screen dimensions
    /// 3. Initializes the game world and entities
    /// 4. Sets up cameras, controls, and timing systems
    ///
    /// # Returns
    ///
    /// * `Ok(Application)` - A fully initialized application ready to run
    /// * `Err(Error)` - If any initialization step fails (e.g., missing assets)
    ///
    /// # Errors
    ///
    /// Will return an error if:
    /// * The ground texture file cannot be loaded
    /// * The renderer fails to initialize
    ///
    /// # Example
    ///
    /// ```rust
    /// match Application::new() {
    ///     Ok(app) => println!("Game initialized successfully"),
    ///     Err(e) => eprintln!("Failed to load game: {}", e),
    /// }
    /// ```
    pub fn new() -> Result<Self> {
        let ground_texture = Texture::from_image("assets/track.png")?;
        let renderer = Renderer::new(PIXELS_WIDTH, PIXELS_HEIGHT / 2, ground_texture);

        Ok(Self {
            world: World::new(),
            renderer,
            camera1: Camera::default(),
            camera2: Camera::default(),
            controls: Inputs::new(),
            fps_counter: FpsCounter::new(1.0),
            last_update: Instant::now(),
        })
    }
}

impl App for Application {
    /// Updates the game state for one frame
    ///
    /// This method performs the complete frame update sequence:
    /// 1. Processes player inputs
    /// 2. Calculates frame timing
    /// 3. Updates world physics and entities
    /// 4. Updates camera positions
    /// 5. Updates performance metrics
    ///
    /// The update is frame-rate independent through delta time,
    /// ensuring consistent game speed regardless of performance.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Current game context containing input state
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Update completed successfully
    /// * `Err(Error)` - If any update step fails
    fn update(&mut self, ctx: &mut Context) -> Result<()> {
        // Process player inputs
        let inputs = self.controls.update(ctx);

        // Calculate frame timing
        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        // Update game world and physics
        self.world.update(inputs, dt);

        // Update camera positions to follow cars
        self.camera1.follow_car(&self.world.cars[0], dt);
        self.camera2.follow_car(&self.world.cars[1], dt);

        // Update and log performance metrics
        if let Some(fps) = self.fps_counter.update() {
            info!("FPS: {:.2}", fps);
        }

        Ok(())
    }

    /// Renders the game scene in split-screen mode
    ///
    /// This method renders the complete game scene, including:
    /// 1. Top half - Player 1's view from camera1
    /// 2. Red separator line between views
    /// 3. Bottom half - Player 2's view from camera2
    ///
    /// The rendering process:
    /// 1. Splits the pixel buffer into top/bottom views
    /// 2. Renders each camera view independently
    /// 3. Draws the separator line
    /// 4. Sends the final buffer to the display
    ///
    /// # Arguments
    ///
    /// * `pixels` - Pixel buffer for drawing
    /// * `_blending_factor` - Unused interpolation factor
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Render completed successfully
    /// * `Err(Error)` - If any rendering step fails
    fn render(&mut self, pixels: &mut Pixels, _blending_factor: f64) -> Result<()> {
        let frame = pixels.frame_mut();
        let half_height = PIXELS_HEIGHT / 2;
        let row_size = PIXELS_WIDTH * 4;
        let view_size = (PIXELS_WIDTH * half_height * 4) as usize;

        // Render player 1's view (top half)
        let top_view = &mut frame[0..view_size];
        self.renderer.render(top_view, &self.camera1);

        // Render player 2's view (bottom half)
        let bottom_view = &mut frame[view_size..];
        self.renderer.render(bottom_view, &self.camera2);

        // Draw red separator line between views
        let separator_row = view_size - row_size as usize;
        for x in 0..PIXELS_WIDTH as usize {
            let pixel_idx = separator_row + x * 4;
            frame[pixel_idx..pixel_idx + 4].copy_from_slice(&[255, 0, 0, 255]);
        }

        // Update display
        pixels.render()?;

        Ok(())
    }
}
