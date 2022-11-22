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

pub struct EditorScriptPickerPlugin;

impl Plugin for EditorScriptPickerPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a editor_script_picker.
pub fn is_editor_script_picker(node: &gdnative::prelude::Node) -> bool {
    node.cast::<EditorScriptPicker>().is_some()
}

/// A bundle for EditorScriptPickers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDEditorScriptPickerBundle {
    pub editor_script_picker: GDEditorScriptPicker,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
pub box_container: GDBoxContainer,
pub h_box_container: GDHBoxContainer,
pub editor_resource_picker: GDEditorResourcePicker,
    pub true_type: TrueNodeType,
}

impl Default for GDEditorScriptPickerBundle {
    fn default() -> Self {
        Self {
            editor_script_picker: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
box_container: Default::default(),
h_box_container: Default::default(),
editor_resource_picker: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "EditorScriptPicker".to_string()
            }
        }
    }
}

/// Represents a EditorScriptPicker.
#[derive(Component)]
pub struct GDEditorScriptPicker {
    
}

impl Default for GDEditorScriptPicker {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDEditorScriptPicker {
    type Parent = GDEditorResourcePicker;
    type GodotClass = EditorScriptPicker;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<EditorScriptPicker>().unwrap();
        world_commands.insert(entity, GDEditorScriptPicker {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDEditorScriptPicker {
    
}

fn sync_bevy_owned(query: Query<(&GDEditorScriptPicker, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<EditorScriptPicker>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDEditorScriptPicker, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<EditorScriptPicker>().unwrap();
        
    }
}