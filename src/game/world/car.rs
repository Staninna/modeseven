//! Vehicle physics simulation

use super::super::rendering::Renderable;
use super::World;
use crate::consts::CAR_FILE;
use glam::Vec2;
use std::cmp::PartialEq;

/// A vehicle with physics-based movement and control
///
/// The Car struct implements a physics simulation for a vehicle that can
/// accelerate, brake, and turn. It uses a simplified force-based model with:
///
/// * Position and velocity tracking
/// * Forward/reverse/breaking acceleration with quadratic air resistance
/// * Speed-dependent turning radius
/// * Viscous friction at low speeds
/// * Maximum speed limiting
///
/// All physics calculations are frame-rate independent through delta time scaling.
#[derive(Debug, Clone, PartialEq)]
pub struct Car {
    /// Current position in world space (read-only)
    position: Vec2,
    /// Normalized vector pointing in car's forward direction
    forward: Vec2,
    /// Current velocity vector in units per second
    velocity: Vec2,
    /// Rate of acceleration in units/s²
    acceleration: f32,
    /// Maximum turning rate in radians/s
    turn_speed: f32,
    /// Maximum speed in units/s
    max_speed: f32,
    /// Quadratic drag coefficient
    drag: f32,
    /// Linear friction coefficient for low speeds
    friction: f32,
    /// Current rotation in radians (counterclockwise from vertical)
    angle: f32,
}

impl Car {
    /// Creates a new car at the specified position with default physics parameters
    ///
    /// # Arguments
    ///
    /// * `x` - Initial x-coordinate
    /// * `y` - Initial y-coordinate
    ///
    /// # Returns
    ///
    /// A new Car instance with:
    /// * Acceleration: 400.0 units/s²
    /// * Turn speed: 2.0 rad/s (8.0 now but needs to be tuned)
    /// * Max speed: 200.0 units/s
    /// * Drag coefficient: 0.001
    /// * Friction: 0.8
    /// * Initial angle: 0.0 rad (vertical)
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            forward: Vec2::new(0.0, 1.0),
            velocity: Vec2::ZERO,
            acceleration: 400.0,
            turn_speed: 8.0, // when we stand still we rotate extremely fast bug == feature???
            max_speed: 200.0,
            drag: 0.005,
            friction: 0.95,
            angle: 0.0,
        }
    }

    /// Updates the car's physics state based on input controls
    ///
    /// # Arguments
    ///
    /// * `dt` - Delta time in seconds
    /// * `throttle` - Forward/reverse control (-1.0 to 1.0)
    /// * `brake` - Braking force (0.0 to 1.0)
    /// * `steering` - Left/right control (-1.0 to 1.0)
    pub fn update(&mut self, dt: f32, throttle: f32, brake: f32, steering: f32) {
        // Update rotation with speed-dependent turning
        if steering != 0.0 {
            let speed_factor = 1.0 - (self.speed() / self.max_speed).min(0.8);
            self.angle += steering * self.turn_speed * speed_factor * dt;

            // Recalculate and normalize forward vector
            self.forward = Vec2::new(-self.angle.sin(), self.angle.cos());
            self.forward = self.forward.normalize();
        }

        // Apply acceleration force
        let mut accel_force = if throttle != 0.0 {
            self.forward * (self.acceleration * throttle)
        } else if brake > 0.0 && self.velocity.length() > 0.1 {
            // Apply brake force against current velocity direction
            -self.velocity.normalize() * (self.acceleration * brake)
        } else {
            Vec2::ZERO
        };

        // Apply quadratic drag at higher speeds
        let speed = self.velocity.length();
        if speed > 1.0 {
            let drag_force = -self.velocity.normalize() * (self.drag * speed * speed);
            accel_force = accel_force + drag_force;
        } else {
            // Apply linear friction at low speeds
            accel_force = accel_force - self.velocity * self.friction;
        }

        // Update velocity with forces
        self.velocity = self.velocity + accel_force * dt;

        // Apply speed limit
        let speed = self.velocity.length();
        if speed > self.max_speed {
            self.velocity = self.velocity.normalize() * self.max_speed;
        }

        // Update position
        self.position = self.position + self.velocity * dt;
    }

    /// Returns the current position
    pub fn position(&self) -> Vec2 {
        self.position
    }

    /// Returns the forward direction vector
    pub fn forward(&self) -> Vec2 {
        self.forward
    }

    /// Returns the current speed in units per second
    pub fn speed(&self) -> f32 {
        self.velocity.length()
    }

    /// Returns the current rotation angle in radians
    pub fn angle(&self) -> f32 {
        self.angle
    }
}

/// Input controls for car movement, with value range validation
///
/// All inputs are normalized to -1.0 to 1.0:
/// * `throttle`: -1.0 (full reverse) to 1.0 (full forward)
/// * `turn`: -1.0 (full right) to 1.0 (full left)
/// * `brake`: 0.0 to 1.0 (full brake)
#[derive(Debug, Clone, Copy)]
pub struct CarInput {
    throttle: f32,
    turn: f32,
    brake: f32,
}

impl CarInput {
    /// Creates new validated car control inputs
    ///
    /// # Panics
    ///
    /// Panics if inputs exceed their valid ranges
    pub fn new(throttle: f32, turn: f32, brake: f32) -> Self {
        assert!(
            (-1.0..=1.0).contains(&throttle),
            "Invalid throttle range: {}",
            throttle
        );
        assert!((-1.0..=1.0).contains(&turn), "Invalid turn range: {}", turn);
        assert!(
            (0.0..=1.0).contains(&brake),
            "Invalid brake range: {}",
            brake
        );

        Self {
            throttle,
            turn,
            brake,
        }
    }

    /// Get the throttle input value
    pub fn throttle(&self) -> f32 {
        self.throttle
    }

    /// Get the turn input value
    pub fn turn(&self) -> f32 {
        self.turn
    }

    /// Get the brake input value
    pub fn brake(&self) -> f32 {
        self.brake
    }
}

impl Renderable for Car {
    fn position(&self) -> Vec2 {
        self.position()
    }

    fn base_size(&self) -> f32 {
        60.0 // Base car size
    }

    fn texture_file(&self, world: &World) -> &str {
        CAR_FILE
    }
}
