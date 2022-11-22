use bevy_ecs::prelude::*;
use gdnative::api::ResourceLoader;

/// Stores an instance of the Godot singleton.
#[derive(Resource)]
pub struct GDResLoader(pub &'static ResourceLoader);