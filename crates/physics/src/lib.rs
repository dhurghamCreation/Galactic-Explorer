//! # Physics Engine
//!
//! Handles orbital mechanics, ship flight dynamics, collision detection,
//! and hazard spawning. All systems operate on ECS component data.

use bevy::math::primitives::{Sphere, Torus};
use bevy::prelude::*;
use bevy::render::render_resource::Face;
use galactic_explorer_core::prelude::*;
use rand::Rng;
use std::time::Duration;

/// Plugin that registers physics-related systems.
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_solar_system).add_systems(
            Update,
            (
                rotate_planets,
                orbit_planets,
                update_saturn_ring,
                move_shuttle,
                apply_ship_scale,
                handle_destroyed_ship_respawn,
                spawn_asteroid_hazards,
                move_asteroids,
                asteroid_hit_ship,
                consume_fuel,
                update_scanner,
                highlight_target,
                handle_target_selection,
                inspect_planet_with_mouse,
                update_mission_progress,
            ),
        );
    }
}

/// Spawns the entire solar system at startup.
pub fn spawn_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Initialize resources
    commands.insert_resource(Target {
        target: PlanetKind::Earth,
    });
    commands.insert_resource(Scanner {
        progress: 0.0,
        active: false,
    });
    commands.insert_resource(Mission {
        discoveries: 0,
        discovered: std::collections::HashSet::new(),
        xp: 0,
        tier: 1,
        unlocked_rewards: Vec::new(),
        challenge_name: "Asteroid Dodge".to_string(),
        challenge_progress: 0,
        challenge_goal: 12,
        last_event: "Mission initialized.".to_string(),
        current_objectives: vec![
            Objective {
                id: 1,
                title: "First Discovery".to_string(),
                description: "Scan your first celestial body".to_string(),
                target: None,
                progress: 0,
                goal: 1,
                completed: false,
                reward_xp: 100,
                reward_unlock: Some("Enhanced Scanner".to_string()),
            },
            Objective {
                id: 2,
                title: "Solar System Tour".to_string(),
                description: "Discover all 8 major planets".to_string(),
                target: None,
                progress: 0,
                goal: 8,
                completed: false,
                reward_xp: 500,
                reward_unlock: Some("Fuel Efficiency Upgrade".to_string()),
            },
        ],
        completed_objectives: Vec::new(),
        total_missions_completed: 0,
    });
    commands.insert_resource(Flight {
        speed: 0.0,
        destroyed: false,
        respawn_timer: Timer::from_seconds(RESPAWN_DELAY, TimerMode::Once),
    });
    commands.insert_resource(Health {
        hearts: DEFAULT_HEARTS,
        max_hearts: MAX_HEARTS,
    });
    commands.insert_resource(Fuel {
        current: DEFAULT_FUEL,
        max: MAX_FUEL,
    });
    commands.insert_resource(Hazards {
        spawn_timer: Timer::from_seconds(3.5, TimerMode::Repeating), // REDUCED: was 1.8, now 3.5s
    });
    commands.insert_resource(FocusedInfo::default());
    commands.insert_resource(GameStats::default());

    // Planet data: (kind, position, speed, emissive)
    let planets_data: [(PlanetKind, Vec3, f32, bool); 14] = [
        (PlanetKind::Sun, Vec3::new(0.0, 0.0, 0.0), 0.03, true),
        (PlanetKind::Mercury, Vec3::new(19.0, 0.0, -6.0), 0.31, false),
        (PlanetKind::Venus, Vec3::new(27.0, 1.0, 8.0), 0.24, false),
        (PlanetKind::Earth, Vec3::new(36.0, 0.0, 0.0), 0.22, false),
        (PlanetKind::Moon, Vec3::new(40.5, 0.0, 3.0), 0.06, false),
        (PlanetKind::Mars, Vec3::new(50.0, 0.0, 0.0), 0.19, false),
        (PlanetKind::Jupiter, Vec3::new(73.0, 0.0, -4.0), 0.11, false),
        (PlanetKind::Saturn, Vec3::new(96.0, 0.0, 4.0), 0.09, false),
        (PlanetKind::Uranus, Vec3::new(120.0, 0.0, -7.0), 0.08, false),
        (PlanetKind::Neptune, Vec3::new(144.0, 0.0, 8.0), 0.07, false),
        (PlanetKind::Ceres, Vec3::new(42.0, 0.0, 2.0), 0.15, false),
        (PlanetKind::Eris, Vec3::new(168.0, 2.0, -4.0), 0.05, false),
        (PlanetKind::Haumea, Vec3::new(156.0, 1.0, -3.0), 0.06, false),
        (PlanetKind::Makemake, Vec3::new(156.0, 1.0, -3.0), 0.06, false),
    ];

    for (kind, position, speed, emissive) in planets_data {
        spawn_planet_entity(
            &mut commands,
            &mut meshes,
            &mut materials,
            &asset_server,
            kind,
            position,
            speed,
            emissive,
        );
    }

    // Starfield
    spawn_starfield(&mut commands, &mut meshes, &mut materials);

    // Star dome skybox
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere { radius: 900.0 }),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("textures/stars.png")),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(-1.0)),
            ..default()
        },
        StarDome,
    ));

    // Player ship
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/shuttle.glb#Scene0"),
            transform: Transform {
                translation: Vec3::new(SHIP_START_X, SHIP_START_Y, SHIP_START_Z),
                scale: Vec3::splat(SHIP_START_SCALE),
                rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_4),
                ..default()
            },
            visibility: Visibility::Visible,
            ..default()
        },
        PlayerShip,
    ));

    // Saturn ring
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Torus {
                major_radius: 8.2,
                minor_radius: 0.55,
            }),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("textures/saturnring.png")),
                alpha_mode: AlphaMode::Blend,
                double_sided: true,
                cull_mode: Some(Face::Back),
                unlit: true,
                ..default()
            }),
            transform: Transform::from_xyz(96.0, 0.0, 4.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ..default()
        },
        SaturnRing,
    ));
}

