use crate::world::Car;
use std::f32::consts::PI;
use crate::utils::Vec2;

/// Camera for dynamic car following and view control 
///
/// Provides:
/// * Smooth position transitions
/// * Dynamic height adjustment
/// * Speed-based view angle changes
/// * Car following behavior
/// * View frustum control
pub struct Camera {
    /// World X position
    pub x: f32,
    /// World Y position
    pub y: f32,
    /// Height above ground
    pub height: f32,
    /// Rotation angle in radians
    pub angle: f32,
    /// Downward tilt in radians
    pub pitch: f32,
    /// Near clip distance
    pub near: f32,
    /// Far clip distance
    pub far: f32,
    /// View scale factor
    pub scale: f32,
}

impl Default for Camera {
    /// Creates a camera at origin with default parameters
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl Camera {
    /// Creates a camera with given position and angle
    ///
    /// # Arguments
    ///
    /// * `x` - World X coordinate
    /// * `y` - World Y coordinate
    /// * `height` - Height above ground
    /// * `angle` - Rotation in radians
    ///
    /// # Returns
    ///
    /// Camera with default viewing parameters:
    /// * 30° pitch (π/6)  
    /// * 1.0 near plane
    /// * 1000.0 far plane
    /// * 1.0 scale
    pub fn new(x: f32, y: f32, height: f32, angle: f32) -> Self {
        Self {
            x,
            y,
            height,
            angle,
            pitch: PI / 6.0,
            near: 1.0,
            far: 1000.0,
            scale: 1.0, // Funny to tweak
        }
    }

    /// Updates camera to follow a car with smooth transitions
    ///
    /// Adjusts camera parameters based on car state:
    /// * Position tracks behind car
    /// * Height increases with speed 
    /// * Pitch tilts down more at high speeds
    /// * Rotation matches car direction
    ///
    /// Uses constant factors:
    /// * FOLLOW_DISTANCE: 0.0 (centered)
    /// * CAMERA_LERP: 5.0 (position speed)
    /// * ANGLE_LERP: 3.0 (rotation speed)
    pub fn follow_car(&mut self, car: &Car, dt: f32) {
        const FOLLOW_DISTANCE: f32 = 0.0;
        const CAMERA_LERP: f32 = 5.0;
        const ANGLE_LERP: f32 = 3.0;

        // Calculate target position behind car
        let car_angle = car.angle();
        let target_x = car.position().x - FOLLOW_DISTANCE * car_angle.sin();
        let target_y = car.position().y - FOLLOW_DISTANCE * car_angle.cos();

        // Smoothly move camera
        self.x += (target_x - self.x) * CAMERA_LERP * dt;
        self.y += (target_y - self.y) * CAMERA_LERP * dt;

        // Find shortest rotation path
        let mut angle_diff = car_angle - self.angle;
        while angle_diff > PI { angle_diff -= 2.0 * PI; }
        while angle_diff < -PI { angle_diff += 2.0 * PI; }
        self.angle += angle_diff * ANGLE_LERP * dt;

        // Adjust height and pitch with speed
        let target_height = 15.0 + car.speed() * 0.05;
        self.height += (target_height - self.height) * CAMERA_LERP * dt;

        let target_pitch = PI / 6.0 + (car.speed() / 400.0) * (PI / 12.0);
        self.pitch += (target_pitch - self.pitch) * CAMERA_LERP * dt;
    }
    
    /// Converts world coordinates to screen coordinates
    ///
    /// # Arguments
    ///
    /// * `world_pos` - World coordinates
    ///
    /// # Returns
    ///
    /// Screen coordinates
    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        let screen_pos = Vec2::new(
            world_pos.x * self.scale + self.x,
            world_pos.y * self.scale + self.y,
        );

        screen_pos
    }
}