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

pub struct PinJointPlugin;

impl Plugin for PinJointPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<PinJoint>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a pin_joint.
pub fn is_pin_joint(node: &gdnative::prelude::Node) -> bool {
    node.cast::<PinJoint>().is_some()
}

/// A bundle for PinJoints.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDPinJointBundle {
    pub pin_joint: GDPinJoint,
    pub node: GDNode,
pub spatial: GDSpatial,
pub joint: GDJoint,
    pub true_type: TrueNodeType,
}

impl Default for GDPinJointBundle {
    fn default() -> Self {
        Self {
            pin_joint: Default::default(),
            node: Default::default(),
spatial: Default::default(),
joint: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "PinJoint".to_string()
            }
        }
    }
}

/// Represents a PinJoint.
#[derive(Component)]
pub struct GDPinJoint {
    
}

impl Default for GDPinJoint {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDPinJoint {
    type Parent = GDJoint;
    type GodotClass = PinJoint;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<PinJoint>().unwrap();
        world_commands.insert(entity, GDPinJoint {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDPinJoint {
    
}

fn sync_bevy_owned(query: Query<(&GDPinJoint, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PinJoint>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDPinJoint, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PinJoint>().unwrap();
        
    }
}