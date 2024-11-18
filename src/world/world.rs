use crate::input::Inputs;
use crate::world::Car;

/// The main game world containing all dynamic game entities
///
/// The World struct serves as the central container for all active game objects
/// and manages their interactions. In its current implementation, it manages a
/// two-player racing game with two cars. The world updates all entities based
/// on player inputs and time progression.
///
/// Each car in the world:
/// * Has an independent position and state
/// * Receives and responds to player inputs
/// * Updates its physics simulation each frame
///
/// # Example
///
/// ```rust
/// let mut world = World::new();
/// let inputs = Inputs::new();
///
/// // In game loop:
/// world.update(&inputs, delta_time);
/// ```
pub struct World {
    /// Array of two cars for a two-player racing game
    ///
    /// Index 0 is player 1's car (WASD controls)
    /// Index 1 is player 2's car (Arrow key controls)
    pub cars: [Car; 2],
}

impl World {
    /// Creates a new game world with default car positions
    ///
    /// Initializes a world with two cars positioned at one-third of the way
    /// across a 1024x1024 game area. Both cars start at the same position
    /// but can be controlled independently.
    ///
    /// The default car positions are:
    /// * x: 341.33... (1024/3)
    /// * y: 341.33... (1024/3)
    ///
    /// # Returns
    ///
    /// A new World instance with two cars at their starting positions
    ///
    /// # Example
    ///
    /// ```rust
    /// let world = World::new();
    /// assert_eq!(world.cars.len(), 2);
    /// ```
    pub fn new() -> Self {
        let car1 = Car::new(1024.0 / 3.0, 1024.0 / 3.0);
        let car2 = Car::new(1024.0 / 3.0, 1024.0 / 3.0);

        Self {
            cars: [car1, car2],
        }
    }

    /// Updates the state of all entities in the world
    ///
    /// This method advances the game simulation by one time step. It:
    /// 1. Processes player inputs from both control schemes
    /// 2. Updates each car's physics and position
    /// 3. Applies any resulting effects or interactions
    ///
    /// The update is frame-rate independent through the use of delta time,
    /// ensuring consistent physics regardless of update frequency.
    ///
    /// # Arguments
    ///
    /// * `inputs` - Current state of player inputs for both cars
    /// * `dt` - Delta time since last update in seconds
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut world = World::new();
    /// let inputs = Inputs::new();
    ///
    /// // Update with 16.6ms frame time (approximately 60 FPS)
    /// world.update(&inputs, 0.0166);
    /// ```
    pub fn update(&mut self, inputs: &Inputs, dt: f32) {
        // Destructure cars for individual updates
        let [car1, car2] = &mut self.cars;

        // Get current inputs for both cars
        let [car1_input, car2_input] = inputs.get_car_inputs();

        // Update each car's state based on inputs and time step
        car1.update(dt, car1_input.throttle, car1_input.brake, car1_input.turn);
        car2.update(dt, car2_input.throttle, car2_input.brake, car2_input.turn);
    }
}