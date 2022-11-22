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

pub struct CullInstancePlugin;

impl Plugin for CullInstancePlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a cull_instance.
pub fn is_cull_instance(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CullInstance>().is_some()
}

/// A bundle for CullInstances.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCullInstanceBundle {
    pub cull_instance: GDCullInstance,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDCullInstanceBundle {
    fn default() -> Self {
        Self {
            cull_instance: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CullInstance".to_string()
            }
        }
    }
}

/// Represents a CullInstance.
#[derive(Component)]
pub struct GDCullInstance {
    pub allow_merging: bool,
pub autoplace_priority: i64,
pub include_in_bound: bool,
}

impl Default for GDCullInstance {
    fn default() -> Self {
        Self {
            allow_merging: Default::default(),
autoplace_priority: Default::default(),
include_in_bound: Default::default(),
        }
    }
}

impl NodeClass for GDCullInstance {
    type Parent = GDSpatial;
    type GodotClass = CullInstance;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CullInstance>().unwrap();
        world_commands.insert(entity, GDCullInstance {
            allow_merging: component_ref.allow_merging(),
autoplace_priority: component_ref.portal_autoplace_priority(),
include_in_bound: component_ref.include_in_bound(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCullInstance {
    
}

fn sync_bevy_owned(query: Query<(&GDCullInstance, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CullInstance>().unwrap();
        component_ref.set_allow_merging(component.allow_merging);
component_ref.set_portal_autoplace_priority(component.autoplace_priority);
component_ref.set_include_in_bound(component.include_in_bound);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCullInstance, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CullInstance>().unwrap();
        component.allow_merging = component_ref.allow_merging();
component.autoplace_priority = component_ref.portal_autoplace_priority();
component.include_in_bound = component_ref.include_in_bound();
    }
}