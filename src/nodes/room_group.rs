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

pub struct RoomGroupPlugin;

impl Plugin for RoomGroupPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<RoomGroup>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a room_group.
pub fn is_room_group(node: &gdnative::prelude::Node) -> bool {
    node.cast::<RoomGroup>().is_some()
}

/// A bundle for RoomGroups.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDRoomGroupBundle {
    pub room_group: GDRoomGroup,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDRoomGroupBundle {
    fn default() -> Self {
        Self {
            room_group: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "RoomGroup".to_string()
            }
        }
    }
}

/// Represents a RoomGroup.
#[derive(Component)]
pub struct GDRoomGroup {
    pub roomgroup_priority: i64,
}

impl Default for GDRoomGroup {
    fn default() -> Self {
        Self {
            roomgroup_priority: Default::default(),
        }
    }
}

impl NodeClass for GDRoomGroup {
    type Parent = GDSpatial;
    type GodotClass = RoomGroup;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<RoomGroup>().unwrap();
        world_commands.insert(entity, GDRoomGroup {
            roomgroup_priority: component_ref.roomgroup_priority(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDRoomGroup {
    
}

fn sync_bevy_owned(query: Query<(&GDRoomGroup, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<RoomGroup>().unwrap();
        component_ref.set_roomgroup_priority(component.roomgroup_priority);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDRoomGroup, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<RoomGroup>().unwrap();
        component.roomgroup_priority = component_ref.roomgroup_priority();
    }
}