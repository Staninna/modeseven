use std::any::Any;
use crate::world::objects::behavior::WorldBehavior;
use crate::world::objects::object::WorldObject;

#[derive(Clone)]
pub struct CheckpointBehavior {
    radius: f32,
    triggered: bool,
}

impl CheckpointBehavior {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            triggered: false,
        }
    }

    pub fn is_triggered(&self) -> bool {
        self.triggered
    }
}

impl WorldBehavior for CheckpointBehavior {
    fn update(&mut self, _dt: f32) {}

    fn on_trigger(&mut self, _other: &WorldObject) {
        self.triggered = true;
    }

    fn is_trigger(&self) -> bool {
        true
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