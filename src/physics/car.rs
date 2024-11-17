use std::cmp::PartialEq;
use crate::physics::Vec2;

#[derive(Debug, Clone, PartialEq)]
pub struct Car {
    pub position: Vec2,
    forward: Vec2,
    velocity: Vec2,
    acceleration: f32,
    turn_speed: f32,
    max_speed: f32,
    friction: f32,
    pub angle: f32,
}

impl Car {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            forward: Vec2::new(0.0, 1.0),
            velocity: Vec2::zero(),
            acceleration: 400.0,
            turn_speed: 2.0,
            max_speed: 200.0,
            friction: 1.0,
            angle: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, throttle: f32, brake: f32, steering: f32) {
        // Update angle
        if steering != 0.0 {
            self.angle += steering * self.turn_speed * dt;
        }

        // Update forward vector based on current angle
        self.forward = Vec2::new(
            -self.angle.sin(),
            self.angle.cos()
        );

        // Calculate acceleration force
        let mut accel_force = Vec2::zero();
        if throttle != 0.0 {
            accel_force = self.forward * (self.acceleration * throttle);
        } else if brake > 0.0 {
            accel_force = self.forward * (-self.acceleration * brake);
        }

        // Apply acceleration to velocity
        self.velocity = self.velocity + accel_force * dt;

        // Apply friction
        let current_speed = self.velocity.length();
        if current_speed > 0.0 {
            let friction_force = self.friction * current_speed;
            let friction_direction = self.velocity.normalized();
            self.velocity = self.velocity + friction_direction * (-friction_force * dt);
        }

        // Clamp to max speed
        let speed = self.velocity.length();
        if speed > self.max_speed {
            self.velocity = self.velocity.normalized() * self.max_speed;
        }

        // Update position
        self.position = self.position + self.velocity * dt;
    }

    pub fn get_speed(&self) -> f32 {
        self.velocity.length()
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }
}

#[derive(Debug, Clone, Copy)]

pub struct CarInput {
    pub throttle: f32, // -1.0 to 1.0
    pub turn: f32,     // -1.0 to 1.0
    pub brake: f32,    // -1.0 to 1.0
}

impl CarInput {
    pub fn new(throttle: f32, turn: f32, brake: f32) -> Self {
        assert!(throttle >= -1.0 && throttle <= 1.0, "throttle must be between -1.0 and 1.0");
        assert!(turn >= -1.0 && turn <= 1.0, "turn must be between -1.0 and 1.0");
        assert!(brake >= -1.0 && brake <= 1.0, "brake must be between -1.0 and 1.0");
        
        Self {
            throttle,
            turn,
            brake,
        }
    }
}
