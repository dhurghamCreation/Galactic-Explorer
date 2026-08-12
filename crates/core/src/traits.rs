//! Shared traits for the Galactic Explorer engine.
//! Traits define behavior contracts that can be implemented by any type.

use crate::enums::GameDifficulty;

/// Types that can provide difficulty-scaled values.
pub trait DifficultyScaled {
    type Output;
    fn for_difficulty(&self, difficulty: GameDifficulty) -> Self::Output;
}

/// Types that can be reset to a default state.
pub trait Resettable {
    fn reset(&mut self);
}

/// Types that can provide a display label.
pub trait Labeled {
    fn label(&self) -> &'static str;
}