fn spawn_planet_entity(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
    kind: PlanetKind,
    position: Vec3,
    speed: f32,
    emissive_star: bool,
) {
    let mut material = StandardMaterial {
        base_color_texture: Some(asset_server.load(kind.texture_path())),
        perceptual_roughness: 0.85,
        ..default()
    };
    if emissive_star {
        material.emissive = Color::rgb(0.68, 0.42, 0.12);
    }

    let orbit_radius = position.xz().length();
    let phase = position.z.atan2(position.x);

    let mut entity = commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere {
                radius: kind.radius(),
            }),
            material: materials.add(material),
            transform: Transform::from_translation(position),
            ..default()
        },
        CelestialBody { kind },
        Visual {
            radius: kind.radius(),
        },
        Rotating { speed },
    ));

    if kind != PlanetKind::Sun {
        entity.insert(Orbiting {
            radius: orbit_radius,
            speed: speed * 0.6,
            phase,
        });
    }
}

fn spawn_starfield(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let star_mesh = meshes.add(Sphere { radius: 0.11 });
    let star_material = materials.add(StandardMaterial {
        emissive: Color::rgb(0.85, 0.9, 1.0),
        unlit: true,
        ..default()
    });
    let mut rng = rand::thread_rng();
    for _ in 0..STAR_COUNT {
        let dir = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        )
        .normalize_or_zero();
        let distance = rng.gen_range(STAR_MIN_DISTANCE..STAR_MAX_DISTANCE);
        commands.spawn((
            PbrBundle {
                mesh: star_mesh.clone(),
                material: star_material.clone(),
                transform: Transform::from_translation(dir * distance),
                ..default()
            },
            Star {
                phase: rng.gen_range(0.0..std::f32::consts::TAU),
                amplitude: rng.gen_range(0.08..0.33),
            },
        ));
    }
}

/// Rotate planets on their axes.
pub fn rotate_planets(
    time: Res<Time>,
    settings: Res<Settings>,
    mut query: Query<(&mut Transform, &Rotating)>,
) {
    for (mut transform, body) in &mut query {
        transform.rotate_y(body.speed * settings.simulation_speed * time.delta_seconds());
    }
}

