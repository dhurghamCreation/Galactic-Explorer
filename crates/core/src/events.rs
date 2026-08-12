//! ECS Events for inter-system communication.
//! Events allow systems to communicate without tight coupling,
//! following a publish-subscribe pattern.

use crate::PlanetKind;
use bevy::prelude::*;

/// Fired when a planet is discovered.
#[derive(Event)]
pub struct PlanetDiscovered {
    pub kind: PlanetKind,
}

/// Fired when the player attempts a landing.
#[derive(Event)]
pub struct LandingAttempt {
    pub target: PlanetKind,
    pub success: bool,
}

/// Fired when an asteroid hits the ship.
#[derive(Event)]
pub struct AsteroidHit {
    pub damage: i32,
}

/// Fired when the player's ship is destroyed.
#[derive(Event)]
pub struct ShipDestroyed;

/// Fired when a mission objective is completed.
#[derive(Event)]
pub struct ObjectiveCompleted {
    pub id: u32,
    pub title: String,
    pub reward_xp: u32,
}

/// Fired when the player levels up (tier increases).
#[derive(Event)]
pub struct TierUp {
    pub new_tier: u32,
}

/// Fired when fuel is depleted.
#[derive(Event)]
pub struct FuelDepleted;

/// Fired when a challenge milestone is reached.
#[derive(Event)]
pub struct ChallengeProgress {
    pub name: String,
    pub progress: u32,
    pub goal: u32,
}
