// In src/world/world.rs

use std::collections::HashMap;
use crate::world::objects::{ObjectType, WorldBehavior, WorldObject};
use crate::rendering::{Sprite, SpriteManager};
use crate::physics::Vec2;

pub struct World {
    objects: HashMap<u64, WorldObject>,
    next_id: u64,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            next_id: 1,
        }
    }

    // Private method to generate next ID
    fn generate_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    // Add methods that create and add objects in one go
    pub fn add_checkpoint(&mut self, position: Vec2, sprite: Option<Sprite>, radius: f32) -> u64 {
        let id = self.generate_id();
        let object = WorldObject::checkpoint(id, position, sprite, radius);
        self.objects.insert(id, object);
        id
    }

    pub fn add_decoration(&mut self, position: Vec2, sprite: Sprite) -> u64 {
        let id = self.generate_id();
        let object = WorldObject::decoration(id, position, sprite);
        self.objects.insert(id, object);
        id
    }

    pub fn add_power_up(&mut self, position: Vec2, sprite: Option<Sprite>, radius: f32, duration: f32) -> u64 {
        let id = self.generate_id();
        let object = WorldObject::power_up(id, position, sprite, radius, duration);
        self.objects.insert(id, object);
        id
    }

    // Generic method to add any custom object
    pub fn add_object_type(&mut self,
                           object_type: ObjectType,
                           position: Vec2,
                           sprite: Option<Sprite>,
                           behavior: Box<dyn WorldBehavior>
    ) -> u64 {
        let id = self.generate_id();
        let object = WorldObject::new(id, object_type, position, sprite, behavior);
        self.objects.insert(id, object);
        id
    }

    // Remove object from world
    pub fn remove_object(&mut self, id: u64) -> Option<WorldObject> {
        self.objects.remove(&id)
    }

    // Get object by ID
    pub fn get_object(&self, id: u64) -> Option<&WorldObject> {
        self.objects.get(&id)
    }

    // Get mutable object by ID
    pub fn get_object_mut(&mut self, id: u64) -> Option<&mut WorldObject> {
        self.objects.get_mut(&id)
    }

    // Update all objects and process triggers
    pub fn update(&mut self, dt: f32) {
        let mut trigger_pairs = Vec::new();
        let object_ids: Vec<u64> = self.objects.keys().copied().collect();

        // Find all objects that can trigger and their potential targets
        for &id in &object_ids {
            if let Some(object) = self.objects.get(&id) {
                if object.behavior().is_trigger() {
                    for &other_id in &object_ids {
                        if id != other_id {
                            trigger_pairs.push((id, other_id));
                        }
                    }
                }
            }
        }

        // Do regular updates
        for object in self.objects.values_mut() {
            if object.active {
                object.update(dt);
            }
        }

        // Process all trigger checks
        for (trigger_id, other_id) in trigger_pairs {
            let other = if let Some(other) = self.objects.get(&other_id).cloned() {
                other
            } else {
                continue;
            };

            if let Some(trigger_obj) = self.objects.get_mut(&trigger_id) {
                trigger_obj.check_trigger(&other);
            }
        }
    }

    // Update sprite manager
    pub fn update_sprites(&self, sprite_manager: &mut SpriteManager) {
        sprite_manager.clear_sprites();

        for object in self.objects.values() {
            if object.active {
                if let Some(sprite) = &object.sprite {
                    sprite_manager.add_sprite(sprite.clone());
                }
            }
        }
    }

    // Utility methods
    pub fn find_by_type(&self, object_type: ObjectType) -> Vec<&WorldObject> {
        self.objects
            .values()
            .filter(|obj| obj.object_type == object_type)
            .collect()
    }

    pub fn find_in_radius(&self, center: Vec2, radius: f32) -> Vec<&WorldObject> {
        self.objects
            .values()
            .filter(|obj| {
                let dx = obj.position.x - center.x;
                let dy = obj.position.y - center.y;
                (dx * dx + dy * dy).sqrt() <= radius
            })
            .collect()
    }

    pub fn cleanup_inactive(&mut self) {
        self.objects.retain(|_, obj| obj.active);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    pub fn count_by_type(&self, object_type: ObjectType) -> usize {
        self.objects
            .values()
            .filter(|obj| obj.object_type == object_type)
            .count()
    }
}