/// Move planets along their orbits.
pub fn orbit_planets(
    time: Res<Time>,
    settings: Res<Settings>,
    mut query: Query<(&mut Transform, &Orbiting)>,
) {
    let t = time.elapsed_seconds() * settings.simulation_speed;
    for (mut transform, orbit) in &mut query {
        let angle = t * orbit.speed + orbit.phase;
        transform.translation =
            Vec3::new(orbit.radius * angle.cos(), 0.0, orbit.radius * angle.sin());
    }
}

/// Keep Saturn's ring aligned with Saturn.
pub fn update_saturn_ring(
    saturn_query: Query<(&Transform, &CelestialBody), Without<SaturnRing>>,
    mut ring_query: Query<&mut Transform, With<SaturnRing>>,
) {
    let mut saturn_pos = None;
    for (transform, body) in &saturn_query {
        if body.kind == PlanetKind::Saturn {
            saturn_pos = Some(transform.translation);
            break;
        }
    }
    if let Some(position) = saturn_pos {
        for mut ring in &mut ring_query {
            ring.translation = position;
        }
    }
}

/// Target selection via keyboard.
pub fn handle_target_selection(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut target: ResMut<Target>,
    mut mission: ResMut<Mission>,
) {
    if keyboard.just_pressed(KeyCode::Digit1) {
        target.target = PlanetKind::Earth;
        mission.last_event = "Target: Earth".into();
    }
    if keyboard.just_pressed(KeyCode::Digit2) || keyboard.just_pressed(KeyCode::KeyM) {
        target.target = PlanetKind::Mars;
        mission.last_event = "Target: Mars".into();
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        target.target = PlanetKind::Jupiter;
        mission.last_event = "Target: Jupiter".into();
    }
    if keyboard.just_pressed(KeyCode::Digit4) {
        target.target = PlanetKind::Saturn;
        mission.last_event = "Target: Saturn".into();
    }
    if keyboard.just_pressed(KeyCode::Digit5) {
        target.target = PlanetKind::Neptune;
        mission.last_event = "Target: Neptune".into();
    }
    if keyboard.just_pressed(KeyCode::Tab) {
        let mut idx = PlanetKind::ALL
            .iter()
            .position(|&k| k == target.target)
            .unwrap_or(0);
        idx = (idx + 1) % PlanetKind::ALL.len();
        target.target = PlanetKind::ALL[idx];
        mission.last_event = format!("Target: {}", target.target.display_name());
    }
}

/// Highlight target planet with emissive glow.
pub fn highlight_target(
    target: Res<Target>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    planets: Query<(&CelestialBody, &Handle<StandardMaterial>)>,
) {
    for (body, material_handle) in &planets {
        if let Some(material) = materials.get_mut(material_handle) {
            let selected = body.kind == target.target;
            if body.kind == PlanetKind::Sun {
                material.emissive = Color::rgb(0.68, 0.42, 0.12);
            } else {
                material.emissive = if selected {
                    Color::rgb(0.12, 0.14, 0.24)
                } else {
                    Color::BLACK
                };
            }
        }
    }
}

