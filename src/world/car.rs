use crate::utils::Vec2;
use std::cmp::PartialEq;

/// A vehicle with physics-based movement and control
///
/// The Car struct implements a simple physics simulation for a vehicle
/// that can accelerate, brake, and turn. It tracks its position, orientation,
/// and movement using a basic force-based physics model that includes:
///
/// * Position and orientation tracking
/// * Velocity-based movement
/// * Forward/reverse acceleration
/// * Turning with variable radius
/// * Speed-based friction
/// * Maximum speed limiting
///
/// The physics simulation uses frame-rate independent updates through
/// delta time calculations.
///
/// # Example
///
/// ```rust
/// let mut car = Car::new(100.0, 100.0);
///
/// // In game loop:
/// car.update(delta_time, throttle, brake, steering);
/// renderer.draw_car(&car);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Car {
    // TODO: Make position private and provide accessor methods
    /// Current position in world space
    pub position: Vec2,
    /// Unit vector pointing in the car's forward direction
    forward: Vec2,
    /// Current velocity vector (direction + speed)
    velocity: Vec2,
    /// Rate of acceleration in units per second squared
    acceleration: f32,
    /// Angular velocity in radians per second
    turn_speed: f32,
    /// Maximum speed in units per second
    max_speed: f32,
    /// Friction coefficient (higher = more drag)
    friction: f32,
    /// Current rotation angle in radians
    angle: f32,
}

impl Car {
    /// Creates a new car at the specified position
    ///
    /// Initializes a car with default physics parameters:
    /// * Acceleration: 400.0 units/s²
    /// * Turn speed: 2.0 rad/s
    /// * Max speed: 200.0 units/s
    /// * Friction: 1.0
    /// * Initial angle: 0.0 rad (facing up)
    /// * Initial velocity: zero
    ///
    /// # Arguments
    ///
    /// * `x` - Initial x-coordinate in world space
    /// * `y` - Initial y-coordinate in world space
    ///
    /// # Returns
    ///
    /// A new Car instance at the specified position
    ///
    /// # Example
    ///
    /// ```rust
    /// let car = Car::new(100.0, 200.0);
    /// assert_eq!(car.position, Vec2::new(100.0, 200.0));
    /// ```
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            forward: Vec2::new(0.0, 1.0), // Initially facing up
            velocity: Vec2::zero(),
            acceleration: 400.0,
            turn_speed: 2.0,
            max_speed: 200.0,
            friction: 1.0,
            angle: 0.0,
        }
    }

    /// Updates the car's physics state based on input controls
    ///
    /// This method performs the complete physics simulation update including:
    /// 1. Rotation based on steering input
    /// 2. Forward vector recalculation
    /// 3. Acceleration force application
    /// 4. Friction force application
    /// 5. Speed limiting
    /// 6. Position update
    ///
    /// All forces are scaled by delta time for frame-rate independence.
    ///
    /// # Arguments
    ///
    /// * `dt` - Delta time since last update in seconds
    /// * `throttle` - Forward/reverse control (-1.0 to 1.0)
    /// * `brake` - Braking force (0.0 to 1.0)
    /// * `steering` - Left/right control (-1.0 to 1.0)
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut car = Car::new(0.0, 0.0);
    ///
    /// // Apply full throttle for 1 second
    /// car.update(1.0, 1.0, 0.0, 0.0);
    /// ```
    pub fn update(&mut self, dt: f32, throttle: f32, brake: f32, steering: f32) {
        // Update rotation angle based on steering input
        if steering != 0.0 {
            self.angle += steering * self.turn_speed * dt;
        }

        // Recalculate forward vector from current angle
        self.forward = Vec2::new(-self.angle.sin(), self.angle.cos());

        // Calculate and apply acceleration force
        let mut accel_force = Vec2::zero();
        if throttle != 0.0 {
            accel_force = self.forward * (self.acceleration * throttle);
        } else if brake > 0.0 {
            accel_force = self.forward * (-self.acceleration * brake);
        }

        // Update velocity with acceleration
        self.velocity = self.velocity + accel_force * dt;

        // Apply friction force based on current speed
        let current_speed = self.velocity.length();
        if current_speed > 0.0 {
            let friction_force = self.friction * current_speed;
            let friction_direction = self.velocity.normalized();
            self.velocity = self.velocity + friction_direction * (-friction_force * dt);
        }

        // Limit to maximum speed
        let speed = self.velocity.length();
        if speed > self.max_speed {
            self.velocity = self.velocity.normalized() * self.max_speed;
        }

        // Update position based on final velocity
        self.position = self.position + self.velocity * dt;
    }

    /// Returns the car's current speed in units per second
    ///
    /// # Returns
    ///
    /// The magnitude of the car's velocity vector
    ///
    /// # Example
    ///
    /// ```rust
    /// let car = Car::new(0.0, 0.0);
    /// assert_eq!(car.get_speed(), 0.0);  // New cars start stationary
    /// ```
    pub fn get_speed(&self) -> f32 {
        self.velocity.length()
    }

    /// Returns the car's current rotation angle in radians
    ///
    /// # Returns
    ///
    /// The car's rotation angle in radians (counterclockwise from right)
    ///
    /// # Example
    ///
    /// ```rust
    /// let car = Car::new(0.0, 0.0);
    /// assert_eq!(car.get_angle(), 0.0);  // New cars start facing up
    /// ```
    pub fn get_angle(&self) -> f32 {
        self.angle
    }
}

/// Input controls for a car's movement
///
/// CarInput encapsulates the three main control inputs for a car:
/// throttle, turning, and braking. All inputs are normalized to the
/// range -1.0 to 1.0 for consistent control handling.
///
/// # Input Ranges
///
/// * Throttle: -1.0 (full reverse) to 1.0 (full forward)
/// * Turn: -1.0 (full right) to 1.0 (full left)
/// * Brake: -1.0 to 1.0 (full brake)
///
/// The struct enforces these ranges through assertions in the constructor.
#[derive(Debug, Clone, Copy)]
pub struct CarInput {
    /// Forward/reverse control (-1.0 to 1.0)
    pub throttle: f32,
    /// Left/right steering control (-1.0 to 1.0)
    pub turn: f32,
    /// Braking force control (-1.0 to 1.0)
    pub brake: f32,
}

impl CarInput {
    /// Creates a new set of car control inputs
    ///
    /// # Arguments
    ///
    /// * `throttle` - Forward/reverse control (-1.0 to 1.0)
    /// * `turn` - Left/right control (-1.0 to 1.0)
    /// * `brake` - Brake control (-1.0 to 1.0)
    ///
    /// # Panics
    ///
    /// Panics if any input value is outside the range -1.0 to 1.0
    ///
    /// # Example
    ///
    /// ```rust
    /// // Half throttle, quarter turn left, no brake
    /// let input = CarInput::new(0.5, 0.25, 0.0);
    ///
    /// // This would panic:
    /// // let invalid = CarInput::new(2.0, 0.0, 0.0);  // Throttle > 1.0
    /// ```
    pub fn new(throttle: f32, turn: f32, brake: f32) -> Self {
        assert!(
            (-1.0..=1.0).contains(&throttle),
            "throttle must be between -1.0 and 1.0"
        );
        assert!(
            (-1.0..=1.0).contains(&turn),
            "turn must be between -1.0 and 1.0"
        );
        assert!(
            (-1.0..=1.0).contains(&brake),
            "brake must be between -1.0 and 1.0"
        );

        Self {
            throttle,
            turn,
            brake,
        }
    }
}
