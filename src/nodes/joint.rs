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

pub struct JointPlugin;

impl Plugin for JointPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a joint.
pub fn is_joint(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Joint>().is_some()
}

/// A bundle for Joints.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDJointBundle {
    pub joint: GDJoint,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDJointBundle {
    fn default() -> Self {
        Self {
            joint: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Joint".to_string()
            }
        }
    }
}

/// Represents a Joint.
#[derive(Component)]
pub struct GDJoint {
    
}

impl Default for GDJoint {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDJoint {
    type Parent = GDSpatial;
    type GodotClass = Joint;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Joint>().unwrap();
        world_commands.insert(entity, GDJoint {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDJoint {
    
}

fn sync_bevy_owned(query: Query<(&GDJoint, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Joint>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDJoint, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Joint>().unwrap();
        
    }
}