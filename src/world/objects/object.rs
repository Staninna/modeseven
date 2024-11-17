use crate::physics::Vec2;
use crate::rendering::Sprite;
use crate::world::objects::behavior::{EmptyBehavior, WorldBehavior};
use crate::world::objects::{CheckpointBehavior, ObjectType, PowerUpBehavior};

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