/// Player ship movement system.
pub fn move_shuttle(
    flow: Res<Flow>,
    settings: Res<Settings>,
    virtual_controls: Res<VirtualControls>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut mission: ResMut<Mission>,
    mut focus: ResMut<FocusedInfo>,
    mut flight: ResMut<Flight>,
    mut health: ResMut<Health>,
    mut ship_query: Query<&mut Transform, With<PlayerShip>>,
    planet_query: Query<(&Transform, &CelestialBody, &Visual), Without<PlayerShip>>,
    target: Res<Target>,
    mut planet_hit_cooldown: Local<f32>,
) {
    if !flow.is_playing() {
        return;
    }
    let Ok(mut transform) = ship_query.get_single_mut() else {
        return;
    };
    if flight.destroyed {
        return;
    }

    let mut direction = Vec3::ZERO;
    let mut speed = SHIP_BASE_SPEED * settings.input_sensitivity;
    if keyboard.pressed(KeyCode::ShiftLeft) || virtual_controls.boost {
        speed = SHIP_BOOST_SPEED;
    }
    if keyboard.pressed(KeyCode::KeyW) || virtual_controls.forward {
        direction.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || virtual_controls.backward {
        direction.z += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || virtual_controls.left {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || virtual_controls.right {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyQ) || virtual_controls.down {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyE) || virtual_controls.up {
        direction.y += 1.0;
    }

    let local_move = transform.forward() * -direction.z
        + transform.right() * direction.x
        + Vec3::Y * direction.y;
    let motion = local_move.normalize_or_zero() * speed * time.delta_seconds();
    transform.translation += motion;
    flight.speed = motion.length() / time.delta_seconds().max(0.0001);

    let yaw_speed = SHIP_YAW_SPEED * settings.input_sensitivity * time.delta_seconds();
    if keyboard.pressed(KeyCode::ArrowLeft) || virtual_controls.yaw_left {
        transform.rotate_y(yaw_speed);
    }
    if keyboard.pressed(KeyCode::ArrowRight) || virtual_controls.yaw_right {
        transform.rotate_y(-yaw_speed);
    }

    // G - warp to target
    if keyboard.just_pressed(KeyCode::KeyG) {
        for (planet_transform, body, _) in &planet_query {
            if body.kind == target.target {
                let to_target =
                    (planet_transform.translation - transform.translation).normalize_or_zero();
                transform.translation += to_target * WARP_DISTANCE;
                transform.look_at(planet_transform.translation, Vec3::Y);
                break;
            }
        }
    }

    // K - assist approach
    if keyboard.just_pressed(KeyCode::KeyK) {
        for (planet_transform, body, visual) in &planet_query {
            if body.kind != target.target {
                continue;
            }
            let to_target = planet_transform.translation - transform.translation;
            let stand_off = (visual.radius + ASSIST_STANDOFF).max(3.2);
            transform.translation =
                planet_transform.translation - to_target.normalize_or_zero() * stand_off;
            transform.look_at(planet_transform.translation, Vec3::Y);
            mission.last_event = format!("Assist approach: {}", body.kind.display_name());
            break;
        }
    }

    // R - reset ship
    if keyboard.just_pressed(KeyCode::KeyR) {
        transform.translation = Vec3::new(0.0, 22.0, 220.0);
        transform.rotation = Quat::IDENTITY;
        transform.scale = Vec3::splat(settings.ship_scale);
        flight.destroyed = false;
        mission.last_event = "Ship reset.".to_string();
    }

    // L - attempt landing
    if keyboard.just_pressed(KeyCode::KeyL) {
        for (planet_transform, body, visual) in &planet_query {
            if body.kind != target.target {
                continue;
            }
            let to_ship = transform.translation - planet_transform.translation;
            let distance = to_ship.length();
            let surface_distance = distance - visual.radius;
            let assisted = settings.auto_landing_assist && surface_distance <= LANDING_ASSIST_RANGE;
            if (surface_distance <= SHIP_LANDING_DISTANCE
                && flight.speed <= SHIP_LANDING_SPEED_THRESHOLD)
                || assisted
            {
                let normal = to_ship.normalize_or_zero();
                transform.translation =
                    planet_transform.translation + normal * (visual.radius + 0.55);
                transform.look_at(planet_transform.translation, Vec3::Y);
                mission.discovered.insert(body.kind);
                mission.discoveries = mission.discovered.len() as u32;
                focus.message = format!(
                    "{}\n{}\nDiscovery Complete!",
                    body.kind.display_name(),
                    body.kind.lore()
                );
                apply_mission_progress(&mut mission, body.kind, "Landing");
                if settings.challenge_mode && mission.challenge_name == "Precision Landing" {
                    register_challenge(&mut mission, 1);
                }
            } else if surface_distance <= SHIP_CRASH_DISTANCE {
                flight.destroyed = true;
                flight.respawn_timer.reset();
                mission.last_event = format!("Crash on {}.", body.kind.display_name());
            } else {
                mission.last_event = format!("Too far from {} surface.", body.kind.display_name());
            }
            break;
        }
    }

    // Planet collision - ALWAYS deals small damage on collision (not just in challenge mode)
    // Cooldown prevents instant death: at most 1 heart lost per second
    *planet_hit_cooldown = (*planet_hit_cooldown - time.delta_seconds()).max(0.0);
    for (planet_transform, body, visual) in &planet_query {
        let to_ship = transform.translation - planet_transform.translation;
        let distance = to_ship.length();
        let min_distance = visual.radius + SHIP_COLLISION_RADIUS;
        if distance < min_distance && distance > 0.001 {
            let push_dir = to_ship.normalize_or_zero();
            let push_distance = (min_distance - distance + 2.0).max(1.0);
            transform.translation += push_dir * push_distance;
            
            // Reduce health a little bit with a 1-second cooldown so it never drains instantly
            if *planet_hit_cooldown <= 0.0 {
                *planet_hit_cooldown = 1.0;
                let damage = 1;
                health.hearts = (health.hearts - damage).max(0);
                mission.last_event = format!("Collided with {}! -{} HP. HP: {}/{}", body.kind.display_name(), damage, health.hearts, health.max_hearts);
                if health.hearts <= 0 {
                    flight.destroyed = true;
                    flight.respawn_timer.reset();
                    mission.last_event = format!("Destroyed by {}!", body.kind.display_name());
                }
            }
            break;
        }
    }

    // Boundary
    let max_radius = if settings.challenge_mode {
        WORLD_BOUNDARY_CHALLENGE
    } else {
        WORLD_BOUNDARY
    };
    if transform.translation.length() > max_radius {
        transform.translation = transform.translation.normalize_or_zero() * max_radius;
    }
}

fn apply_mission_progress(mission: &mut Mission, kind: PlanetKind, action: &str) {
    mission.xp += DISCOVERY_XP_REWARD;
    let required_for_next = mission.tier * XP_PER_LEVEL;
    if mission.xp >= required_for_next {
        mission.tier += 1;
        let reward = match mission.tier {
            2 => "Long-range scanner",
            3 => "Reinforced hull",
            4 => "Quantum drive",
            _ => "Exploration bonus",
        };
        let reward_str = format!("Reward: {}", reward);
        if !mission.unlocked_rewards.contains(&reward_str) {
            mission.unlocked_rewards.push(reward_str.clone());
        }
        mission.last_event = format!("{}: {} | {}", action, kind.display_name(), reward_str);
    } else {
        mission.last_event = format!("{}: {} | XP {}", action, kind.display_name(), mission.xp);
    }
}

fn register_challenge(mission: &mut Mission, amount: u32) {
    mission.challenge_progress = (mission.challenge_progress + amount).min(mission.challenge_goal);
    if mission.challenge_progress >= mission.challenge_goal {
        mission.xp += CHALLENGE_XP_REWARD;
        mission.last_event = format!(
            "Challenge complete: {} (+{} XP)",
            mission.challenge_name, CHALLENGE_XP_REWARD
        );
        // Advance challenge
        mission.challenge_name = match mission.challenge_name.as_str() {
            "Asteroid Dodge" => "Precision Landing".to_string(),
            "Precision Landing" => "Deep Scan".to_string(),
            _ => "Asteroid Dodge".to_string(),
        };
        mission.challenge_goal = match mission.challenge_name.as_str() {
            "Precision Landing" => 2,
            "Deep Scan" => 3,
            _ => 14,
        };
        mission.challenge_progress = 0;
    }
}

/// Apply ship scale from settings.
pub fn apply_ship_scale(
    flow: Res<Flow>,
    settings: Res<Settings>,
    mut ship_query: Query<&mut Transform, With<PlayerShip>>,
) {
    if !flow.is_playing() {
        return;
    }
    for mut transform in &mut ship_query {
        transform.scale = Vec3::splat(settings.ship_scale);
    }
}

/// Respawn destroyed ship.
pub fn handle_destroyed_ship_respawn(
    flow: Res<Flow>,
    time: Res<Time>,
    settings: Res<Settings>,
    mut flight: ResMut<Flight>,
    mut health: ResMut<Health>,
    mut fuel: ResMut<Fuel>,
    mut mission: ResMut<Mission>,
    mut ship_query: Query<&mut Transform, With<PlayerShip>>,
) {
    if !flow.is_playing() || !flight.destroyed {
        return;
    }
    flight.respawn_timer.tick(time.delta());
    if flight.respawn_timer.finished() {
        for mut transform in &mut ship_query {
            transform.translation =
                Vec3::new(RESPAWN_POSITION.0, RESPAWN_POSITION.1, RESPAWN_POSITION.2);
            transform.rotation = Quat::IDENTITY;
        }
        flight.destroyed = false;
        flight.speed = 0.0;
        match settings.difficulty {
            GameDifficulty::Easy => {
                health.hearts = health.max_hearts;
                fuel.current = fuel.max;
            }
            GameDifficulty::Medium => {
                health.hearts = (health.max_hearts * 3) / 4;
                fuel.current = fuel.max * 0.75;
            }
            GameDifficulty::Hard => {
                health.hearts = health.max_hearts / 2;
                fuel.current = fuel.max * 0.5;
            }
            GameDifficulty::Extreme => {
                health.hearts = 1;
                fuel.current = fuel.max * 0.25;
            }
        }
        mission.last_event = format!("Respawned: {} HP, {:.0}% fuel", health.hearts, fuel.current);
    }
}

/// Spawn asteroid hazards - REDUCED spawn rate significantly.
pub fn spawn_asteroid_hazards(
    flow: Res<Flow>,
    settings: Res<Settings>,
    time: Res<Time>,
    mut hazard: ResMut<Hazards>,
    ship_query: Query<&Transform, With<PlayerShip>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    if !flow.is_playing() {
        return;
    }
    let Ok(ship) = ship_query.get_single() else {
        return;
    };
    // REDUCED: spawn interval increased, spawn count decreased
    let (spawn_interval, spawn_count, speed_min, speed_max) = match settings.difficulty {
        GameDifficulty::Easy => (5.0, 1, 6.0, 12.0),
        GameDifficulty::Medium => (3.5, 1, 8.0, 16.0),  // was 1.8, 3
        GameDifficulty::Hard => (2.5, 2, 12.0, 22.0),   // was 1.1, 5
        GameDifficulty::Extreme => (1.8, 3, 16.0, 30.0), // was 0.8, 7
    };
    hazard
        .spawn_timer
        .set_duration(Duration::from_secs_f32(spawn_interval));
    hazard.spawn_timer.tick(time.delta());
    if !hazard.spawn_timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    for _ in 0..spawn_count {
        let random_dir = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-0.6..0.6),
            rng.gen_range(-1.0..1.0),
        )
        .normalize_or_zero();
        let spawn_pos = ship.translation + random_dir * rng.gen_range(140.0..210.0);
        let velocity = (ship.translation - spawn_pos).normalize_or_zero()
            * rng.gen_range(speed_min..speed_max);
        let radius = rng.gen_range(0.45..1.2);
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere { radius }),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.32, 0.31, 0.34),
                    perceptual_roughness: 0.97,
                    metallic: 0.06,
                    ..default()
                }),
                transform: Transform::from_translation(spawn_pos),
                ..default()
            },
            Asteroid { velocity },
            Rotating {
                speed: rng.gen_range(0.5..2.4),
            },
        ));
    }
}

