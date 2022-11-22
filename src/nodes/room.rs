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

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Room>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a room.
pub fn is_room(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Room>().is_some()
}

/// A bundle for Rooms.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDRoomBundle {
    pub room: GDRoom,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDRoomBundle {
    fn default() -> Self {
        Self {
            room: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Room".to_string()
            }
        }
    }
}

/// Represents a Room.
#[derive(Component)]
pub struct GDRoom {
    pub points: Vec<Vector3>,
pub room_simplify: f64,
pub use_default_simplify: bool,
}

impl Default for GDRoom {
    fn default() -> Self {
        Self {
            points: Default::default(),
room_simplify: Default::default(),
use_default_simplify: Default::default(),
        }
    }
}

impl NodeClass for GDRoom {
    type Parent = GDSpatial;
    type GodotClass = Room;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Room>().unwrap();
        world_commands.insert(entity, GDRoom {
            points: component_ref.points().to_vec(),
room_simplify: component_ref.room_simplify(),
use_default_simplify: component_ref.use_default_simplify(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDRoom {
    
}

fn sync_bevy_owned(query: Query<(&GDRoom, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Room>().unwrap();
        component_ref.set_points(Vector3Array::from_vec(component.points.clone()));
component_ref.set_room_simplify(component.room_simplify);
component_ref.set_use_default_simplify(component.use_default_simplify);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDRoom, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Room>().unwrap();
        component.points = component_ref.points().to_vec();
component.room_simplify = component_ref.room_simplify();
component.use_default_simplify = component_ref.use_default_simplify();
    }
}