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

pub struct PathFollowPlugin;

impl Plugin for PathFollowPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<PathFollow>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a path_follow.
pub fn is_path_follow(node: &gdnative::prelude::Node) -> bool {
    node.cast::<PathFollow>().is_some()
}

/// A bundle for PathFollows.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDPathFollowBundle {
    pub path_follow: GDPathFollow,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDPathFollowBundle {
    fn default() -> Self {
        Self {
            path_follow: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "PathFollow".to_string()
            }
        }
    }
}

/// Represents a PathFollow.
#[derive(Component)]
pub struct GDPathFollow {
    pub cubic_interp: bool,
pub h_offset: f64,
pub _loop: bool,
pub offset: f64,
pub unit_offset: f64,
pub v_offset: f64,
}

impl Default for GDPathFollow {
    fn default() -> Self {
        Self {
            cubic_interp: Default::default(),
h_offset: Default::default(),
_loop: Default::default(),
offset: Default::default(),
unit_offset: Default::default(),
v_offset: Default::default(),
        }
    }
}

impl NodeClass for GDPathFollow {
    type Parent = GDSpatial;
    type GodotClass = PathFollow;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<PathFollow>().unwrap();
        world_commands.insert(entity, GDPathFollow {
            cubic_interp: component_ref.cubic_interpolation(),
h_offset: component_ref.h_offset(),
_loop: component_ref.has_loop(),
offset: component_ref.offset(),
unit_offset: component_ref.unit_offset(),
v_offset: component_ref.v_offset(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDPathFollow {
    
}

fn sync_bevy_owned(query: Query<(&GDPathFollow, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PathFollow>().unwrap();
        component_ref.set_cubic_interpolation(component.cubic_interp);
component_ref.set_h_offset(component.h_offset);
component_ref.set_loop(component._loop);
component_ref.set_offset(component.offset);
component_ref.set_unit_offset(component.unit_offset);
component_ref.set_v_offset(component.v_offset);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDPathFollow, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PathFollow>().unwrap();
        component.cubic_interp = component_ref.cubic_interpolation();
component.h_offset = component_ref.h_offset();
component._loop = component_ref.has_loop();
component.offset = component_ref.offset();
component.unit_offset = component_ref.unit_offset();
component.v_offset = component_ref.v_offset();
    }
}