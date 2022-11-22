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

pub struct PinJoint2DPlugin;

impl Plugin for PinJoint2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<PinJoint2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a pin_joint_2_d.
pub fn is_pin_joint_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<PinJoint2D>().is_some()
}

/// A bundle for PinJoint2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDPinJoint2DBundle {
    pub pin_joint_2_d: GDPinJoint2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
pub joint_2_d: GDJoint2D,
    pub true_type: TrueNodeType,
}

impl Default for GDPinJoint2DBundle {
    fn default() -> Self {
        Self {
            pin_joint_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
joint_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "PinJoint2D".to_string()
            }
        }
    }
}

/// Represents a PinJoint2D.
#[derive(Component)]
pub struct GDPinJoint2D {
    pub softness: f64,
}

impl Default for GDPinJoint2D {
    fn default() -> Self {
        Self {
            softness: Default::default(),
        }
    }
}

impl NodeClass for GDPinJoint2D {
    type Parent = GDJoint2D;
    type GodotClass = PinJoint2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<PinJoint2D>().unwrap();
        world_commands.insert(entity, GDPinJoint2D {
            softness: component_ref.softness(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDPinJoint2D {
    
}

fn sync_bevy_owned(query: Query<(&GDPinJoint2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PinJoint2D>().unwrap();
        component_ref.set_softness(component.softness);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDPinJoint2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PinJoint2D>().unwrap();
        component.softness = component_ref.softness();
    }
}