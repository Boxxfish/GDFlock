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

pub struct EditorPropertyPlugin;

impl Plugin for EditorPropertyPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a editor_property.
pub fn is_editor_property(node: &gdnative::prelude::Node) -> bool {
    node.cast::<EditorProperty>().is_some()
}

/// A bundle for EditorPropertys.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDEditorPropertyBundle {
    pub editor_property: GDEditorProperty,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDEditorPropertyBundle {
    fn default() -> Self {
        Self {
            editor_property: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "EditorProperty".to_string()
            }
        }
    }
}

/// Represents a EditorProperty.
#[derive(Component)]
pub struct GDEditorProperty {
    pub checkable: bool,
pub checked: bool,
pub draw_red: bool,
pub keying: bool,
pub label: String,
pub read_only: bool,
}

impl Default for GDEditorProperty {
    fn default() -> Self {
        Self {
            checkable: Default::default(),
checked: Default::default(),
draw_red: Default::default(),
keying: Default::default(),
label: Default::default(),
read_only: Default::default(),
        }
    }
}

impl NodeClass for GDEditorProperty {
    type Parent = GDContainer;
    type GodotClass = EditorProperty;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<EditorProperty>().unwrap();
        world_commands.insert(entity, GDEditorProperty {
            checkable: component_ref.is_checkable(),
checked: component_ref.is_checked(),
draw_red: component_ref.is_draw_red(),
keying: component_ref.is_keying(),
label: component_ref.label().to_string(),
read_only: component_ref.is_read_only(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDEditorProperty {
    
}

fn sync_bevy_owned(query: Query<(&GDEditorProperty, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<EditorProperty>().unwrap();
        component_ref.set_checkable(component.checkable);
component_ref.set_checked(component.checked);
component_ref.set_draw_red(component.draw_red);
component_ref.set_keying(component.keying);
component_ref.set_label(component.label.clone());
component_ref.set_read_only(component.read_only);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDEditorProperty, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<EditorProperty>().unwrap();
        component.checkable = component_ref.is_checkable();
component.checked = component_ref.is_checked();
component.draw_red = component_ref.is_draw_red();
component.keying = component_ref.is_keying();
component.label = component_ref.label().to_string();
component.read_only = component_ref.is_read_only();
    }
}