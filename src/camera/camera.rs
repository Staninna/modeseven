use std::f32::consts::PI;
use crate::utils::vec2::Vec2;
use crate::world::Car;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub height: f32,
    pub angle: f32,
    pub pitch: f32,
    pub near: f32,
    pub far: f32,
    pub scale: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl Camera {
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

    pub fn follow_car(&mut self, car: &Car, dt: f32) {
        const FOLLOW_DISTANCE: f32 = 0.0;
        const CAMERA_LERP: f32 = 5.0;
        const ANGLE_LERP: f32 = 3.0;

        // Get car's angle
        let car_angle = car.get_angle();

        // Calculate ideal camera position
        let target_x = car.position.x - FOLLOW_DISTANCE * car_angle.sin();
        let target_y = car.position.y - FOLLOW_DISTANCE * car_angle.cos();

        // Smoothly move camera to target position
        self.x += (target_x - self.x) * CAMERA_LERP * dt;
        self.y += (target_y - self.y) * CAMERA_LERP * dt;

        // Smoothly rotate camera to match car's direction (angle difference)
        let mut angle_diff = car_angle - self.angle;

        // Make sure we rotate the shortest way
        while angle_diff > PI {
            angle_diff -= 2.0 * PI;
        }
        while angle_diff < -PI {
            angle_diff += 2.0 * PI;
        }

        self.angle += angle_diff * ANGLE_LERP * dt;

        // Calculate camera height
        let target_height = 15.0 + car.get_speed() * 0.01;
        self.height += (target_height - self.height) * CAMERA_LERP * dt;

        // Adjust pitch based on speed (look further ahead at higher speeds)
        let target_pitch = PI / 6.0 + (car.get_speed() / 400.0) * (PI / 12.0);
        self.pitch += (target_pitch - self.pitch) * CAMERA_LERP * dt;
    }
}