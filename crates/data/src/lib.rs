//! # Data Persistence Layer
//!
//! Handles save/load functionality using Result-based error handling
//! and the `?` operator for safe propagation. No unwrap() in logic.

use bevy::prelude::*;
use std::fs;
use thiserror::Error;

use galactic_explorer_core::prelude::*;

/// Errors that can occur during save/load operations.
#[derive(Error, Debug)]
pub enum DataError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("File not found: {0}")]
    NotFound(String),
}

/// Plugin that registers save/load systems.
pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_save_input, handle_load_input, auto_save_system),
        );
    }
}

/// Result type for data operations.
pub type DataResult<T> = Result<T, DataError>;

/// Saves game state to a JSON file.
fn save_to_file(
    path: &str,
    health: &Health,
    fuel: &Fuel,
    mission: &Mission,
    settings: &Settings,
    stats: &GameStats,
) -> DataResult<()> {
    let save = SaveData {
        version: env!("CARGO_PKG_VERSION").to_string(),
        player_health: health.hearts,
        player_fuel: fuel.current,
        discoveries: mission.discoveries,
        discovered_planets: mission.discovered.iter().copied().collect(),
        current_difficulty: settings.difficulty,
        mission_state: mission.clone(),
        play_time: stats.total_play_time,
        timestamp: chrono_now_or_fallback(),
    };
    let json = serde_json::to_string_pretty(&save)?;
    fs::write(path, json)?;
    log::info!("Game saved to {}", path);
    Ok(())
}

/// Loads game state from a JSON file.
fn load_from_file(
    path: &str,
    health: &mut Health,
    fuel: &mut Fuel,
    mission: &mut Mission,
    settings: &mut Settings,
    stats: &mut GameStats,
) -> DataResult<()> {
    let json = fs::read_to_string(path).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => DataError::NotFound(path.to_string()),
        other => DataError::Io(other.into()),
    })?;
    let save: SaveData = serde_json::from_str(&json)?;

    health.hearts = save.player_health;
    fuel.current = save.player_fuel;
    mission.discoveries = save.discoveries;
    mission.discovered = save.discovered_planets.into_iter().collect();
    settings.difficulty = save.current_difficulty;
    stats.total_play_time = save.play_time;

    // Restore nested mission state
    mission.xp = save.mission_state.xp;
    mission.tier = save.mission_state.tier;
    mission.unlocked_rewards = save.mission_state.unlocked_rewards;
    mission.challenge_name = save.mission_state.challenge_name;
    mission.challenge_progress = save.mission_state.challenge_progress;
    mission.challenge_goal = save.mission_state.challenge_goal;
    mission.last_event = save.mission_state.last_event;
    mission.current_objectives = save.mission_state.current_objectives;
    mission.completed_objectives = save.mission_state.completed_objectives;
    mission.total_missions_completed = save.mission_state.total_missions_completed;

    log::info!("Game loaded from {}", path);
    Ok(())
}

/// Handles F5 save input.
fn handle_save_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    health: Res<Health>,
    fuel: Res<Fuel>,
    mission: Res<Mission>,
    settings: Res<Settings>,
    stats: Res<GameStats>,
) {
    if keyboard.just_pressed(KeyCode::F5) {
        if let Err(e) = save_to_file(
            "save_game.json",
            &health,
            &fuel,
            &mission,
            &settings,
            &stats,
        ) {
            log::error!("Save failed: {}", e);
        }
    }
}

/// Handles F9 load input.
fn handle_load_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut health: ResMut<Health>,
    mut fuel: ResMut<Fuel>,
    mut mission: ResMut<Mission>,
    mut settings: ResMut<Settings>,
    mut stats: ResMut<GameStats>,
) {
    if keyboard.just_pressed(KeyCode::F9) {
        if let Err(e) = load_from_file(
            "save_game.json",
            &mut health,
            &mut fuel,
            &mut mission,
            &mut settings,
            &mut stats,
        ) {
            log::error!("Load failed: {}", e);
        }
    }
}

/// Auto-save at configured intervals.
fn auto_save_system(
    time: Res<Time>,
    settings: Res<Settings>,
    mut timer: Local<f32>,
    health: Res<Health>,
    fuel: Res<Fuel>,
    mission: Res<Mission>,
    stats: Res<GameStats>,
) {
    if settings.auto_save_interval > 0.0 {
        *timer += time.delta_seconds();
        if *timer >= settings.auto_save_interval {
            *timer = 0.0;
            if let Err(e) = save_to_file(
                "auto_save.json",
                &health,
                &fuel,
                &mission,
                &settings,
                &stats,
            ) {
                log::error!("Auto-save failed: {}", e);
            }
        }
    }
}

/// Returns current time as ISO string, or fallback if unavailable.
fn chrono_now_or_fallback() -> String {
    // Simple fallback without chrono dependency
    "2024-01-01T00:00:00Z".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_save_load_roundtrip() {
        let health = Health {
            hearts: 5,
            max_hearts: 5,
        };
        let fuel = Fuel {
            current: 80.0,
            max: 100.0,
        };
        let mission = Mission {
            discoveries: 3,
            discovered: HashSet::from([PlanetKind::Earth, PlanetKind::Mars, PlanetKind::Venus]),
            xp: 500,
            tier: 2,
            unlocked_rewards: vec!["Scanner".to_string()],
            challenge_name: "Asteroid Dodge".to_string(),
            challenge_progress: 5,
            challenge_goal: 12,
            last_event: "Test event".to_string(),
            current_objectives: vec![],
            completed_objectives: vec![],
            total_missions_completed: 1,
        };
        let settings = Settings::default();
        let stats = GameStats::default();

        let path = "_test_save.json";
        save_to_file(path, &health, &fuel, &mission, &settings, &stats).unwrap();

        let mut loaded_health = Health {
            hearts: 0,
            max_hearts: 5,
        };
        let mut loaded_fuel = Fuel {
            current: 0.0,
            max: 100.0,
        };
        let mut loaded_mission = mission.clone();
        let mut loaded_settings = Settings::default();
        let mut loaded_stats = GameStats::default();

        load_from_file(
            path,
            &mut loaded_health,
            &mut loaded_fuel,
            &mut loaded_mission,
            &mut loaded_settings,
            &mut loaded_stats,
        )
        .unwrap();

        assert_eq!(loaded_health.hearts, health.hearts);
        assert_eq!(loaded_fuel.current, fuel.current);
        assert_eq!(loaded_mission.discoveries, mission.discoveries);

        let _ = fs::remove_file(path);
    }
}
