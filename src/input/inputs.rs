use crate::world::CarInput;
use pix_win_loop::{Context, KeyCode};

/// A handler for keyboard input that manages controls for two cars
///
/// The Inputs struct manages keyboard input state for a two-player racing game,
/// supporting simultaneous control of two cars. The first car uses WASD controls,
/// while the second car uses arrow keys. Each frame, the input state is updated
/// and can be converted into car control parameters (throttle, steering, and braking).
///
/// # Controls
///
/// Car 1 (WASD):
/// - W: Accelerate/Throttle
/// - S: Reverse/Brake
/// - A: Steer Left
/// - D: Steer Right
///
/// Car 2 (Arrow Keys):
/// - Up: Accelerate/Throttle
/// - Down: Reverse/Brake
/// - Left: Steer Left
/// - Right: Steer Right
///
/// # Example
///
/// ```rust
/// let mut inputs = Inputs::new();
///
/// // In game loop:
/// inputs.update(&context);
/// let [car1_input, car2_input] = inputs.get_car_inputs();
/// ```
pub struct Inputs {
    // Car 1 controls (WASD)
    /// W key state - Forward movement for car 1
    w: bool,
    /// S key state - Backward movement for car 1
    s: bool,
    /// A key state - Left turn for car 1
    a: bool,
    /// D key state - Right turn for car 1
    d: bool,

    // Car 2 controls (Arrow keys)
    /// Up arrow key state - Forward movement for car 2
    up: bool,
    /// Down arrow key state - Backward movement for car 2
    down: bool,
    /// Left arrow key state - Left turn for car 2
    left: bool,
    /// Right arrow key state - Right turn for car 2
    right: bool,
}

impl Inputs {
    /// Creates a new Inputs instance with all controls initialized to unpressed
    ///
    /// # Returns
    ///
    /// A new `Inputs` instance with all boolean flags set to false
    pub fn new() -> Self {
        Self {
            w: false,
            s: false,
            a: false,
            d: false,
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    /// Updates the input state based on the current keyboard state
    ///
    /// This method should be called once per frame to update the internal
    /// state of which keys are currently pressed. It queries the physical
    /// key states for both WASD and arrow key controls.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The game context containing current input state
    ///
    /// # Returns
    ///
    /// A reference to self for method chaining
    ///
    /// # Example
    ///
    /// ```rust
    /// inputs.update(&context).get_car_inputs();
    /// ```
    pub fn update(&mut self, ctx: &Context) -> &Self {
        // Update key states for both control schemes
        self.w = ctx.input.is_physical_key_down(KeyCode::KeyW);
        self.s = ctx.input.is_physical_key_down(KeyCode::KeyS);
        self.a = ctx.input.is_physical_key_down(KeyCode::KeyA);
        self.d = ctx.input.is_physical_key_down(KeyCode::KeyD);
        self.up = ctx.input.is_physical_key_down(KeyCode::ArrowUp);
        self.down = ctx.input.is_physical_key_down(KeyCode::ArrowDown);
        self.left = ctx.input.is_physical_key_down(KeyCode::ArrowLeft);
        self.right = ctx.input.is_physical_key_down(KeyCode::ArrowRight);

        self
    }

    /// Converts the current input state into control inputs for both cars
    ///
    /// This method processes the raw key states and converts them into normalized
    /// control values for throttle, turning, and braking for both cars. The values
    /// are returned as an array of two CarInputs, one for each car.
    ///
    /// # Returns
    ///
    /// An array containing two CarInput instances, where:
    /// * index 0 contains inputs for car 1 (WASD controls)
    /// * index 1 contains inputs for car 2 (arrow key controls)
    ///
    /// # Example
    ///
    /// ```rust
    /// let [car1_input, car2_input] = inputs.get_car_inputs();
    /// world.update_cars(car1_input, car2_input);
    /// ```
    pub fn get_car_inputs(&self) -> [CarInput; 2] {
        let car1_input = self.get_car1_input();
        let car2_input = self.get_car2_input();
        [car1_input, car2_input]
    }

    /// Processes WASD inputs into control values for car 1
    ///
    /// Converts the current state of the WASD keys into normalized control values:
    /// * Throttle: W increases (1.0), S decreases (-1.0)
    /// * Turn: A increases (1.0), D decreases (-1.0)
    /// * Brake: S increases (1.0), W decreases (-1.0)
    ///
    /// # Returns
    ///
    /// A CarInput instance containing the processed control values
    fn get_car1_input(&self) -> CarInput {
        // Calculate throttle (-1.0 to 1.0)
        let mut throttle = 0.0;
        if self.w {
            throttle += 1.0;
        }
        if self.s {
            throttle -= 1.0;
        }

        // Calculate steering (-1.0 to 1.0)
        let mut turn = 0.0;
        if self.a {
            turn += 1.0;
        }
        if self.d {
            turn -= 1.0;
        }

        // Calculate brake (0.0 to 1.0)
        let mut brake = 0.0;
        if self.w {
            brake -= 1.0;
        }
        if self.s {
            brake += 1.0;
        }

        CarInput::new(throttle, turn, brake)
    }

    /// Processes arrow key inputs into control values for car 2
    ///
    /// Converts the current state of the arrow keys into normalized control values:
    /// * Throttle: Up increases (1.0), Down decreases (-1.0)
    /// * Turn: Left increases (1.0), Right decreases (-1.0)
    /// * Brake: Down increases (1.0), Up decreases (-1.0)
    ///
    /// # Returns
    ///
    /// A CarInput instance containing the processed control values
    fn get_car2_input(&self) -> CarInput {
        // Calculate throttle (-1.0 to 1.0)
        let mut throttle = 0.0;
        if self.up {
            throttle += 1.0;
        }
        if self.down {
            throttle -= 1.0;
        }

        // Calculate steering (-1.0 to 1.0)
        let mut turn = 0.0;
        if self.left {
            turn += 1.0;
        }
        if self.right {
            turn -= 1.0;
        }

        // Calculate brake (0.0 to 1.0)
        let mut brake = 0.0;
        if self.up {
            brake -= 1.0;
        }
        if self.down {
            brake += 1.0;
        }

        CarInput::new(throttle, turn, brake)
    }
}
