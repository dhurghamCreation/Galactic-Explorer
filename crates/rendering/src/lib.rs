//! # Rendering Pipeline
//!
//! Manages camera systems, lighting, and visual effects.
//! Implements smooth camera transitions and advanced PBR lighting.

use bevy::prelude::*;
use galactic_explorer_core::prelude::*;

/// Marker component for the main 3D camera (used by combat system)
#[derive(Component)]
pub struct MainCamera;

/// Plugin that registers rendering-related systems.
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraState {
            cockpit: false,
            overview: false,
        })
        .insert_resource(CameraOverviewSettings::default())
        .add_systems(Startup, setup_rendering)
        .add_systems(
            Update,
            (
                camera_follow_ship,
                cockpit_camera_follow,
                toggle_camera_mode,
                camera_zoom,
                camera_overview_controls,
                cycle_camera_mode,
                twinkle_stars,
            ),
        );
    }
}

/// Sets up initial camera and lighting.
pub fn setup_rendering(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 11.0, 26.0).looking_at(Vec3::ZERO, Vec3::Y),
            projection: Projection::Perspective(PerspectiveProjection {
                near: 0.03,
                far: 3000.0,
                ..default()
            }),
            camera: Camera {
                is_active: true,
                ..default()
            },
            ..default()
        },
        FollowCamera {
            distance: 56.0,
            height: 14.0,
        },
        ChaseCamera,
        MainCamera,
    ));

    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                near: 0.03,
                far: 3000.0,
                fov: std::f32::consts::FRAC_PI_3,
                ..default()
            }),
            camera: Camera {
                is_active: false,
                ..default()
            },
            ..default()
        },
        CockpitCamera,
    ));

    // Directional lights
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: SUN_ILLUMINANCE,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(60.0, 90.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: FILL_ILLUMINANCE,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(-45.0, 35.0, -25.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Point light at Sun position
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: SUN_LIGHT_INTENSITY,
            range: SUN_LIGHT_RANGE,
            color: Color::rgb(1.0, 0.88, 0.55),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

/// Smoothly follows the player ship with chase camera.
pub fn camera_follow_ship(
    time: Res<Time>,
    flow: Res<Flow>,
    mode: Res<CameraState>,
    mut camera_query: Query<
        (&mut Transform, &FollowCamera),
        (With<ChaseCamera>, Without<PlayerShip>),
    >,
    ship_query: Query<&Transform, (With<PlayerShip>, Without<FollowCamera>)>,
) {
    if !flow.is_playing() || mode.cockpit {
        return;
    }
    let Ok(ship_transform) = ship_query.get_single() else {
        return;
    };
    for (mut camera_transform, settings) in &mut camera_query {
        let ship_back = -ship_transform.forward();
        let desired =
            ship_transform.translation + ship_back * settings.distance + Vec3::Y * settings.height;
        camera_transform.translation = camera_transform
            .translation
            .lerp(desired, CAMERA_FOLLOW_LERP * time.delta_seconds());
        camera_transform.look_at(ship_transform.translation, Vec3::Y);
    }
}

/// Cockpit camera follows ship rotation exactly.
pub fn cockpit_camera_follow(
    flow: Res<Flow>,
    mode: Res<CameraState>,
    mut camera_query: Query<&mut Transform, (With<CockpitCamera>, Without<PlayerShip>)>,
    ship_query: Query<&Transform, With<PlayerShip>>,
) {
    if !flow.is_playing() || !mode.cockpit {
        return;
    }
    let Ok(ship_transform) = ship_query.get_single() else {
        return;
    };
    for mut camera_transform in &mut camera_query {
        // Place camera in front of the ship looking forward, not inside it
        let cockpit_offset = ship_transform.forward() * 3.5 + Vec3::Y * 0.5;
        camera_transform.translation = ship_transform.translation + cockpit_offset;
        camera_transform.look_at(ship_transform.translation + ship_transform.forward() * 20.0, Vec3::Y);
    }
}

/// Toggle between chase and cockpit camera.
pub fn toggle_camera_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    flow: Res<Flow>,
    mut mode: ResMut<CameraState>,
    mut camera_query: Query<(&mut Camera, Option<&ChaseCamera>, Option<&CockpitCamera>)>,
) {
    if !flow.is_playing() || !keyboard.just_pressed(KeyCode::KeyV) {
        return;
    }
    mode.cockpit = !mode.cockpit;
    if mode.cockpit {
        mode.overview = false;
    }
    for (mut camera, chase, cockpit) in &mut camera_query {
        if chase.is_some() {
            camera.is_active = !mode.cockpit;
        }
        if cockpit.is_some() {
            camera.is_active = mode.cockpit;
        }
    }
}

/// Zoom the chase camera in/out.
pub fn camera_zoom(
    keyboard: Res<ButtonInput<KeyCode>>,
    flow: Res<Flow>,
    mode: Res<CameraState>,
    time: Res<Time>,
    mut camera_query: Query<&mut FollowCamera, With<ChaseCamera>>,
) {
    if !flow.is_playing() || mode.cockpit {
        return;
    }
    for mut settings in &mut camera_query {
        if keyboard.pressed(KeyCode::Equal) || keyboard.pressed(KeyCode::NumpadAdd) {
            settings.distance -= CAMERA_ZOOM_SPEED * time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::Minus) || keyboard.pressed(KeyCode::NumpadSubtract) {
            settings.distance += CAMERA_ZOOM_SPEED * time.delta_seconds();
        }
        settings.distance = settings
            .distance
            .clamp(CAMERA_MIN_DISTANCE, CAMERA_MAX_DISTANCE);
    }
}

/// Overview/P mode toggle.
pub fn camera_overview_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    flow: Res<Flow>,
    mut mode: ResMut<CameraState>,
    mut camera_query: Query<&mut FollowCamera, With<ChaseCamera>>,
) {
    if !flow.is_playing() || mode.cockpit {
        return;
    }
    for mut settings in &mut camera_query {
        if keyboard.just_pressed(KeyCode::KeyO) {
            mode.overview = true;
            mode.cockpit = false;
            settings.distance = 320.0;
            settings.height = 120.0;
        }
        if keyboard.just_pressed(KeyCode::KeyP) {
            mode.overview = false;
            mode.cockpit = false;
            settings.distance = 56.0;
            settings.height = 14.0;
        }
    }
}