/// Move asteroids and despawn far ones.
pub fn move_asteroids(
    flow: Res<Flow>,
    settings: Res<Settings>,
    time: Res<Time>,
    mut mission: ResMut<Mission>,
    ship_query: Query<&Transform, (With<PlayerShip>, Without<Asteroid>)>,
    mut asteroid_query: Query<(Entity, &mut Transform, &Asteroid), Without<PlayerShip>>,
    mut commands: Commands,
) {
    if !flow.is_playing() {
        return;
    }
    let ship_position = ship_query
        .get_single()
        .map(|s| s.translation)
        .unwrap_or(Vec3::ZERO);
    for (entity, mut transform, asteroid) in &mut asteroid_query {
        transform.translation += asteroid.velocity * time.delta_seconds();
        let distance = transform.translation.distance(ship_position);
        if distance > ASTEROID_DESPAWN_RADIUS
            || transform.translation.length() > ASTEROID_MAX_WORLD_RADIUS
        {
            commands.entity(entity).despawn_recursive();
            if settings.challenge_mode && mission.challenge_name == "Asteroid Dodge" {
                register_challenge(&mut mission, 1);
            }
        }
    }
}

/// Check asteroids colliding with ship.
pub fn asteroid_hit_ship(
    flow: Res<Flow>,
    settings: Res<Settings>,
    mut health: ResMut<Health>,
    mut flight: ResMut<Flight>,
    mut mission: ResMut<Mission>,
    ship_query: Query<&Transform, (With<PlayerShip>, Without<Asteroid>)>,
    asteroid_query: Query<(Entity, &Transform, &Asteroid), Without<PlayerShip>>,
    mut commands: Commands,
) {
    if !flow.is_playing() || flight.destroyed {
        return;
    }
    let Ok(ship) = ship_query.get_single() else {
        return;
    };
    for (entity, asteroid_transform, _) in &asteroid_query {
        if ship.translation.distance(asteroid_transform.translation) < SHIP_COLLISION_RADIUS {
            // ALWAYS deal damage on asteroid collision
            let damage = if settings.challenge_mode {
                settings.difficulty.asteroid_damage() as i32
            } else {
                1
            };
            health.hearts = (health.hearts - damage).max(0);
            commands.entity(entity).despawn_recursive();
            mission.last_event = format!(
                "Asteroid hit! {} damage. HP: {}/{}",
                damage, health.hearts, health.max_hearts
            );
            if health.hearts <= 0 {
                flight.destroyed = true;
                flight.respawn_timer.reset();
                mission.last_event = "Ship destroyed.".to_string();
            }
            if mission.challenge_name == "Asteroid Dodge" {
                mission.challenge_progress = mission.challenge_progress.saturating_sub(1);
            }
            break;
        }
    }
}

