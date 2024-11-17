mod object;
mod behavior;
mod checkpoint;
mod power_up;

pub use checkpoint::CheckpointBehavior;
pub use power_up::PowerUpBehavior;
pub use object::WorldObject;
pub use behavior::{EmptyBehavior, WorldBehavior};
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObjectType {
    Checkpoint,
    PowerUp,
    Decoration,
}