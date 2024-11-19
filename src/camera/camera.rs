use crate::world::Car;
use std::f32::consts::PI;

/// A dynamic camera that can follow cars with smooth transitions
///
/// The Camera struct represents a viewpoint in the game world, defining both position
/// and orientation. It supports dynamic following behavior with smooth transitions,
/// making it ideal for racing games. The camera adjusts its height and pitch based
/// on the car's speed, providing better visibility at higher speeds.
///
/// Camera parameters include:
/// * Position (x, y)
/// * Height above ground
/// * Viewing angle (yaw)
/// * Pitch (tilt down angle)
/// * Near/far clipping planes
/// * Scale factor
///
/// # Example
///
/// ```rust
/// let mut camera = Camera::new(0.0, 0.0, 15.0, 0.0);
///
/// // In game loop:
/// camera.follow_car(&player_car, delta_time);
/// renderer.render(&camera);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    /// X-coordinate in world space
    pub x: f32,
    /// Y-coordinate in world space
    pub y: f32,
    /// Height above the ground plane
    pub height: f32,
    /// Horizontal rotation angle (yaw) in radians
    pub angle: f32,
    /// Vertical tilt angle (pitch) in radians
    pub pitch: f32,
    /// Near clipping plane distance
    pub near: f32,
    /// Far clipping plane distance
    pub far: f32,
    /// View scale factor (affects perceived field of view)
    pub scale: f32,
}

impl Default for Camera {
    /// Creates a new camera at the origin with default parameters
    ///
    /// Equivalent to `Camera::new(0.0, 0.0, 0.0, 0.0)`
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl Camera {
    /// Creates a new camera with specified position and orientation
    ///
    /// Initializes a camera with the given position and angle, using default values
    /// for other parameters:
    /// * pitch: 30 degrees (π/6 radians)
    /// * near: 1.0 units
    /// * far: 1000.0 units
    /// * scale: 1.0
    ///
    /// # Arguments
    ///
    /// * `x` - Initial x-coordinate in world space
    /// * `y` - Initial y-coordinate in world space
    /// * `height` - Initial height above the ground plane
    /// * `angle` - Initial horizontal rotation angle in radians
    ///
    /// # Returns
    ///
    /// A new Camera instance with the specified parameters
    ///
    /// # Example
    ///
    /// ```rust
    /// // Create a camera 15 units above the origin, looking along positive X
    /// let camera = Camera::new(0.0, 0.0, 15.0, 0.0);
    /// ```
    pub fn new(x: f32, y: f32, height: f32, angle: f32) -> Self {
        Self {
            x,
            y,
            height,
            angle,
            pitch: PI / 6.0, // 30 degrees
            near: 1.0,
            far: 1000.0,
            scale: 1.0,
        }
    }

    /// Updates camera position and orientation to smoothly follow a car
    ///
    /// This method implements a dynamic camera system that follows a car with smooth
    /// transitions. It adjusts multiple camera parameters based on the car's state:
    ///
    /// * Position: Smoothly moves toward a position behind the car
    /// * Angle: Gradually rotates to match the car's direction
    /// * Height: Increases with car speed for better visibility
    /// * Pitch: Adjusts to look further ahead at higher speeds
    ///
    /// The camera's movement is controlled by several constants:
    /// * FOLLOW_DISTANCE: Distance to maintain behind the car (0.0 for centered)
    /// * CAMERA_LERP: Position and height transition speed (5.0)
    /// * ANGLE_LERP: Rotation transition speed (3.0)
    ///
    /// The method uses linear interpolation (lerp) for smooth transitions, with
    /// the interpolation factor scaled by delta time for frame-rate independence.
    ///
    /// # Arguments
    ///
    /// * `car` - Reference to the Car to follow
    /// * `dt` - Delta time since last update in seconds
    ///
    /// # Example
    ///
    /// ```rust
    /// camera.follow_car(&player_car, 0.016);  // 60 FPS
    /// ```
    pub fn follow_car(&mut self, car: &Car, dt: f32) {
        const FOLLOW_DISTANCE: f32 = 0.0; // Distance behind car
        const CAMERA_LERP: f32 = 5.0; // Position transition speed
        const ANGLE_LERP: f32 = 3.0; // Rotation transition speed

        // Get car's current angle
        let car_angle = car.get_angle();

        // Calculate ideal camera position behind the car
        let target_x = car.position.x - FOLLOW_DISTANCE * car_angle.sin();
        let target_y = car.position.y - FOLLOW_DISTANCE * car_angle.cos();

        // Smoothly interpolate camera position
        self.x += (target_x - self.x) * CAMERA_LERP * dt;
        self.y += (target_y - self.y) * CAMERA_LERP * dt;

        // Calculate the shortest rotation path to match car's angle
        let mut angle_diff = car_angle - self.angle;
        while angle_diff > PI {
            angle_diff -= 2.0 * PI;
        }
        while angle_diff < -PI {
            angle_diff += 2.0 * PI;
        }
        self.angle += angle_diff * ANGLE_LERP * dt;

        // Adjust camera height based on car speed
        let target_height = 15.0 + car.get_speed() * 0.05;
        self.height += (target_height - self.height) * CAMERA_LERP * dt;

        // Dynamic pitch adjustment for better visibility at high speeds
        // Pitch ranges from 30 degrees (PI/6) at rest to 45 degrees (PI/4) at max speed
        let target_pitch = PI / 6.0 + (car.get_speed() / 400.0) * (PI / 12.0);
        self.pitch += (target_pitch - self.pitch) * CAMERA_LERP * dt;
    }
}
