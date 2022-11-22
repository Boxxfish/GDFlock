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


use crate::node::add_nodes;

pub struct NodePlugin;

impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Node>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a node.
pub fn is_node(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Node>().is_some()
}

/// A bundle for Nodes.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDNodeBundle {
    pub node: GDNode,
    
    pub true_type: TrueNodeType,
}

impl Default for GDNodeBundle {
    fn default() -> Self {
        Self {
            node: Default::default(),
            
            true_type: TrueNodeType {
                node: None,
                class_name: "Node".to_string()
            }
        }
    }
}

/// Represents a Node.
#[derive(Component)]
pub struct GDNode {
    pub filename: String,
pub name: String,
pub process_priority: i64,
pub unique_name_in_owner: bool,
}

impl Default for GDNode {
    fn default() -> Self {
        Self {
            filename: Default::default(),
name: Default::default(),
process_priority: Default::default(),
unique_name_in_owner: Default::default(),
        }
    }
}

impl NodeClass for GDNode {
    type Parent = GDNullClass;
    type GodotClass = Node;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Node>().unwrap();
        world_commands.insert(entity, GDNode {
            filename: component_ref.filename().to_string(),
name: component_ref.name().to_string(),
process_priority: component_ref.process_priority(),
unique_name_in_owner: component_ref.is_unique_name_in_owner(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDNode {
    
}

fn sync_bevy_owned(query: Query<(&GDNode, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Node>().unwrap();
        component_ref.set_filename(component.filename.clone());
component_ref.set_name(component.name.clone());
component_ref.set_process_priority(component.process_priority);
component_ref.set_unique_name_in_owner(component.unique_name_in_owner);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDNode, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Node>().unwrap();
        component.filename = component_ref.filename().to_string();
component.name = component_ref.name().to_string();
component.process_priority = component_ref.process_priority();
component.unique_name_in_owner = component_ref.is_unique_name_in_owner();
    }
}