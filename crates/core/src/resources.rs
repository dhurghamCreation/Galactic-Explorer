//! Global ECS Resources - shared state for game systems.
//! Resources are singleton data stores that any system can access.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::enums::*;

/// The currently targeted celestial body.
#[derive(Resource, Clone, Copy)]
pub struct Target {
    pub target: crate::PlanetKind,
}

/// Scanner system state.
#[derive(Resource)]
pub struct Scanner {
    pub progress: f32,
    pub active: bool,
}

/// Player's ship flight state.
#[derive(Resource)]
pub struct Flight {
    pub speed: f32,
    pub destroyed: bool,
    pub respawn_timer: Timer,
}

/// Player health tracking.
#[derive(Resource)]
pub struct Health {
    pub hearts: i32,
    pub max_hearts: i32,
}

/// Player fuel tracking.
#[derive(Resource)]
pub struct Fuel {
    pub current: f32,
    pub max: f32,
}

/// Asteroid hazard spawner state.
#[derive(Resource)]
pub struct Hazards {
    pub spawn_timer: Timer,
}

/// Mission progress and discovery tracking.
#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct Mission {
    pub discoveries: u32,
    pub discovered: HashSet<crate::PlanetKind>,
    pub xp: u32,
    pub tier: u32,
    pub unlocked_rewards: Vec<String>,
    pub challenge_name: String,
    pub challenge_progress: u32,
    pub challenge_goal: u32,
    pub last_event: String,
    pub current_objectives: Vec<Objective>,
    pub completed_objectives: Vec<Objective>,
    pub total_missions_completed: u32,
}

/// A single mission objective.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Objective {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub target: Option<crate::PlanetKind>,
    pub progress: u32,
    pub goal: u32,
    pub completed: bool,
    pub reward_xp: u32,
    pub reward_unlock: Option<String>,
}

/// Application flow state (screen navigation).
#[derive(Resource)]
pub struct Flow {
    pub screen: ScreenMode,
    pub loading_progress: f32,
}

impl Flow {
    pub fn is_playing(&self) -> bool {
        self.screen == ScreenMode::Playing
    }
}

/// Planet info displayed on click inspection.
#[derive(Resource, Default)]
pub struct FocusedInfo {
    pub message: String,
}

/// Game settings - all configurable options.
#[derive(Resource)]
pub struct Settings {
    pub simulation_speed: f32,
    pub ship_scale: f32,
    pub show_touch_controls: bool,
    pub difficulty: GameDifficulty,
    pub auto_landing_assist: bool,
    pub challenge_mode: bool,
    pub input_sensitivity: f32,
    pub graphics_quality: GraphicsQuality,
    pub star_density: f32,
    pub planet_detail: PlanetDetail,
    pub camera_smoothing: f32,
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub engine_sound: bool,
    pub tutorial_hints: bool,
    pub crosshair_enabled: bool,
    pub minimap_enabled: bool,
    pub auto_save_interval: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            simulation_speed: 1.0,
            ship_scale: 0.2,
            show_touch_controls: true,
            difficulty: GameDifficulty::Medium,
            auto_landing_assist: true,
            challenge_mode: false,
            input_sensitivity: 1.0,
            graphics_quality: GraphicsQuality::High,
            star_density: 1.0,
            planet_detail: PlanetDetail::Standard,
            camera_smoothing: 0.8,
            master_volume: 0.8,
            music_volume: 0.7,
            sfx_volume: 0.9,
            engine_sound: true,
            tutorial_hints: true,
            crosshair_enabled: true,
            minimap_enabled: true,
            auto_save_interval: 60.0,
        }
    }
}

/// Persistent game statistics.
#[derive(Resource, Default)]
pub struct GameStats {
    pub total_play_time: f32,
    pub missions_completed: u32,
    pub total_discoveries: u32,
    pub highest_difficulty: GameDifficulty,
}

/// Search state for planet lookup.
#[derive(Resource, Default)]
pub struct Search {
    pub active: bool,
    pub query: String,
}

/// Background music state.
#[derive(Resource, Default)]
pub struct Music {
    pub started: bool,
}

/// Virtual/touch control states.
#[derive(Resource, Default)]
pub struct VirtualControls {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub yaw_left: bool,
    pub yaw_right: bool,
    pub boost: bool,
}

/// Camera mode state.
#[derive(Resource, Clone, Copy, PartialEq, Eq)]
pub struct CameraState {
    pub cockpit: bool,
    pub overview: bool,
}

/// Camera overview settings.
#[derive(Resource)]
pub struct CameraOverviewSettings {
    pub distance: f32,
    pub height: f32,
}

impl Default for CameraOverviewSettings {
    fn default() -> Self {
        Self {
            distance: 56.0,
            height: 14.0,
        }
    }
}

