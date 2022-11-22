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

pub struct PathFollow2DPlugin;

impl Plugin for PathFollow2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<PathFollow2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a path_follow_2_d.
pub fn is_path_follow_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<PathFollow2D>().is_some()
}

/// A bundle for PathFollow2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDPathFollow2DBundle {
    pub path_follow_2_d: GDPathFollow2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDPathFollow2DBundle {
    fn default() -> Self {
        Self {
            path_follow_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "PathFollow2D".to_string()
            }
        }
    }
}

/// Represents a PathFollow2D.
#[derive(Component)]
pub struct GDPathFollow2D {
    pub cubic_interp: bool,
pub h_offset: f64,
pub lookahead: f64,
pub _loop: bool,
pub offset: f64,
pub rotate: bool,
pub unit_offset: f64,
pub v_offset: f64,
}

impl Default for GDPathFollow2D {
    fn default() -> Self {
        Self {
            cubic_interp: Default::default(),
h_offset: Default::default(),
lookahead: Default::default(),
_loop: Default::default(),
offset: Default::default(),
rotate: Default::default(),
unit_offset: Default::default(),
v_offset: Default::default(),
        }
    }
}

impl NodeClass for GDPathFollow2D {
    type Parent = GDNode2D;
    type GodotClass = PathFollow2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<PathFollow2D>().unwrap();
        world_commands.insert(entity, GDPathFollow2D {
            cubic_interp: component_ref.cubic_interpolation(),
h_offset: component_ref.h_offset(),
lookahead: component_ref.lookahead(),
_loop: component_ref.has_loop(),
offset: component_ref.offset(),
rotate: component_ref.is_rotating(),
unit_offset: component_ref.unit_offset(),
v_offset: component_ref.v_offset(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDPathFollow2D {
    
}

fn sync_bevy_owned(query: Query<(&GDPathFollow2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PathFollow2D>().unwrap();
        component_ref.set_cubic_interpolation(component.cubic_interp);
component_ref.set_h_offset(component.h_offset);
component_ref.set_lookahead(component.lookahead);
component_ref.set_loop(component._loop);
component_ref.set_offset(component.offset);
component_ref.set_rotate(component.rotate);
component_ref.set_unit_offset(component.unit_offset);
component_ref.set_v_offset(component.v_offset);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDPathFollow2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PathFollow2D>().unwrap();
        component.cubic_interp = component_ref.cubic_interpolation();
component.h_offset = component_ref.h_offset();
component.lookahead = component_ref.lookahead();
component._loop = component_ref.has_loop();
component.offset = component_ref.offset();
component.rotate = component_ref.is_rotating();
component.unit_offset = component_ref.unit_offset();
component.v_offset = component_ref.v_offset();
    }
}