//! # Asset Pipeline
//!
//! Handles loading, caching, and management of game assets.
//! Uses async loading to prevent game freezes during asset streaming.

use bevy::prelude::*;
use galactic_explorer_core::prelude::*;
use std::collections::HashMap;

/// Plugin that registers the asset loading systems.
pub struct AssetPipelinePlugin;

impl Plugin for AssetPipelinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetCache>()
            .add_systems(Startup, warm_asset_cache);
    }
}

/// Tracks loaded assets and their states.
#[derive(Resource, Default)]
pub struct AssetCache {
    pub textures: HashMap<String, Handle<Image>>,
    pub models: HashMap<String, Handle<Scene>>,
    pub audio: HashMap<String, Handle<AudioSource>>,
    pub ready: bool,
}

/// Warmed asset handles ready for use.
#[derive(Resource)]
pub struct WarmedAssets {
    pub textures: Vec<Handle<Image>>,
    pub scenes: Vec<Handle<Scene>>,
}

impl Default for WarmedAssets {
    fn default() -> Self {
        Self {
            textures: Vec::new(),
            scenes: Vec::new(),
        }
    }
}

fn warm_asset_cache(asset_server: Res<AssetServer>, mut cache: ResMut<AssetCache>) {
    // Pre-warm essential textures
    for kind in PlanetKind::ALL {
        let path = kind.texture_path();
        let handle: Handle<Image> = asset_server.load(path);
        cache.textures.insert(path.to_string(), handle);
    }

    // Pre-warm additional textures
    let extra_textures = &["textures/stars.png", "textures/saturnring.png"];
    for path in extra_textures {
        let handle: Handle<Image> = asset_server.load(*path);
        cache.textures.insert(path.to_string(), handle);
    }

    // Pre-warm models
    let model_handle: Handle<Scene> = asset_server.load("models/shuttle.glb#Scene0");
    cache.models.insert("shuttle".to_string(), model_handle);

    cache.ready = true;
    log::info!("Asset cache warmed with {} textures", cache.textures.len());
}

/// Returns a texture handle from cache, loading it if necessary.
pub fn get_texture(
    _cache: &AssetCache,
    asset_server: &AssetServer,
    path: &'static str,
) -> Handle<Image> {
    asset_server.load(path)
}
