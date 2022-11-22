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

pub struct RoomManagerPlugin;

impl Plugin for RoomManagerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<RoomManager>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a room_manager.
pub fn is_room_manager(node: &gdnative::prelude::Node) -> bool {
    node.cast::<RoomManager>().is_some()
}

/// A bundle for RoomManagers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDRoomManagerBundle {
    pub room_manager: GDRoomManager,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDRoomManagerBundle {
    fn default() -> Self {
        Self {
            room_manager: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "RoomManager".to_string()
            }
        }
    }
}

/// Represents a RoomManager.
#[derive(Component)]
pub struct GDRoomManager {
    pub active: bool,
pub debug_sprawl: bool,
pub default_portal_margin: f64,
pub gameplay_monitor: bool,
pub merge_meshes: bool,
pub overlap_warning_threshold: i64,
pub portal_depth_limit: i64,
pub preview_camera: NodePath,
pub roaming_expansion_margin: f64,
pub room_simplify: f64,
pub roomlist: NodePath,
pub show_margins: bool,
pub use_secondary_pvs: bool,
}

impl Default for GDRoomManager {
    fn default() -> Self {
        Self {
            active: Default::default(),
debug_sprawl: Default::default(),
default_portal_margin: Default::default(),
gameplay_monitor: Default::default(),
merge_meshes: Default::default(),
overlap_warning_threshold: Default::default(),
portal_depth_limit: Default::default(),
preview_camera: Default::default(),
roaming_expansion_margin: Default::default(),
room_simplify: Default::default(),
roomlist: Default::default(),
show_margins: Default::default(),
use_secondary_pvs: Default::default(),
        }
    }
}

impl NodeClass for GDRoomManager {
    type Parent = GDSpatial;
    type GodotClass = RoomManager;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<RoomManager>().unwrap();
        world_commands.insert(entity, GDRoomManager {
            active: component_ref.rooms_get_active(),
debug_sprawl: component_ref.debug_sprawl(),
default_portal_margin: component_ref.default_portal_margin(),
gameplay_monitor: component_ref.gameplay_monitor_enabled(),
merge_meshes: component_ref.merge_meshes(),
overlap_warning_threshold: component_ref.overlap_warning_threshold(),
portal_depth_limit: component_ref.portal_depth_limit(),
preview_camera: component_ref.preview_camera_path(),
roaming_expansion_margin: component_ref.roaming_expansion_margin(),
room_simplify: component_ref.room_simplify(),
roomlist: component_ref.roomlist_path(),
show_margins: component_ref.show_margins(),
use_secondary_pvs: component_ref.use_secondary_pvs(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDRoomManager {
    
}

fn sync_bevy_owned(query: Query<(&GDRoomManager, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<RoomManager>().unwrap();
        component_ref.rooms_set_active(component.active);
component_ref.set_debug_sprawl(component.debug_sprawl);
component_ref.set_default_portal_margin(component.default_portal_margin);
component_ref.set_gameplay_monitor_enabled(component.gameplay_monitor);
component_ref.set_merge_meshes(component.merge_meshes);
component_ref.set_overlap_warning_threshold(component.overlap_warning_threshold);
component_ref.set_portal_depth_limit(component.portal_depth_limit);
component_ref.set_preview_camera_path(component.preview_camera.to_godot_string());
component_ref.set_roaming_expansion_margin(component.roaming_expansion_margin);
component_ref.set_room_simplify(component.room_simplify);
component_ref.set_roomlist_path(component.roomlist.to_godot_string());
component_ref.set_show_margins(component.show_margins);
component_ref.set_use_secondary_pvs(component.use_secondary_pvs);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDRoomManager, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<RoomManager>().unwrap();
        component.active = component_ref.rooms_get_active();
component.debug_sprawl = component_ref.debug_sprawl();
component.default_portal_margin = component_ref.default_portal_margin();
component.gameplay_monitor = component_ref.gameplay_monitor_enabled();
component.merge_meshes = component_ref.merge_meshes();
component.overlap_warning_threshold = component_ref.overlap_warning_threshold();
component.portal_depth_limit = component_ref.portal_depth_limit();
component.preview_camera = component_ref.preview_camera_path();
component.roaming_expansion_margin = component_ref.roaming_expansion_margin();
component.room_simplify = component_ref.room_simplify();
component.roomlist = component_ref.roomlist_path();
component.show_margins = component_ref.show_margins();
component.use_secondary_pvs = component_ref.use_secondary_pvs();
    }
}