/// Consume fuel while moving.
pub fn consume_fuel(
    flow: Res<Flow>,
    settings: Res<Settings>,
    time: Res<Time>,
    mut fuel: ResMut<Fuel>,
    mut mission: ResMut<Mission>,
    flight: Res<Flight>,
) {
    if !flow.is_playing() || flight.destroyed {
        return;
    }
    if flight.speed > FUEL_CONSUMPTION_SPEED_THRESHOLD {
        let rate = settings.difficulty.fuel_consumption_rate();
        fuel.current = (fuel.current - rate * time.delta_seconds()).max(0.0);
        if fuel.current <= 0.0 {
            mission.last_event = "Fuel depleted! Press R to respawn.".to_string();
        } else if fuel.current < FUEL_WARNING_THRESHOLD {
            mission.last_event = format!("Low fuel: {:.1}%", fuel.current);
        }
    }
}

/// Update scanner progress toward target planet.
pub fn update_scanner(
    flow: Res<Flow>,
    settings: Res<Settings>,
    time: Res<Time>,
    target: Res<Target>,
    mut scanner: ResMut<Scanner>,
    mut mission: ResMut<Mission>,
    ship_query: Query<&Transform, With<PlayerShip>>,
    planet_query: Query<(&Transform, &CelestialBody)>,
) {
    if !flow.is_playing() {
        return;
    }
    let Ok(ship) = ship_query.get_single() else {
        return;
    };
    let mut target_distance = None;
    for (planet_transform, body) in &planet_query {
        if body.kind == target.target {
            target_distance = Some(ship.translation.distance(planet_transform.translation));
            break;
        }
    }
    if let Some(distance) = target_distance {
        let scanner_range = settings.difficulty.scanner_range();
        scanner.active = distance <= scanner_range;
        let delta = if scanner.active {
            SCANNER_ACTIVE_RATE
        } else {
            SCANNER_PASSIVE_RATE
        };
        scanner.progress = (scanner.progress + delta * time.delta_seconds()).clamp(0.0, 1.0);
        if scanner.progress >= 1.0 && !mission.discovered.contains(&target.target) {
            mission.discovered.insert(target.target);
            mission.discoveries = mission.discovered.len() as u32;
            apply_mission_progress(&mut mission, target.target, "Scan");
            if settings.challenge_mode && mission.challenge_name == "Deep Scan" {
                register_challenge(&mut mission, 1);
            }
        }
    }
}

