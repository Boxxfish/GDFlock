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

pub struct ConeTwistJointPlugin;

impl Plugin for ConeTwistJointPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ConeTwistJoint>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a cone_twist_joint.
pub fn is_cone_twist_joint(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ConeTwistJoint>().is_some()
}

/// A bundle for ConeTwistJoints.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDConeTwistJointBundle {
    pub cone_twist_joint: GDConeTwistJoint,
    pub node: GDNode,
pub spatial: GDSpatial,
pub joint: GDJoint,
    pub true_type: TrueNodeType,
}

impl Default for GDConeTwistJointBundle {
    fn default() -> Self {
        Self {
            cone_twist_joint: Default::default(),
            node: Default::default(),
spatial: Default::default(),
joint: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ConeTwistJoint".to_string()
            }
        }
    }
}

/// Represents a ConeTwistJoint.
#[derive(Component)]
pub struct GDConeTwistJoint {
    
}

impl Default for GDConeTwistJoint {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDConeTwistJoint {
    type Parent = GDJoint;
    type GodotClass = ConeTwistJoint;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ConeTwistJoint>().unwrap();
        world_commands.insert(entity, GDConeTwistJoint {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDConeTwistJoint {
    
}

fn sync_bevy_owned(query: Query<(&GDConeTwistJoint, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ConeTwistJoint>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDConeTwistJoint, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ConeTwistJoint>().unwrap();
        
    }
}