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

pub struct Camera2DPlugin;

impl Plugin for Camera2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Camera2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a camera_2_d.
pub fn is_camera_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Camera2D>().is_some()
}

/// A bundle for Camera2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCamera2DBundle {
    pub camera_2_d: GDCamera2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDCamera2DBundle {
    fn default() -> Self {
        Self {
            camera_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Camera2D".to_string()
            }
        }
    }
}

/// Represents a Camera2D.
#[derive(Component)]
pub struct GDCamera2D {
    pub drag_margin_h_enabled: bool,
pub drag_margin_v_enabled: bool,
pub editor_draw_drag_margin: bool,
pub editor_draw_limits: bool,
pub editor_draw_screen: bool,
pub limit_smoothed: bool,
pub offset: Vector2,
pub offset_h: f64,
pub offset_v: f64,
pub rotating: bool,
pub smoothing_enabled: bool,
pub smoothing_speed: f64,
pub zoom: Vector2,
}

impl Default for GDCamera2D {
    fn default() -> Self {
        Self {
            drag_margin_h_enabled: Default::default(),
drag_margin_v_enabled: Default::default(),
editor_draw_drag_margin: Default::default(),
editor_draw_limits: Default::default(),
editor_draw_screen: Default::default(),
limit_smoothed: Default::default(),
offset: Default::default(),
offset_h: Default::default(),
offset_v: Default::default(),
rotating: Default::default(),
smoothing_enabled: Default::default(),
smoothing_speed: Default::default(),
zoom: Default::default(),
        }
    }
}

impl NodeClass for GDCamera2D {
    type Parent = GDNode2D;
    type GodotClass = Camera2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Camera2D>().unwrap();
        world_commands.insert(entity, GDCamera2D {
            drag_margin_h_enabled: component_ref.is_h_drag_enabled(),
drag_margin_v_enabled: component_ref.is_v_drag_enabled(),
editor_draw_drag_margin: component_ref.is_margin_drawing_enabled(),
editor_draw_limits: component_ref.is_limit_drawing_enabled(),
editor_draw_screen: component_ref.is_screen_drawing_enabled(),
limit_smoothed: component_ref.is_limit_smoothing_enabled(),
offset: component_ref.offset(),
offset_h: component_ref.h_offset(),
offset_v: component_ref.v_offset(),
rotating: component_ref.is_rotating(),
smoothing_enabled: component_ref.is_follow_smoothing_enabled(),
smoothing_speed: component_ref.follow_smoothing(),
zoom: component_ref.zoom(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCamera2D {
    
}

fn sync_bevy_owned(query: Query<(&GDCamera2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Camera2D>().unwrap();
        component_ref.set_h_drag_enabled(component.drag_margin_h_enabled);
component_ref.set_v_drag_enabled(component.drag_margin_v_enabled);
component_ref.set_margin_drawing_enabled(component.editor_draw_drag_margin);
component_ref.set_limit_drawing_enabled(component.editor_draw_limits);
component_ref.set_screen_drawing_enabled(component.editor_draw_screen);
component_ref.set_limit_smoothing_enabled(component.limit_smoothed);
component_ref.set_offset(component.offset);
component_ref.set_h_offset(component.offset_h);
component_ref.set_v_offset(component.offset_v);
component_ref.set_rotating(component.rotating);
component_ref.set_enable_follow_smoothing(component.smoothing_enabled);
component_ref.set_follow_smoothing(component.smoothing_speed);
component_ref.set_zoom(component.zoom);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCamera2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Camera2D>().unwrap();
        component.drag_margin_h_enabled = component_ref.is_h_drag_enabled();
component.drag_margin_v_enabled = component_ref.is_v_drag_enabled();
component.editor_draw_drag_margin = component_ref.is_margin_drawing_enabled();
component.editor_draw_limits = component_ref.is_limit_drawing_enabled();
component.editor_draw_screen = component_ref.is_screen_drawing_enabled();
component.limit_smoothed = component_ref.is_limit_smoothing_enabled();
component.offset = component_ref.offset();
component.offset_h = component_ref.h_offset();
component.offset_v = component_ref.v_offset();
component.rotating = component_ref.is_rotating();
component.smoothing_enabled = component_ref.is_follow_smoothing_enabled();
component.smoothing_speed = component_ref.follow_smoothing();
component.zoom = component_ref.zoom();
    }
}