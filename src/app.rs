//! TODO: Update docs they are currently wrong
//! Main application and game loop management
//!
//! This module contains the core Application struct that manages the game's
//! lifecycle, including initialization, update loop, and rendering.

use crate::assets::AssetManager;
use crate::consts::{PIXELS_HEIGHT, PIXELS_WIDTH, TRACK_FILE};
#[cfg(debug_assertions)]
use crate::game::utils::FpsCounter;
use crate::game::{
    camera::Camera,
    input::Inputs, /* TODO: Move from this piece of shit to the handle() func */
    rendering::Renderer, world::World,
};
use crate::menu::MenuRenderer;
use crate::menu::MenuState;
use crate::state::GameState;
use anyhow::Result;
use pix_win_loop::winit::event::{Event, WindowEvent};
use pix_win_loop::{App, Context, KeyCode, Pixels};
use std::time::Instant;

/// TODO: Update docs they are currently wrong
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
pub struct Application {
    // Game state stuff
    /// Renderer instance for drawing the game world
    renderer: Renderer,
    /// Game world containing all game entities
    world: World,
    /// Camera for player 1's view (top screen)
    camera_player_one: Camera,
    /// Camera for player 2's view (bottom screen)
    camera_player_two: Camera,
    /// Input handler for both players
    controls: Inputs,

    // Menu stuff
    /// Menu renderer
    menu_renderer: MenuRenderer,

    // Global state and stuff
    /// Menu/game state
    state: GameState,
    /// Asset manager for loading assets
    asset_manager: AssetManager,
    #[cfg(debug_assertions)]
    /// FPS counter for performance monitoring
    fps_counter: FpsCounter,
    /// Timestamp of last update for delta time calculation
    last_update: Instant,
}

impl Application {
    /// TODO: Update docs they are currently wrong
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
    pub fn new() -> Result<Self> {
        let asset_manager = AssetManager::new();
        let ground_texture = asset_manager.get(TRACK_FILE);
        let renderer = Renderer::new(PIXELS_WIDTH, PIXELS_HEIGHT / 2, ground_texture.clone());

        Ok(Self {
            state: GameState::Menu(MenuState::Main),
            world: World::new(),
            renderer,
            asset_manager,
            camera_player_one: Camera::default(),
            camera_player_two: Camera::default(),
            controls: Inputs::new(),
            #[cfg(debug_assertions)]
            fps_counter: FpsCounter::new(1.0),
            last_update: Instant::now(),
            menu_renderer: MenuRenderer::new(),
        })
    }
}

