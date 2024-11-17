use std::any::Any;
use crate::world::objects::behavior::WorldBehavior;
use crate::world::objects::object::WorldObject;

#[derive(Clone)]
pub struct PowerUpBehavior {
    radius: f32,
    duration: f32,
    remaining_time: f32,
    collected: bool,
}

impl PowerUpBehavior {
    pub fn new(radius: f32, duration: f32) -> Self {
        Self {
            radius,
            duration,
            remaining_time: duration,
            collected: false,
        }
    }

    pub fn is_collected(&self) -> bool {
        self.collected
    }

    pub fn remaining_time(&self) -> f32 {
        self.remaining_time
    }
}

impl WorldBehavior for PowerUpBehavior {
    fn update(&mut self, dt: f32) {
        if self.collected {
            self.remaining_time = (self.remaining_time - dt).max(0.0);
        }
    }

    fn on_trigger(&mut self, _other: &WorldObject) {
        self.collected = true;
    }

    fn is_trigger(&self) -> bool {
        !self.collected
    }

    fn trigger_radius(&self) -> Option<f32> {
        Some(self.radius)
    }

    fn clone_box(&self) -> Box<dyn WorldBehavior> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}