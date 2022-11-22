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

pub struct Generic6DOFJointPlugin;

impl Plugin for Generic6DOFJointPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Generic6DOFJoint>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a generic_6_d_o_f_joint.
pub fn is_generic_6_d_o_f_joint(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Generic6DOFJoint>().is_some()
}

/// A bundle for Generic6DOFJoints.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDGeneric6DOFJointBundle {
    pub generic_6_d_o_f_joint: GDGeneric6DOFJoint,
    pub node: GDNode,
pub spatial: GDSpatial,
pub joint: GDJoint,
    pub true_type: TrueNodeType,
}

impl Default for GDGeneric6DOFJointBundle {
    fn default() -> Self {
        Self {
            generic_6_d_o_f_joint: Default::default(),
            node: Default::default(),
spatial: Default::default(),
joint: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Generic6DOFJoint".to_string()
            }
        }
    }
}

/// Represents a Generic6DOFJoint.
#[derive(Component)]
pub struct GDGeneric6DOFJoint {
    
}

impl Default for GDGeneric6DOFJoint {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDGeneric6DOFJoint {
    type Parent = GDJoint;
    type GodotClass = Generic6DOFJoint;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Generic6DOFJoint>().unwrap();
        world_commands.insert(entity, GDGeneric6DOFJoint {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDGeneric6DOFJoint {
    
}

fn sync_bevy_owned(query: Query<(&GDGeneric6DOFJoint, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Generic6DOFJoint>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDGeneric6DOFJoint, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Generic6DOFJoint>().unwrap();
        
    }
}