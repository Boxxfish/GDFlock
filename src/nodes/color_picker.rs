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

pub struct ColorPickerPlugin;

impl Plugin for ColorPickerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ColorPicker>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a color_picker.
pub fn is_color_picker(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ColorPicker>().is_some()
}

/// A bundle for ColorPickers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDColorPickerBundle {
    pub color_picker: GDColorPicker,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
pub box_container: GDBoxContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDColorPickerBundle {
    fn default() -> Self {
        Self {
            color_picker: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
box_container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ColorPicker".to_string()
            }
        }
    }
}

/// Represents a ColorPicker.
#[derive(Component)]
pub struct GDColorPicker {
    pub color: Color,
pub deferred_mode: bool,
pub edit_alpha: bool,
pub hsv_mode: bool,
pub presets_enabled: bool,
pub presets_visible: bool,
pub raw_mode: bool,
}

impl Default for GDColorPicker {
    fn default() -> Self {
        Self {
            color: Color::from_rgb(0.0, 0.0, 0.0),
deferred_mode: Default::default(),
edit_alpha: Default::default(),
hsv_mode: Default::default(),
presets_enabled: Default::default(),
presets_visible: Default::default(),
raw_mode: Default::default(),
        }
    }
}

impl NodeClass for GDColorPicker {
    type Parent = GDBoxContainer;
    type GodotClass = ColorPicker;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ColorPicker>().unwrap();
        world_commands.insert(entity, GDColorPicker {
            color: component_ref.pick_color(),
deferred_mode: component_ref.is_deferred_mode(),
edit_alpha: component_ref.is_editing_alpha(),
hsv_mode: component_ref.is_hsv_mode(),
presets_enabled: component_ref.are_presets_enabled(),
presets_visible: component_ref.are_presets_visible(),
raw_mode: component_ref.is_raw_mode(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDColorPicker {
    
}

fn sync_bevy_owned(query: Query<(&GDColorPicker, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ColorPicker>().unwrap();
        component_ref.set_pick_color(component.color);
component_ref.set_deferred_mode(component.deferred_mode);
component_ref.set_edit_alpha(component.edit_alpha);
component_ref.set_hsv_mode(component.hsv_mode);
component_ref.set_presets_enabled(component.presets_enabled);
component_ref.set_presets_visible(component.presets_visible);
component_ref.set_raw_mode(component.raw_mode);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDColorPicker, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ColorPicker>().unwrap();
        component.color = component_ref.pick_color();
component.deferred_mode = component_ref.is_deferred_mode();
component.edit_alpha = component_ref.is_editing_alpha();
component.hsv_mode = component_ref.is_hsv_mode();
component.presets_enabled = component_ref.are_presets_enabled();
component.presets_visible = component_ref.are_presets_visible();
component.raw_mode = component_ref.is_raw_mode();
    }
}