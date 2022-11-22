use bevy_ecs::prelude::*;
use bevy_hierarchy::prelude::*;
use bevy_app::prelude::*;
use gdnative::prelude::*;
use gdnative::api::*;
use super::*;

use crate::sync::{BevyOwned, GodotOwned};
use crate::{node_tree::{TrueNodeType, NodeClass}, runner::{GodotStages}};
use crate::node_tree::WorldCommands;
use crate::node_tree::GDNullClass;

use super::GDNode;
use crate::node::add_nodes;

pub struct VisibilityNotifierPlugin;

impl Plugin for VisibilityNotifierPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<VisibilityNotifier>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a visibility_notifier.
pub fn is_visibility_notifier(node: &gdnative::prelude::Node) -> bool {
    node.cast::<VisibilityNotifier>().is_some()
}

/// A bundle for VisibilityNotifiers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDVisibilityNotifierBundle {
    pub visibility_notifier: GDVisibilityNotifier,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
    pub true_type: TrueNodeType,
}

impl Default for GDVisibilityNotifierBundle {
    fn default() -> Self {
        Self {
            visibility_notifier: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "VisibilityNotifier".to_string()
            }
        }
    }
}

/// Represents a VisibilityNotifier.
#[derive(Component)]
pub struct GDVisibilityNotifier {
    pub aabb: Aabb,
pub max_distance: f64,
}

impl Default for GDVisibilityNotifier {
    fn default() -> Self {
        Self {
            aabb: Default::default(),
max_distance: Default::default(),
        }
    }
}

impl NodeClass for GDVisibilityNotifier {
    type Parent = GDCullInstance;
    type GodotClass = VisibilityNotifier;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<VisibilityNotifier>().unwrap();
        world_commands.insert(entity, GDVisibilityNotifier {
            aabb: component_ref.aabb(),
max_distance: component_ref.max_distance(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDVisibilityNotifier {
    
}

fn sync_bevy_owned(query: Query<(&GDVisibilityNotifier, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VisibilityNotifier>().unwrap();
        component_ref.set_aabb(component.aabb);
component_ref.set_max_distance(component.max_distance);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDVisibilityNotifier, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VisibilityNotifier>().unwrap();
        component.aabb = component_ref.aabb();
component.max_distance = component_ref.max_distance();
    }
}