impl App for Application {
    /// TODO: Update docs they are currently wrong
    /// Updates the game state for one frame
    ///
    /// This method performs the complete frame update sequence:
    /// 1. Processes player inputs
    /// 2. Calculates frame timing
    /// 3. Updates world physics and entities
    /// 4. Updates camera positions
    ///
    /// # Arguments
    ///
    /// * `ctx` - Current game context containing update state
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Update completed successfully
    /// * `Err(Error)` - If any update step fails (doesn't happen normally)
    ///
    /// Updated menu flow:
    /// [![](https://mermaid.ink/img/pako:eNqVVEtu2zAQvcqAQXYyihZdEUU2ctGVCjnsqlYWtDSShUikwY-BIMk1cpAui54mJylJfeqITpDKG3Lmcea9mQffk1JWSChpFD_s4ce6EDB92u6GaEEy3grIUFj4zo9tw00rRUFOsP7Lsm2ArMDDv-zU1fPTb8g7fgffeI8-wNCYVjTan1OFVWv0gPv14fnpD4VMHhFKq7RUPv5VGFQUGHZYmpuoHaxWVw8FGV8X5MHFTkAoqkKc15NKhYEUMMMN6khLvvW8HVVPIwCVFWK8-xQqKKUwSnYaeGnaIy755Wybc6uxCkp0yQ9I4Rq1HUaxobCxbaQqH0UND7ymnEWFY8z7ZDO7C1uM9bJ5d9OKlsTSGTEu7uaNnpeX8M8yRoI0ezcwPcxaxCsMiwYpgluWgmLUxNEjWfYmdCTrkWm25HiNxiqhPcOZ7gliHvQm8hbLoh28yKfn8yeIsuNar7GG3nUOc6rbrqMXn3b-l2hnrVukF3VdJ6XspArHc-8b587g4rHAR_xc_VcBZ5AX_X2BV58vC_iRTwqiXJ64Ec78ojTLEjensT1JSI_Klarcv9G9xxbEuaZ3s6PuWHF161376HDcGsnuREmoURYToqRt9oTWvNPuZg-Va7ZuuXN9P0cPXPyUcro__gUm3n0i?type=png)](https://mermaid.live/edit#pako:eNqVVEtu2zAQvcqAQXYyihZdEUU2ctGVCjnsqlYWtDSShUikwY-BIMk1cpAui54mJylJfeqITpDKG3Lmcea9mQffk1JWSChpFD_s4ce6EDB92u6GaEEy3grIUFj4zo9tw00rRUFOsP7Lsm2ArMDDv-zU1fPTb8g7fgffeI8-wNCYVjTan1OFVWv0gPv14fnpD4VMHhFKq7RUPv5VGFQUGHZYmpuoHaxWVw8FGV8X5MHFTkAoqkKc15NKhYEUMMMN6khLvvW8HVVPIwCVFWK8-xQqKKUwSnYaeGnaIy755Wybc6uxCkp0yQ9I4Rq1HUaxobCxbaQqH0UND7ymnEWFY8z7ZDO7C1uM9bJ5d9OKlsTSGTEu7uaNnpeX8M8yRoI0ezcwPcxaxCsMiwYpgluWgmLUxNEjWfYmdCTrkWm25HiNxiqhPcOZ7gliHvQm8hbLoh28yKfn8yeIsuNar7GG3nUOc6rbrqMXn3b-l2hnrVukF3VdJ6XspArHc-8b587g4rHAR_xc_VcBZ5AX_X2BV58vC_iRTwqiXJ64Ec78ojTLEjensT1JSI_Klarcv9G9xxbEuaZ3s6PuWHF161376HDcGsnuREmoURYToqRt9oTWvNPuZg-Va7ZuuXN9P0cPXPyUcro__gUm3n0i)
    fn update(&mut self, ctx: &mut Context) -> Result<()> {
        // Calculate dt but only update last_update timestamp when playing
        let now = Instant::now();
        let dt = if matches!(self.state, GameState::Playing) {
            let dt = now.duration_since(self.last_update).as_secs_f32();
            self.last_update = now;
            dt
        } else {
            0.0 // No time passes while paused or in menus
        };

        match self.state {
            GameState::Menu(menu_state) => {
                // Handle menu input
                if ctx.input.is_physical_key_pressed(KeyCode::ArrowUp) {
                    self.menu_renderer.move_selection(-1, 3); // 3 menu items
                    log::info!(
                        "Menu selection moved up to item {}",
                        self.menu_renderer.selected_item()
                    );
                }
                if ctx.input.is_physical_key_pressed(KeyCode::ArrowDown) {
                    self.menu_renderer.move_selection(1, 3);
                    log::info!(
                        "Menu selection moved down to item {}",
                        self.menu_renderer.selected_item()
                    );
                }
                if ctx.input.is_physical_key_pressed(KeyCode::Enter) {
                    match (menu_state, self.menu_renderer.selected_item()) {
                        (MenuState::Main, 0) => {
                            log::info!("State change: Main Menu -> Playing");
                            self.state = GameState::Playing;
                            self.last_update = now;
                            // TODO: Reset menu selection and game state
                        }
                        (MenuState::Main, 1) => {
                            log::info!("State change: Main Menu -> Settings Menu");
                            self.state = GameState::Menu(MenuState::Settings);
                        }
                        (MenuState::Main, 2) => {
                            log::info!("State change: Main Menu -> Credits Menu");
                            self.state = GameState::Menu(MenuState::Credits);
                        }
                        _ => {}
                    }
                }
                if ctx.input.is_physical_key_pressed(KeyCode::Escape)
                    && menu_state != MenuState::Main
                {
                    log::info!("State change: {} Menu -> Main Menu", menu_state);
                    self.state = GameState::Menu(MenuState::Main);
                }
            }
            GameState::Playing => {
                self.controls.update(ctx);
                self.world.update(&self.controls, dt);
                self.camera_player_one.follow_car(&self.world.cars[0], dt);
                self.camera_player_two.follow_car(&self.world.cars[1], dt);

                if ctx.input.is_physical_key_pressed(KeyCode::Escape) {
                    log::info!("State change: Playing -> Paused");
                    self.state = GameState::Paused;
                }
            }
            GameState::Paused => {
                if ctx.input.is_physical_key_pressed(KeyCode::Escape) {
                    log::info!("State change: Paused -> Playing");
                    self.state = GameState::Playing;
                    self.last_update = now;
                }
                if ctx.input.is_physical_key_pressed(KeyCode::KeyQ) {
                    log::info!("State change: Paused -> Main Menu");
                    self.state = GameState::Menu(MenuState::Main);
                }
            }
        }

        Ok(())
    }

