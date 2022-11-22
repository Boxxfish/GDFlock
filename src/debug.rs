use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use gdnative::prelude::godot_warn;

use crate::prelude::*;

/// Monitors for common mistakes.
/// This should only be used during development, since it adds overhead.
pub struct GodotDebugPlugin;

impl Plugin for GodotDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(warn_multi_sync)
            .add_system(warn_no_sync);
    }
}

/// Warns that two sync components are on the same entity.
#[allow(clippy::type_complexity)]
fn warn_multi_sync(query: Query<(&GDNode, Option<&GodotOwned>, Option<&BevyOwned>, Option<&NoneOwned>)>) {
    for (node, g, b, n) in query.iter() {
        if (g.is_some() && b.is_some()) || (g.is_some() && n.is_some()) || (b.is_some() && n.is_some()) {
            godot_warn!("Node \"{}\" has multiple sync components.", node.name);
        }
    }
}

/// Warns that an entity has no sync components.
#[allow(clippy::type_complexity)]
fn warn_no_sync(query: Query<&GDNode, (Without<GodotOwned>, Without<BevyOwned>, Without<NoneOwned>)>) {
    for node in query.iter() {
        godot_warn!("Node \"{}\" has no sync components.", node.name);
    }
}