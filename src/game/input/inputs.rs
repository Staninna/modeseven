use crate::game::world::CarInput;
use pix_win_loop::{Context, KeyCode, NamedKey};

/// Input handler for two-player racing controls
///
/// Manages keyboard input for dual car control:
/// * Car 1: WASD keys + Space for brake
/// * Car 2: Arrow keys + Shift for brake
/// * Updates per-frame input state
/// * Converts key states to normalized controls
pub struct Inputs {
    // Car 1 - WASD controls
    /// Car 1 Forward movement (W)
    w: bool,
    /// Car 1 Backward movement (S)
    s: bool,
    /// Car 1 Left turn (A)
    a: bool,
    /// Car 1 Right turn (D)
    d: bool,
    /// Car 1 Brake (Space)
    space: bool,

    // Car 2 - Arrow controls
    /// Car 2 Forward movement (Up)
    up: bool,
    /// Car 2 Backward movement (Down)
    down: bool,
    /// Car 2 Left turn (Left)
    left: bool,
    /// Car 2 Right turn (Right)
    right: bool,
    /// Car 2 Brake (Shift)
    shift: bool,
}

impl Inputs {
    /// Creates a new input handler with keys unpressed
    ///
    /// # Returns
    ///
    /// New input state with all controls inactive
    pub fn new() -> Self {
        Self {
            w: false,
            s: false,
            a: false,
            d: false,
            space: false,
            up: false,
            down: false,
            left: false,
            right: false,
            shift: false,
        }
    }

    /// Updates key states from keyboard input
    ///
    /// # Arguments
    ///
    /// * `ctx` - Current input context
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn update(&mut self, ctx: &Context) -> &Self {
        // Update WASD states
        self.w = ctx.input.is_physical_key_down(KeyCode::KeyW);
        self.s = ctx.input.is_physical_key_down(KeyCode::KeyS);
        self.a = ctx.input.is_physical_key_down(KeyCode::KeyA);
        self.d = ctx.input.is_physical_key_down(KeyCode::KeyD);
        self.space = ctx.input.is_physical_key_down(KeyCode::Space);

        // Update arrow key states
        self.up = ctx.input.is_physical_key_down(KeyCode::ArrowUp);
        self.down = ctx.input.is_physical_key_down(KeyCode::ArrowDown);
        self.left = ctx.input.is_physical_key_down(KeyCode::ArrowLeft);
        self.right = ctx.input.is_physical_key_down(KeyCode::ArrowRight);
        self.shift = ctx.input.is_logical_key_down(NamedKey::Shift);

        self
    }

    /// Converts current key states to car control inputs
    ///
    /// # Returns
    ///
    /// Array of two CarInputs:
    /// * \[0\]: Car 1 controls from WASD
    /// * \[1\]: Car 2 controls from arrows
    pub fn get_car_inputs(&self) -> [CarInput; 2] {
        [self.get_car1_input(), self.get_car2_input()]
    }

    /// Processes WASD controls for car 1
    ///
    /// Creates normalized inputs (-1.0 to 1.0):
    /// * W/S: Forward/Backward throttle
    /// * A/D: Left/Right steering
    /// * Space: Brake (0.0 to 1.0)
    fn get_car1_input(&self) -> CarInput {
        // Calculate control values
        let throttle = if self.w {
            1.0
        } else if self.s {
            -1.0
        } else {
            0.0
        };

        let turn = if self.a {
            1.0
        } else if self.d {
            -1.0
        } else {
            0.0
        };

        let brake = if self.space { 1.0 } else { 0.0 };

        CarInput::new(throttle, turn, brake)
    }

    /// Processes arrow key controls for car 2
    ///
    /// Creates normalized inputs (-1.0 to 1.0):
    /// * Up/Down: Forward/Backward throttle
    /// * Left/Right: Left/Right steering  
    /// * Shift: Brake (0.0 to 1.0)
    fn get_car2_input(&self) -> CarInput {
        // Calculate control values
        let throttle = if self.up {
            1.0
        } else if self.down {
            -1.0
        } else {
            0.0
        };

        let turn = if self.left {
            1.0
        } else if self.right {
            -1.0
        } else {
            0.0
        };

        let brake = if self.shift { 1.0 } else { 0.0 };

        CarInput::new(throttle, turn, brake)
    }
}