    /// TODO: Update docs they are currently wrong
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
    /// * `_blending_factor` - Unused parameter don't know what it does/is
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Render completed successfully
    /// * `Err(Error)` - If any rendering step fails
    fn render(&mut self, pixels: &mut Pixels, _blending_factor: f64) -> Result<()> {
        let frame = pixels.frame_mut();

        match self.state {
            GameState::Playing | GameState::Paused => {
                let half_height = PIXELS_HEIGHT / 2;
                let row_size = PIXELS_WIDTH * 4;
                let view_size = (PIXELS_WIDTH * half_height * 4) as usize;

                // Render player 1's view (top half)
                let top_view = &mut frame[0..view_size];
                self.renderer.render(
                    top_view,
                    &self.world,
                    &self.camera_player_one,
                    &self.asset_manager,
                );

                // Render player 2's view (bottom half)
                let bottom_view = &mut frame[view_size..];
                self.renderer.render(
                    bottom_view,
                    &self.world,
                    &self.camera_player_two,
                    &self.asset_manager,
                );

                // Draw red separator line between views
                let separator_row = view_size - row_size as usize;
                for x in 0..PIXELS_WIDTH as usize {
                    let pixel_idx = separator_row + x * 4;
                    frame[pixel_idx..pixel_idx + 4].copy_from_slice(&[255, 0, 0, 255]);
                }

                if self.state == GameState::Paused {
                    // TODO: Draw text?? paused
                    // use menu renderer without clearing background so u can overlay menus/ui is hacky but would work
                }
            }
            GameState::Menu(menu_state) => {
                self.menu_renderer
                    .render(frame, menu_state, &self.asset_manager)?
            }
        }

        // Update display
        pixels.render()?;

        Ok(())
    }

    /// Handles events from the window system
    ///
    /// # Arguments
    ///
    /// * `event` - Event to handle
    fn handle(&mut self, event: &Event<()>) -> Result<()> {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                // WindowEvent::ActivationTokenDone { .. } => {}
                // WindowEvent::Resized(_) => {}
                // WindowEvent::Moved(_) => {}
                // WindowEvent::CloseRequested => {}
                // WindowEvent::Destroyed => {}
                // WindowEvent::DroppedFile(_) => {}
                // WindowEvent::HoveredFile(_) => {}
                // WindowEvent::HoveredFileCancelled => {}
                // WindowEvent::Focused(_) => {}
                // WindowEvent::KeyboardInput { .. } => {}
                // WindowEvent::ModifiersChanged(_) => {}
                // WindowEvent::Ime(_) => {}
                // WindowEvent::CursorMoved { .. } => {}
                // WindowEvent::CursorEntered { .. } => {}
                // WindowEvent::CursorLeft { .. } => {}
                // WindowEvent::MouseWheel { .. } => {}
                // WindowEvent::MouseInput { .. } => {}
                // WindowEvent::TouchpadMagnify { .. } => {}
                // WindowEvent::SmartMagnify { .. } => {}
                // WindowEvent::TouchpadRotate { .. } => {}
                // WindowEvent::TouchpadPressure { .. } => {}
                // WindowEvent::AxisMotion { .. } => {}
                // WindowEvent::Touch(_) => {}
                // WindowEvent::ScaleFactorChanged { .. } => {}
                // WindowEvent::ThemeChanged(_) => {}
                // WindowEvent::Occluded(_) => {}
                WindowEvent::RedrawRequested => {}

                _ => {
                    // dbg!(event);
                }
            };

            Ok(())
        } else {
            Ok(())
        }
    }
}
