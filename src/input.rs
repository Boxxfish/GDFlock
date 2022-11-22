use bevy_ecs::prelude::*;
use gdnative::api::Input;

/// Stores an instance of the Godot singleton.
#[derive(Resource)]
pub struct GDInput(pub &'static Input);