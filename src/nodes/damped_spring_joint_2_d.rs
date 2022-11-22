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

pub struct DampedSpringJoint2DPlugin;

impl Plugin for DampedSpringJoint2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<DampedSpringJoint2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a damped_spring_joint_2_d.
pub fn is_damped_spring_joint_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<DampedSpringJoint2D>().is_some()
}

/// A bundle for DampedSpringJoint2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDDampedSpringJoint2DBundle {
    pub damped_spring_joint_2_d: GDDampedSpringJoint2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
pub joint_2_d: GDJoint2D,
    pub true_type: TrueNodeType,
}

impl Default for GDDampedSpringJoint2DBundle {
    fn default() -> Self {
        Self {
            damped_spring_joint_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
joint_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "DampedSpringJoint2D".to_string()
            }
        }
    }
}

/// Represents a DampedSpringJoint2D.
#[derive(Component)]
pub struct GDDampedSpringJoint2D {
    pub damping: f64,
pub length: f64,
pub rest_length: f64,
pub stiffness: f64,
}

impl Default for GDDampedSpringJoint2D {
    fn default() -> Self {
        Self {
            damping: Default::default(),
length: Default::default(),
rest_length: Default::default(),
stiffness: Default::default(),
        }
    }
}

impl NodeClass for GDDampedSpringJoint2D {
    type Parent = GDJoint2D;
    type GodotClass = DampedSpringJoint2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<DampedSpringJoint2D>().unwrap();
        world_commands.insert(entity, GDDampedSpringJoint2D {
            damping: component_ref.damping(),
length: component_ref.length(),
rest_length: component_ref.rest_length(),
stiffness: component_ref.stiffness(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDDampedSpringJoint2D {
    
}

fn sync_bevy_owned(query: Query<(&GDDampedSpringJoint2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<DampedSpringJoint2D>().unwrap();
        component_ref.set_damping(component.damping);
component_ref.set_length(component.length);
component_ref.set_rest_length(component.rest_length);
component_ref.set_stiffness(component.stiffness);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDDampedSpringJoint2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<DampedSpringJoint2D>().unwrap();
        component.damping = component_ref.damping();
component.length = component_ref.length();
component.rest_length = component_ref.rest_length();
component.stiffness = component_ref.stiffness();
    }
}