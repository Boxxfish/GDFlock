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

pub struct OccluderPlugin;

impl Plugin for OccluderPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Occluder>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a occluder.
pub fn is_occluder(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Occluder>().is_some()
}

/// A bundle for Occluders.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDOccluderBundle {
    pub occluder: GDOccluder,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDOccluderBundle {
    fn default() -> Self {
        Self {
            occluder: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Occluder".to_string()
            }
        }
    }
}

/// Represents a Occluder.
#[derive(Component)]
pub struct GDOccluder {
    
}

impl Default for GDOccluder {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDOccluder {
    type Parent = GDSpatial;
    type GodotClass = Occluder;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Occluder>().unwrap();
        world_commands.insert(entity, GDOccluder {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDOccluder {
    
}

fn sync_bevy_owned(query: Query<(&GDOccluder, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Occluder>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDOccluder, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Occluder>().unwrap();
        
    }
}