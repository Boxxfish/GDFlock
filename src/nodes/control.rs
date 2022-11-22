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

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Control>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a control.
pub fn is_control(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Control>().is_some()
}

/// A bundle for Controls.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDControlBundle {
    pub control: GDControl,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
    pub true_type: TrueNodeType,
}

impl Default for GDControlBundle {
    fn default() -> Self {
        Self {
            control: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Control".to_string()
            }
        }
    }
}

/// Represents a Control.
#[derive(Component)]
pub struct GDControl {
    pub focus_next: NodePath,
pub focus_previous: NodePath,
pub input_pass_on_modal_close_click: bool,
pub rect_clip_content: bool,
pub rect_min_size: Vector2,
pub rect_pivot_offset: Vector2,
pub rect_rotation: f64,
pub rect_scale: Vector2,
pub size_flags_horizontal: i64,
pub size_flags_stretch_ratio: f64,
pub size_flags_vertical: i64,
pub theme_type_variation: String,
}

impl Default for GDControl {
    fn default() -> Self {
        Self {
            focus_next: Default::default(),
focus_previous: Default::default(),
input_pass_on_modal_close_click: Default::default(),
rect_clip_content: Default::default(),
rect_min_size: Default::default(),
rect_pivot_offset: Default::default(),
rect_rotation: Default::default(),
rect_scale: Default::default(),
size_flags_horizontal: Default::default(),
size_flags_stretch_ratio: Default::default(),
size_flags_vertical: Default::default(),
theme_type_variation: Default::default(),
        }
    }
}

impl NodeClass for GDControl {
    type Parent = GDCanvasItem;
    type GodotClass = Control;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Control>().unwrap();
        world_commands.insert(entity, GDControl {
            focus_next: component_ref.focus_next(),
focus_previous: component_ref.focus_previous(),
input_pass_on_modal_close_click: component_ref.pass_on_modal_close_click(),
rect_clip_content: component_ref.is_clipping_contents(),
rect_min_size: component_ref.custom_minimum_size(),
rect_pivot_offset: component_ref.pivot_offset(),
rect_rotation: component_ref.rotation_degrees(),
rect_scale: component_ref.scale(),
size_flags_horizontal: component_ref.h_size_flags(),
size_flags_stretch_ratio: component_ref.stretch_ratio(),
size_flags_vertical: component_ref.v_size_flags(),
theme_type_variation: component_ref.theme_type_variation().to_string(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDControl {
    
}

fn sync_bevy_owned(query: Query<(&GDControl, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Control>().unwrap();
        component_ref.set_focus_next(component.focus_next.to_godot_string());
component_ref.set_focus_previous(component.focus_previous.to_godot_string());
component_ref.set_pass_on_modal_close_click(component.input_pass_on_modal_close_click);
component_ref.set_clip_contents(component.rect_clip_content);
component_ref.set_custom_minimum_size(component.rect_min_size);
component_ref.set_pivot_offset(component.rect_pivot_offset);
component_ref.set_rotation_degrees(component.rect_rotation);
component_ref.set_scale(component.rect_scale);
component_ref.set_h_size_flags(component.size_flags_horizontal);
component_ref.set_stretch_ratio(component.size_flags_stretch_ratio);
component_ref.set_v_size_flags(component.size_flags_vertical);
component_ref.set_theme_type_variation(component.theme_type_variation.clone());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDControl, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Control>().unwrap();
        component.focus_next = component_ref.focus_next();
component.focus_previous = component_ref.focus_previous();
component.input_pass_on_modal_close_click = component_ref.pass_on_modal_close_click();
component.rect_clip_content = component_ref.is_clipping_contents();
component.rect_min_size = component_ref.custom_minimum_size();
component.rect_pivot_offset = component_ref.pivot_offset();
component.rect_rotation = component_ref.rotation_degrees();
component.rect_scale = component_ref.scale();
component.size_flags_horizontal = component_ref.h_size_flags();
component.size_flags_stretch_ratio = component_ref.stretch_ratio();
component.size_flags_vertical = component_ref.v_size_flags();
component.theme_type_variation = component_ref.theme_type_variation().to_string();
    }
}