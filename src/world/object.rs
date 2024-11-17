use std::any::Any;
use crate::{
    physics::Vec2,
    rendering::Sprite,
};

// Base trait for behaviors - object safe
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

// Example of a checkpoint behavior
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObjectType {
    Checkpoint,
    PowerUp,
    Obstacle,
    Trigger,
    Decoration,
}

pub struct WorldObject {
    pub id: u64,
    pub object_type: ObjectType,
    pub position: Vec2,
    pub rotation: f32,
    pub active: bool,
    pub sprite: Option<Sprite>,
    behavior: Box<dyn WorldBehavior>,
}

// Manual Clone implementation for WorldObject
impl Clone for WorldObject {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            object_type: self.object_type,
            position: self.position,
            rotation: self.rotation,
            active: self.active,
            sprite: self.sprite.clone(),
            behavior: self.behavior.clone_box(),
        }
    }
}

impl WorldObject {
    pub fn new(
        id: u64,
        object_type: ObjectType,
        position: Vec2,
        sprite: Option<Sprite>,
        behavior: Box<dyn WorldBehavior>
    ) -> Self {
        Self {
            id,
            object_type,
            position,
            rotation: 0.0,
            active: true,
            sprite,
            behavior,
        }
    }

    // Create a simple decorative object
    pub fn decoration(id: u64, position: Vec2, sprite: Sprite) -> Self {
        Self::new(
            id,
            ObjectType::Decoration,
            position,
            Some(sprite),
            Box::new(EmptyBehavior::default())
        )
    }

    // Create a checkpoint
    pub fn checkpoint(id: u64, position: Vec2, sprite: Option<Sprite>, radius: f32) -> Self {
        Self::new(
            id,
            ObjectType::Checkpoint,
            position,
            sprite,
            Box::new(CheckpointBehavior::new(radius))
        )
    }
    
    // Create a power-up
    pub fn power_up(id: u64, position: Vec2, sprite: Option<Sprite>, radius: f32, duration: f32) -> Self {
        Self::new(
            id,
            ObjectType::PowerUp,
            position,
            sprite,
            Box::new(PowerUpBehavior::new(radius, duration))
        )
    }

    pub fn update(&mut self, dt: f32) {
        self.behavior.update(dt);

        if let Some(sprite) = &mut self.sprite {
            sprite.position = self.position;
            sprite.rotation = self.rotation;
        }
    }

    pub fn check_trigger(&mut self, other: &WorldObject) {
        if !self.behavior.is_trigger() {
            return;
        }

        if let Some(radius) = self.behavior.trigger_radius() {
            let distance = (other.position - self.position).length();
            if distance <= radius {
                self.behavior.on_trigger(other);
            }
        }
    }

    // Get reference to the behavior for type-specific operations
    pub fn behavior(&self) -> &dyn WorldBehavior {
        &*self.behavior
    }

    // Get mutable reference to the behavior
    pub fn behavior_mut(&mut self) -> &mut dyn WorldBehavior {
        &mut *self.behavior
    }
}

// Example of how to implement a custom behavior
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