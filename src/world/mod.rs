//! Game world state and physics simulation
//!
//! Manages the game's physical world including car physics,
//! object positioning, and state updates. Handles all dynamic
//! object interactions and maintains the game's physical state.

pub use car::{Car, CarInput};
pub use world::World;

mod car;
mod world;
