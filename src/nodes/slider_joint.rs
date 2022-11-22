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

pub struct SliderJointPlugin;

impl Plugin for SliderJointPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<SliderJoint>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a slider_joint.
pub fn is_slider_joint(node: &gdnative::prelude::Node) -> bool {
    node.cast::<SliderJoint>().is_some()
}

/// A bundle for SliderJoints.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDSliderJointBundle {
    pub slider_joint: GDSliderJoint,
    pub node: GDNode,
pub spatial: GDSpatial,
pub joint: GDJoint,
    pub true_type: TrueNodeType,
}

impl Default for GDSliderJointBundle {
    fn default() -> Self {
        Self {
            slider_joint: Default::default(),
            node: Default::default(),
spatial: Default::default(),
joint: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "SliderJoint".to_string()
            }
        }
    }
}

/// Represents a SliderJoint.
#[derive(Component)]
pub struct GDSliderJoint {
    
}

impl Default for GDSliderJoint {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDSliderJoint {
    type Parent = GDJoint;
    type GodotClass = SliderJoint;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<SliderJoint>().unwrap();
        world_commands.insert(entity, GDSliderJoint {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDSliderJoint {
    
}

fn sync_bevy_owned(query: Query<(&GDSliderJoint, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<SliderJoint>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDSliderJoint, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<SliderJoint>().unwrap();
        
    }
}