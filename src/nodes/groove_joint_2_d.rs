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

pub struct GrooveJoint2DPlugin;

impl Plugin for GrooveJoint2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<GrooveJoint2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a groove_joint_2_d.
pub fn is_groove_joint_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<GrooveJoint2D>().is_some()
}

/// A bundle for GrooveJoint2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDGrooveJoint2DBundle {
    pub groove_joint_2_d: GDGrooveJoint2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
pub joint_2_d: GDJoint2D,
    pub true_type: TrueNodeType,
}

impl Default for GDGrooveJoint2DBundle {
    fn default() -> Self {
        Self {
            groove_joint_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
joint_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "GrooveJoint2D".to_string()
            }
        }
    }
}

/// Represents a GrooveJoint2D.
#[derive(Component)]
pub struct GDGrooveJoint2D {
    pub initial_offset: f64,
pub length: f64,
}

impl Default for GDGrooveJoint2D {
    fn default() -> Self {
        Self {
            initial_offset: Default::default(),
length: Default::default(),
        }
    }
}

impl NodeClass for GDGrooveJoint2D {
    type Parent = GDJoint2D;
    type GodotClass = GrooveJoint2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<GrooveJoint2D>().unwrap();
        world_commands.insert(entity, GDGrooveJoint2D {
            initial_offset: component_ref.initial_offset(),
length: component_ref.length(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDGrooveJoint2D {
    
}

fn sync_bevy_owned(query: Query<(&GDGrooveJoint2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<GrooveJoint2D>().unwrap();
        component_ref.set_initial_offset(component.initial_offset);
component_ref.set_length(component.length);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDGrooveJoint2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<GrooveJoint2D>().unwrap();
        component.initial_offset = component_ref.initial_offset();
component.length = component_ref.length();
    }
}