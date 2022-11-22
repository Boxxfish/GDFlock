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

pub struct Node2DPlugin;

impl Plugin for Node2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Node2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a node_2_d.
pub fn is_node_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Node2D>().is_some()
}

/// A bundle for Node2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDNode2DBundle {
    pub node_2_d: GDNode2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
    pub true_type: TrueNodeType,
}

impl Default for GDNode2DBundle {
    fn default() -> Self {
        Self {
            node_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Node2D".to_string()
            }
        }
    }
}

/// Represents a Node2D.
#[derive(Component)]
pub struct GDNode2D {
    pub global_position: Vector2,
pub global_rotation: f64,
pub global_rotation_degrees: f64,
pub global_scale: Vector2,
pub position: Vector2,
pub rotation: f64,
pub rotation_degrees: f64,
pub scale: Vector2,
pub z_as_relative: bool,
pub z_index: i64,
}

impl Default for GDNode2D {
    fn default() -> Self {
        Self {
            global_position: Default::default(),
global_rotation: Default::default(),
global_rotation_degrees: Default::default(),
global_scale: Default::default(),
position: Default::default(),
rotation: Default::default(),
rotation_degrees: Default::default(),
scale: Default::default(),
z_as_relative: Default::default(),
z_index: Default::default(),
        }
    }
}

impl NodeClass for GDNode2D {
    type Parent = GDCanvasItem;
    type GodotClass = Node2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Node2D>().unwrap();
        world_commands.insert(entity, GDNode2D {
            global_position: component_ref.global_position(),
global_rotation: component_ref.global_rotation(),
global_rotation_degrees: component_ref.global_rotation_degrees(),
global_scale: component_ref.global_scale(),
position: component_ref.position(),
rotation: component_ref.rotation(),
rotation_degrees: component_ref.rotation_degrees(),
scale: component_ref.scale(),
z_as_relative: component_ref.is_z_relative(),
z_index: component_ref.z_index(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDNode2D {
    
}

fn sync_bevy_owned(query: Query<(&GDNode2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Node2D>().unwrap();
        component_ref.set_global_position(component.global_position);
component_ref.set_global_rotation(component.global_rotation);
component_ref.set_global_rotation_degrees(component.global_rotation_degrees);
component_ref.set_global_scale(component.global_scale);
component_ref.set_position(component.position);
component_ref.set_rotation(component.rotation);
component_ref.set_rotation_degrees(component.rotation_degrees);
component_ref.set_scale(component.scale);
component_ref.set_z_as_relative(component.z_as_relative);
component_ref.set_z_index(component.z_index);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDNode2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Node2D>().unwrap();
        component.global_position = component_ref.global_position();
component.global_rotation = component_ref.global_rotation();
component.global_rotation_degrees = component_ref.global_rotation_degrees();
component.global_scale = component_ref.global_scale();
component.position = component_ref.position();
component.rotation = component_ref.rotation();
component.rotation_degrees = component_ref.rotation_degrees();
component.scale = component_ref.scale();
component.z_as_relative = component_ref.is_z_relative();
component.z_index = component_ref.z_index();
    }
}