/// Update mission objectives.
pub fn update_mission_progress(mut mission: ResMut<Mission>) {
    let discoveries = mission.discoveries;
    let discovered = mission.discovered.clone();
    let mut completed = Vec::new();

    for objective in &mut mission.current_objectives {
        if objective.title == "First Discovery" && !objective.completed {
            objective.progress = discoveries;
            if objective.progress >= objective.goal {
                objective.completed = true;
                completed.push((objective.reward_xp, objective.title.clone()));
            }
        }
        if objective.title == "Solar System Tour" && !objective.completed {
            let major = discovered
                .iter()
                .filter(|&&p| {
                    matches!(
                        p,
                        PlanetKind::Mercury
                            | PlanetKind::Venus
                            | PlanetKind::Earth
                            | PlanetKind::Mars
                            | PlanetKind::Jupiter
                            | PlanetKind::Saturn
                            | PlanetKind::Uranus
                            | PlanetKind::Neptune
                    )
                })
                .count() as u32;
            objective.progress = major;
            if objective.progress >= objective.goal {
                objective.completed = true;
                completed.push((objective.reward_xp, objective.title.clone()));
            }
        }
    }

    for (xp, title) in completed {
        mission.xp += xp;
        mission.last_event = format!("Objective complete: {}! +{} XP", title, xp);
    }

    let new_tier = (mission.xp / XP_PER_TIER) + 1;
    if new_tier > mission.tier {
        mission.tier = new_tier;
        mission.last_event = format!("Tier {} reached!", mission.tier);
    }
}

