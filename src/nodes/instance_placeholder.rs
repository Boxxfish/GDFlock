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

pub struct InstancePlaceholderPlugin;

impl Plugin for InstancePlaceholderPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a instance_placeholder.
pub fn is_instance_placeholder(node: &gdnative::prelude::Node) -> bool {
    node.cast::<InstancePlaceholder>().is_some()
}

/// A bundle for InstancePlaceholders.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDInstancePlaceholderBundle {
    pub instance_placeholder: GDInstancePlaceholder,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDInstancePlaceholderBundle {
    fn default() -> Self {
        Self {
            instance_placeholder: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "InstancePlaceholder".to_string()
            }
        }
    }
}

/// Represents a InstancePlaceholder.
#[derive(Component)]
pub struct GDInstancePlaceholder {
    
}

impl Default for GDInstancePlaceholder {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDInstancePlaceholder {
    type Parent = GDNode;
    type GodotClass = InstancePlaceholder;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<InstancePlaceholder>().unwrap();
        world_commands.insert(entity, GDInstancePlaceholder {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDInstancePlaceholder {
    
}

fn sync_bevy_owned(query: Query<(&GDInstancePlaceholder, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<InstancePlaceholder>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDInstancePlaceholder, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<InstancePlaceholder>().unwrap();
        
    }
}