/// Cycle through camera modes.
pub fn cycle_camera_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    flow: Res<Flow>,
    mut mode: ResMut<CameraState>,
    mut camera_query: Query<(&mut Camera, Option<&ChaseCamera>, Option<&CockpitCamera>)>,
    mut settings_query: Query<&mut FollowCamera, With<ChaseCamera>>,
) {
    if !flow.is_playing() || !keyboard.just_pressed(KeyCode::KeyC) {
        return;
    }
    if !mode.cockpit && !mode.overview {
        mode.cockpit = true;
        mode.overview = false;
    } else if mode.cockpit {
        mode.cockpit = false;
        mode.overview = true;
        for mut settings in &mut settings_query {
            settings.distance = 320.0;
            settings.height = 120.0;
        }
    } else {
        mode.cockpit = false;
        mode.overview = false;
        for mut settings in &mut settings_query {
            settings.distance = 56.0;
            settings.height = 14.0;
        }
    }
    for (mut camera, chase, cockpit) in &mut camera_query {
        if chase.is_some() {
            camera.is_active = !mode.cockpit;
        }
        if cockpit.is_some() {
            camera.is_active = mode.cockpit;
        }
    }
}

/// Star twinkle animation system.
pub fn twinkle_stars(time: Res<Time>, mut query: Query<(&Star, &mut Transform)>) {
    let t = time.elapsed_seconds();
    for (star, mut transform) in &mut query {
        let pulse = 1.0 + (t * STAR_TWINKLE_SPEED + star.phase).sin() * star.amplitude;
        transform.scale = Vec3::splat(pulse.max(STAR_MIN_SCALE));
    }
}
