//! # Galactic Explorer Core
//!
//! This crate defines the foundational ECS types for the Galactic Explorer engine.
//! It follows Data-Oriented Design principles: components are pure data,
//! systems operate on component bundles, and resources manage global state.
//!
//! ## Architecture
//!
//! - **Components**: Pure data structs with `#[derive(Component)]`
//! - **Resources**: Global state managed via Bevy's ECS resource system
//! - **Events**: Inter-system communication via Bevy events
//! - **Enums**: Type-safe planet kinds, difficulty levels, etc.

pub mod components;
pub mod constants;
pub mod enums;
pub mod events;
pub mod planets_section;
pub mod prelude;
pub mod resources;
pub mod traits;

// Re-export commonly used types at crate level
pub use components::*;
pub use constants::*;
pub use enums::*;
pub use events::*;
pub use planets_section::*;
pub use resources::*;
pub use traits::*;
