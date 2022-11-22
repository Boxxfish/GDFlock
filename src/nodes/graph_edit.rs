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

pub struct GraphEditPlugin;

impl Plugin for GraphEditPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<GraphEdit>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a graph_edit.
pub fn is_graph_edit(node: &gdnative::prelude::Node) -> bool {
    node.cast::<GraphEdit>().is_some()
}

/// A bundle for GraphEdits.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDGraphEditBundle {
    pub graph_edit: GDGraphEdit,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
    pub true_type: TrueNodeType,
}

impl Default for GDGraphEditBundle {
    fn default() -> Self {
        Self {
            graph_edit: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "GraphEdit".to_string()
            }
        }
    }
}

/// Represents a GraphEdit.
#[derive(Component)]
pub struct GDGraphEdit {
    pub minimap_enabled: bool,
pub minimap_opacity: f64,
pub minimap_size: Vector2,
pub right_disconnects: bool,
pub scroll_offset: Vector2,
pub show_zoom_label: bool,
pub snap_distance: i64,
pub use_snap: bool,
pub zoom: f64,
pub zoom_max: f64,
pub zoom_min: f64,
pub zoom_step: f64,
}

impl Default for GDGraphEdit {
    fn default() -> Self {
        Self {
            minimap_enabled: Default::default(),
minimap_opacity: Default::default(),
minimap_size: Default::default(),
right_disconnects: Default::default(),
scroll_offset: Default::default(),
show_zoom_label: Default::default(),
snap_distance: Default::default(),
use_snap: Default::default(),
zoom: Default::default(),
zoom_max: Default::default(),
zoom_min: Default::default(),
zoom_step: Default::default(),
        }
    }
}

impl NodeClass for GDGraphEdit {
    type Parent = GDControl;
    type GodotClass = GraphEdit;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<GraphEdit>().unwrap();
        world_commands.insert(entity, GDGraphEdit {
            minimap_enabled: component_ref.is_minimap_enabled(),
minimap_opacity: component_ref.minimap_opacity(),
minimap_size: component_ref.minimap_size(),
right_disconnects: component_ref.is_right_disconnects_enabled(),
scroll_offset: component_ref.scroll_ofs(),
show_zoom_label: component_ref.is_showing_zoom_label(),
snap_distance: component_ref.snap(),
use_snap: component_ref.is_using_snap(),
zoom: component_ref.zoom(),
zoom_max: component_ref.zoom_max(),
zoom_min: component_ref.zoom_min(),
zoom_step: component_ref.zoom_step(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDGraphEdit {
    
}

fn sync_bevy_owned(query: Query<(&GDGraphEdit, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<GraphEdit>().unwrap();
        component_ref.set_minimap_enabled(component.minimap_enabled);
component_ref.set_minimap_opacity(component.minimap_opacity);
component_ref.set_minimap_size(component.minimap_size);
component_ref.set_right_disconnects(component.right_disconnects);
component_ref.set_scroll_ofs(component.scroll_offset);
component_ref.set_show_zoom_label(component.show_zoom_label);
component_ref.set_snap(component.snap_distance);
component_ref.set_use_snap(component.use_snap);
component_ref.set_zoom(component.zoom);
component_ref.set_zoom_max(component.zoom_max);
component_ref.set_zoom_min(component.zoom_min);
component_ref.set_zoom_step(component.zoom_step);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDGraphEdit, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<GraphEdit>().unwrap();
        component.minimap_enabled = component_ref.is_minimap_enabled();
component.minimap_opacity = component_ref.minimap_opacity();
component.minimap_size = component_ref.minimap_size();
component.right_disconnects = component_ref.is_right_disconnects_enabled();
component.scroll_offset = component_ref.scroll_ofs();
component.show_zoom_label = component_ref.is_showing_zoom_label();
component.snap_distance = component_ref.snap();
component.use_snap = component_ref.is_using_snap();
component.zoom = component_ref.zoom();
component.zoom_max = component_ref.zoom_max();
component.zoom_min = component_ref.zoom_min();
component.zoom_step = component_ref.zoom_step();
    }
}