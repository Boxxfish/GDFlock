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

pub struct Joint2DPlugin;

impl Plugin for Joint2DPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a joint_2_d.
pub fn is_joint_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Joint2D>().is_some()
}

/// A bundle for Joint2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDJoint2DBundle {
    pub joint_2_d: GDJoint2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDJoint2DBundle {
    fn default() -> Self {
        Self {
            joint_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Joint2D".to_string()
            }
        }
    }
}

/// Represents a Joint2D.
#[derive(Component)]
pub struct GDJoint2D {
    pub bias: f64,
pub disable_collision: bool,
pub node_a: NodePath,
pub node_b: NodePath,
}

impl Default for GDJoint2D {
    fn default() -> Self {
        Self {
            bias: Default::default(),
disable_collision: Default::default(),
node_a: Default::default(),
node_b: Default::default(),
        }
    }
}

impl NodeClass for GDJoint2D {
    type Parent = GDNode2D;
    type GodotClass = Joint2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Joint2D>().unwrap();
        world_commands.insert(entity, GDJoint2D {
            bias: component_ref.bias(),
disable_collision: component_ref.exclude_nodes_from_collision(),
node_a: component_ref.node_a(),
node_b: component_ref.node_b(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDJoint2D {
    
}

fn sync_bevy_owned(query: Query<(&GDJoint2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Joint2D>().unwrap();
        component_ref.set_bias(component.bias);
component_ref.set_exclude_nodes_from_collision(component.disable_collision);
component_ref.set_node_a(component.node_a.to_godot_string());
component_ref.set_node_b(component.node_b.to_godot_string());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDJoint2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Joint2D>().unwrap();
        component.bias = component_ref.bias();
component.disable_collision = component_ref.exclude_nodes_from_collision();
component.node_a = component_ref.node_a();
component.node_b = component_ref.node_b();
    }
}