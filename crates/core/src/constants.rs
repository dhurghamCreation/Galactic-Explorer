//! Game constants and configuration values.
//! Centralizing magic numbers improves maintainability.

/// Bounds for the playable area.
pub const WORLD_BOUNDARY: f32 = 760.0;
pub const WORLD_BOUNDARY_CHALLENGE: f32 = 860.0;

/// Camera constraints.
pub const CAMERA_MIN_DISTANCE: f32 = 10.0;
pub const CAMERA_MAX_DISTANCE: f32 = 420.0;
pub const CAMERA_ZOOM_SPEED: f32 = 14.0;
pub const CAMERA_FOLLOW_LERP: f32 = 3.0;

/// Ship physics.
pub const SHIP_BASE_SPEED: f32 = 20.0;
pub const SHIP_BOOST_SPEED: f32 = 36.0;
pub const SHIP_YAW_SPEED: f32 = 1.4;
pub const SHIP_COLLISION_RADIUS: f32 = 3.5;
pub const SHIP_LANDING_SPEED_THRESHOLD: f32 = 12.0;
pub const SHIP_LANDING_DISTANCE: f32 = 1.3;
pub const SHIP_CRASH_DISTANCE: f32 = 2.8;

/// Scanner system.
pub const SCANNER_ACTIVE_RATE: f32 = 0.2;
pub const SCANNER_PASSIVE_RATE: f32 = -0.14;

/// Asteroid hazards.
pub const ASTEROID_DESPAWN_RADIUS: f32 = 260.0;
pub const ASTEROID_MAX_WORLD_RADIUS: f32 = 1200.0;

/// Starfield.
pub const STAR_COUNT: usize = 650;
pub const STAR_MIN_DISTANCE: f32 = 160.0;
pub const STAR_MAX_DISTANCE: f32 = 260.0;
pub const STAR_TWINKLE_SPEED: f32 = 2.3;
pub const STAR_MIN_SCALE: f32 = 0.5;

/// UI layout constants.
pub const HUD_HEADER_SIZE: f32 = 27.0;
pub const HUD_TEXT_SIZE: f32 = 18.0;
pub const HUD_FOCUS_SIZE: f32 = 16.0;
pub const HUD_MINIMAP_SIZE: f32 = 14.0;
pub const MENU_TITLE_SIZE: f32 = 46.0;
pub const MENU_BODY_SIZE: f32 = 22.0;

/// Loading screen.
pub const LOADING_DURATION: f32 = 3.0;

/// Player defaults.
pub const DEFAULT_HEARTS: i32 = 5;
pub const MAX_HEARTS: i32 = 5;
pub const DEFAULT_FUEL: f32 = 100.0;
pub const MAX_FUEL: f32 = 100.0;
pub const FUEL_WARNING_THRESHOLD: f32 = 20.0;
pub const FUEL_CONSUMPTION_SPEED_THRESHOLD: f32 = 0.1;

/// Respawn.
pub const RESPAWN_DELAY: f32 = 3.0;
pub const RESPAWN_POSITION: (f32, f32, f32) = (0.0, 12.0, 45.0);

/// Exploration points.
pub const DISCOVERY_XP_REWARD: u32 = 100;
pub const CHALLENGE_XP_REWARD: u32 = 200;
pub const XP_PER_TIER: u32 = 1000;
pub const XP_PER_LEVEL: u32 = 300;

/// Ship warp/assist distances.
pub const WARP_DISTANCE: f32 = 18.0;
pub const ASSIST_STANDOFF: f32 = 2.2;
pub const LANDING_ASSIST_RANGE: f32 = 22.0;

/// Lighting.
pub const SUN_ILLUMINANCE: f32 = 18000.0;
pub const FILL_ILLUMINANCE: f32 = 5000.0;
pub const SUN_LIGHT_INTENSITY: f32 = 9500.0;
pub const SUN_LIGHT_RANGE: f32 = 260.0;

/// Ambient.
pub const AMBIENT_RED: f32 = 0.22;
pub const AMBIENT_GREEN: f32 = 0.28;
pub const AMBIENT_BLUE: f32 = 0.38;
pub const AMBIENT_BRIGHTNESS: f32 = 0.6;

/// Ship start.
pub const SHIP_START_X: f32 = 0.0;
pub const SHIP_START_Y: f32 = 15.0;
pub const SHIP_START_Z: f32 = 80.0;
pub const SHIP_START_SCALE: f32 = 0.4;