/// Quiz system state for the Learning section (Planets Quiz).
#[derive(Resource)]
pub struct QuizState {
    pub questions: Vec<crate::QuizQuestion>,
    pub current_question: usize,
    pub score: u32,
    pub total_questions: usize,
    pub answered: bool,
    pub selected_index: Option<usize>,
    pub showing_explanation: bool,
    pub quiz_finished: bool,
    pub correct_count: u32,
    pub incorrect_count: u32,
}

impl Default for QuizState {
    fn default() -> Self {
        let all_questions = crate::get_all_planet_questions();
        let total = all_questions.len();
        Self {
            questions: all_questions,
            current_question: 0,
            score: 0,
            total_questions: total,
            answered: false,
            selected_index: None,
            showing_explanation: false,
            quiz_finished: false,
            correct_count: 0,
            incorrect_count: 0,
        }
    }
}

/// Challenge mode state for the Combat/Challenges section.
#[derive(Resource, Default)]
pub struct ChallengeState {
    pub active: bool,
    pub current_challenge: u32,
    pub challenge_name: String,
    pub challenge_description: String,
    pub progress: u32,
    pub goal: u32,
    pub completed: bool,
    pub reward_earned: bool,
    pub challenge_log: Vec<String>,
    pub challenge_index: usize,
}

/// Defines challenges available in the Combat section.
#[derive(Clone, Debug)]
pub struct Challenge {
    pub id: u32,
    pub name: &'static str,
    pub description: &'static str,
    pub objective: &'static str,
    pub reward_xp: u32,
    pub difficulty: &'static str,
}

/// Returns all available challenges.
pub fn get_all_challenges() -> Vec<Challenge> {
    vec![
        Challenge {
            id: 1,
            name: "🏆 Asteroid Field Navigation",
            description: "Navigate through a dense asteroid field without taking damage.",
            objective: "Dodge 20 asteroids successfully",
            reward_xp: 200,
            difficulty: "Medium",
        },
        Challenge {
            id: 2,
            name: "⚡ Speed Run: Mercury Loop",
            description: "Complete a full orbit around Mercury at top speed.",
            objective: "Reach 150% max speed for 10 seconds",
            reward_xp: 300,
            difficulty: "Hard",
        },
        Challenge {
            id: 3,
            name: "🛡️ Radiation Belt Survival",
            description: "Survive Jupiter's radiation belts for 60 seconds.",
            objective: "Survive with 3+ hearts remaining",
            reward_xp: 400,
            difficulty: "Hard",
        },
        Challenge {
            id: 4,
            name: "🎯 Precision Landing: Venus",
            description: "Land on Venus' surface within the designated zone.",
            objective: "Land with speed under 5 m/s",
            reward_xp: 250,
            difficulty: "Medium",
        },
        Challenge {
            id: 5,
            name: "🌪️ Saturn Storm Runner",
            description: "Fly through Saturn's hexagonal storm system.",
            objective: "Navigate the storm for 30 seconds",
            reward_xp: 500,
            difficulty: "Expert",
        },
        Challenge {
            id: 6,
            name: "🔭 Discovery Rush",
            description: "Scan and discover all planets in record time.",
            objective: "Discover 5 planets within 2 minutes",
            reward_xp: 350,
            difficulty: "Hard",
        },
        Challenge {
            id: 7,
            name: "💫 Gravity Assist Mastery",
            description: "Use planetary gravity assists to reach Neptune.",
            objective: "Reach Neptune using 3 gravity assists",
            reward_xp: 600,
            difficulty: "Expert",
        },
        Challenge {
            id: 8,
            name: "🌑 Dark Side Explorer",
            description: "Explore the dark side of the Moon without lights.",
            objective: "Navigate 500m in complete darkness",
            reward_xp: 150,
            difficulty: "Easy",
        },
        Challenge {
            id: 9,
            name: "🚀 Rescue Mission",
            description: "Rescue stranded astronauts from Mars orbit.",
            objective: "Pick up 5 astronaut pods",
            reward_xp: 300,
            difficulty: "Medium",
        },
        Challenge {
            id: 10,
            name: "🌟 Ultimate Explorer",
            description: "Visit every planet in the solar system in one flight.",
            objective: "Visit all 8 planets without crashing",
            reward_xp: 1000,
            difficulty: "Expert",
        },
    ]
}

/// Save data for serialization.
#[derive(Serialize, Deserialize, Clone)]
pub struct SaveData {
    pub version: String,
    pub player_health: i32,
    pub player_fuel: f32,
    pub discoveries: u32,
    pub discovered_planets: Vec<crate::PlanetKind>,
    pub current_difficulty: GameDifficulty,
    pub mission_state: Mission,
    pub play_time: f32,
    pub timestamp: String,
}
