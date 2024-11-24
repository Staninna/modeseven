//! Game world state and update logic

use super::super::input::Inputs;
use super::Car;

/// The main game world containing all dynamic game entities
///
/// The World struct manages a two-player racing game with two cars. Each car:
/// * Has independent physics and controls
/// * Updates based on player inputs (WASD or Arrow keys)
/// * Maintains its own position and state
///
/// All world updates are frame-rate independent through delta time scaling.
pub struct World {
    /// Array of two cars for the racing game
    /// Index 0: Player 1 (WASD controls)
    /// Index 1: Player 2 (Arrow controls)
    pub cars: [Car; 2],
}

impl World {
    /// Creates a new game world with two cars at default positions
    ///
    /// # Returns
    ///
    /// A new World instance with:
    /// * Two cars
    pub fn new() -> Self {
        let car1 = Car::new(1024.0 / 3.0, 1024.0 / 3.0);
        let car2 = Car::new(1024.0 / 3.3, 1024.0 / 3.3);

        Self { cars: [car1, car2] }
    }

    /// Updates the state of all entities in the world
    ///
    /// # Arguments
    ///
    /// * `inputs` - Current state of player inputs
    /// * `dt` - Delta time in seconds
    ///
    /// Updates both cars' physics and positions based on their
    /// respective player inputs and the time step.
    pub fn update(&mut self, inputs: &Inputs, dt: f32) {
        let [car1, car2] = &mut self.cars;
        let [car1_input, car2_input] = inputs.get_car_inputs();

        car1.update(
            dt,
            car1_input.throttle(),
            car1_input.brake(),
            car1_input.turn(),
        );
        car2.update(
            dt,
            car2_input.throttle(),
            car2_input.brake(),
            car2_input.turn(),
        );
    }
}
