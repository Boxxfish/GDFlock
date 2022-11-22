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

pub struct EditorResourcePickerPlugin;

impl Plugin for EditorResourcePickerPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a editor_resource_picker.
pub fn is_editor_resource_picker(node: &gdnative::prelude::Node) -> bool {
    node.cast::<EditorResourcePicker>().is_some()
}

/// A bundle for EditorResourcePickers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDEditorResourcePickerBundle {
    pub editor_resource_picker: GDEditorResourcePicker,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
pub box_container: GDBoxContainer,
pub h_box_container: GDHBoxContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDEditorResourcePickerBundle {
    fn default() -> Self {
        Self {
            editor_resource_picker: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
box_container: Default::default(),
h_box_container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "EditorResourcePicker".to_string()
            }
        }
    }
}

/// Represents a EditorResourcePicker.
#[derive(Component)]
pub struct GDEditorResourcePicker {
    pub base_type: String,
pub editable: bool,
pub toggle_mode: bool,
}

impl Default for GDEditorResourcePicker {
    fn default() -> Self {
        Self {
            base_type: Default::default(),
editable: Default::default(),
toggle_mode: Default::default(),
        }
    }
}

impl NodeClass for GDEditorResourcePicker {
    type Parent = GDHBoxContainer;
    type GodotClass = EditorResourcePicker;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<EditorResourcePicker>().unwrap();
        world_commands.insert(entity, GDEditorResourcePicker {
            base_type: component_ref.base_type().to_string(),
editable: component_ref.is_editable(),
toggle_mode: component_ref.is_toggle_mode(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDEditorResourcePicker {
    
}

fn sync_bevy_owned(query: Query<(&GDEditorResourcePicker, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<EditorResourcePicker>().unwrap();
        component_ref.set_base_type(component.base_type.clone());
component_ref.set_editable(component.editable);
component_ref.set_toggle_mode(component.toggle_mode);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDEditorResourcePicker, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<EditorResourcePicker>().unwrap();
        component.base_type = component_ref.base_type().to_string();
component.editable = component_ref.is_editable();
component.toggle_mode = component_ref.is_toggle_mode();
    }
}