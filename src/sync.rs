use bevy_ecs::prelude::*;

/// Marks that this node should be controlled from the Bevy side. Any modifications from
/// GDScript will be overwritten by the Bevy component.
#[derive(Component)]
pub struct BevyOwned;

/// Marks that this node should be controlled from the Godot side. This makes the node
/// read only from the Bevy side.
#[derive(Component)]
pub struct GodotOwned;

/// Marks that this component deliberately is not being synced.
#[derive(Component)]
pub struct NoneOwned;