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

pub struct HingeJointPlugin;

impl Plugin for HingeJointPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<HingeJoint>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a hinge_joint.
pub fn is_hinge_joint(node: &gdnative::prelude::Node) -> bool {
    node.cast::<HingeJoint>().is_some()
}

/// A bundle for HingeJoints.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDHingeJointBundle {
    pub hinge_joint: GDHingeJoint,
    pub node: GDNode,
pub spatial: GDSpatial,
pub joint: GDJoint,
    pub true_type: TrueNodeType,
}

impl Default for GDHingeJointBundle {
    fn default() -> Self {
        Self {
            hinge_joint: Default::default(),
            node: Default::default(),
spatial: Default::default(),
joint: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "HingeJoint".to_string()
            }
        }
    }
}

/// Represents a HingeJoint.
#[derive(Component)]
pub struct GDHingeJoint {
    
}

impl Default for GDHingeJoint {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDHingeJoint {
    type Parent = GDJoint;
    type GodotClass = HingeJoint;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<HingeJoint>().unwrap();
        world_commands.insert(entity, GDHingeJoint {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDHingeJoint {
    
}

fn sync_bevy_owned(query: Query<(&GDHingeJoint, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<HingeJoint>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDHingeJoint, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<HingeJoint>().unwrap();
        
    }
}