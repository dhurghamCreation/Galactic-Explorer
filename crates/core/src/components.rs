//! ECS Components - pure data structs following Data-Oriented Design.
//! Components are flat data structures that systems operate on in bulk,
//! enabling cache-efficient iteration patterns.

use bevy::prelude::*;

/// A celestial body with rotation behavior.
#[derive(Component)]
pub struct Rotating {
    pub speed: f32,
}

/// A celestial body with orbital motion around a central point.
#[derive(Component)]
pub struct Orbiting {
    pub radius: f32,
    pub speed: f32,
    pub phase: f32,
}

/// Marker component for the player's ship.
#[derive(Component)]
pub struct PlayerShip;

/// A celestial body entity.
#[derive(Component)]
pub struct CelestialBody {
    pub kind: crate::PlanetKind,
}

/// Visual representation data for a celestial body.
#[derive(Component)]
pub struct Visual {
    pub radius: f32,
}

/// Marker for Saturn's ring entity.
#[derive(Component)]
pub struct SaturnRing;

/// Marker for the star dome skybox.
#[derive(Component)]
pub struct StarDome;

/// Individual star in the starfield, with twinkle animation data.
#[derive(Component)]
pub struct Star {
    pub phase: f32,
    pub amplitude: f32,
}

/// An asteroid hazard entity.
#[derive(Component)]
pub struct Asteroid {
    pub velocity: Vec3,
}

/// Marker for the chase camera entity.
#[derive(Component)]
pub struct ChaseCamera;

/// Marker for the cockpit camera entity.
#[derive(Component)]
pub struct CockpitCamera;

/// Camera settings for the chase/follow camera.
#[derive(Component)]
pub struct FollowCamera {
    pub distance: f32,
    pub height: f32,
}

/// UI component markers
#[derive(Component)]
pub struct HudRoot;
#[derive(Component)]
pub struct MenuRoot;
#[derive(Component)]
pub struct HudText;
#[derive(Component)]
pub struct HeaderText;
#[derive(Component)]
pub struct FocusInfo;
#[derive(Component)]
pub struct MenuTitle;
#[derive(Component)]
pub struct MenuBody;
#[derive(Component)]
pub struct LoadingBar;
#[derive(Component)]
pub struct ControlsRoot;
#[derive(Component)]
pub struct TouchButton(pub crate::TouchAction);
#[derive(Component)]
pub struct MinimapText;
#[derive(Component)]
pub struct ProgressionText;
#[derive(Component)]
pub struct SettingsButtonRoot;
#[derive(Component)]
pub struct BackButtonRoot;
#[derive(Component)]
pub struct LearningContentRoot;
#[derive(Component)]
pub struct CombatContentRoot;