/// Mouse-click planet inspection.
pub fn inspect_planet_with_mouse(
    flow: Res<Flow>,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    target: Res<Target>,
    ship_query: Query<&Transform, With<PlayerShip>>,
    camera_state: Res<CameraState>,
    camera_query: Query<(&Camera, &GlobalTransform, Option<&CockpitCamera>)>,
    planets: Query<(&GlobalTransform, &CelestialBody, &Visual)>,
    mut focus: ResMut<FocusedInfo>,
) {
    if !flow.is_playing() || !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };

    let mut active_ray = None;
    for (camera, cam_transform, cockpit) in &camera_query {
        let is_cockpit = cockpit.is_some();
        if camera_state.cockpit != is_cockpit || !camera.is_active {
            continue;
        }
        active_ray = camera.viewport_to_world(cam_transform, cursor);
        break;
    }
    let Some(ray) = active_ray else { return };

    let ship_pos = ship_query
        .get_single()
        .map(|t| t.translation)
        .unwrap_or(Vec3::ZERO);
    let mut best_hit: Option<(f32, PlanetKind, Vec3)> = None;

    for (planet_transform, body, visual) in &planets {
        let center = planet_transform.translation();
        if let Some(hit_t) =
            intersect_ray_sphere(ray.origin, ray.direction.into(), center, visual.radius)
        {
            match best_hit {
                None => best_hit = Some((hit_t, body.kind, center)),
                Some((best_t, _, _)) if hit_t < best_t => {
                    best_hit = Some((hit_t, body.kind, center))
                }
                _ => {}
            }
        }
    }

    if let Some((_, kind, center)) = best_hit {
        let distance = ship_pos.distance(center);
        focus.message = format!(
            "{}\n{}\nDistance: {:.1} u\nTarget: {}",
            kind.display_name(),
            kind.lore(),
            distance,
            if kind == target.target { "YES" } else { "NO" }
        );
    }
}

/// Ray-sphere intersection test.
fn intersect_ray_sphere(origin: Vec3, direction: Vec3, center: Vec3, radius: f32) -> Option<f32> {
    let oc = origin - center;
    let a = direction.length_squared();
    let b = 2.0 * oc.dot(direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }
    let sqrt_d = discriminant.sqrt();
    let t0 = (-b - sqrt_d) / (2.0 * a);
    let t1 = (-b + sqrt_d) / (2.0 * a);
    if t0 > 0.0 { Some(t0) } else if t1 > 0.0 { Some(t1) } else { None }
}