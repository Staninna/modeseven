use std::any::Any;
use crate::world::objects::object::WorldObject;

pub trait WorldBehavior: Any {
    fn update(&mut self, dt: f32);
    fn on_trigger(&mut self, other: &WorldObject);
    fn is_trigger(&self) -> bool;
    fn trigger_radius(&self) -> Option<f32>;
    // Method to clone the behavior while maintaining object safety
    fn clone_box(&self) -> Box<dyn WorldBehavior>;
    // Downcast to a specific type if possible
    fn as_any(&self) -> &dyn Any;
}

// Empty behavior implementation
#[derive(Clone, Default)]
pub struct EmptyBehavior;

impl WorldBehavior for EmptyBehavior {
    fn update(&mut self, _dt: f32) {}

    fn on_trigger(&mut self, _other: &WorldObject) {}

    fn is_trigger(&self) -> bool {
        false
    }

    fn trigger_radius(&self) -> Option<f32> {
        None
    }

    fn clone_box(&self) -> Box<dyn WorldBehavior> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}