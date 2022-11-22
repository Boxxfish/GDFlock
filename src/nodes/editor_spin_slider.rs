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

pub struct EditorSpinSliderPlugin;

impl Plugin for EditorSpinSliderPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a editor_spin_slider.
pub fn is_editor_spin_slider(node: &gdnative::prelude::Node) -> bool {
    node.cast::<EditorSpinSlider>().is_some()
}

/// A bundle for EditorSpinSliders.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDEditorSpinSliderBundle {
    pub editor_spin_slider: GDEditorSpinSlider,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub range: GDRange,
    pub true_type: TrueNodeType,
}

impl Default for GDEditorSpinSliderBundle {
    fn default() -> Self {
        Self {
            editor_spin_slider: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
range: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "EditorSpinSlider".to_string()
            }
        }
    }
}

/// Represents a EditorSpinSlider.
#[derive(Component)]
pub struct GDEditorSpinSlider {
    pub flat: bool,
pub hide_slider: bool,
pub label: String,
pub read_only: bool,
}

impl Default for GDEditorSpinSlider {
    fn default() -> Self {
        Self {
            flat: Default::default(),
hide_slider: Default::default(),
label: Default::default(),
read_only: Default::default(),
        }
    }
}

impl NodeClass for GDEditorSpinSlider {
    type Parent = GDRange;
    type GodotClass = EditorSpinSlider;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<EditorSpinSlider>().unwrap();
        world_commands.insert(entity, GDEditorSpinSlider {
            flat: component_ref.is_flat(),
hide_slider: component_ref.is_hiding_slider(),
label: component_ref.label().to_string(),
read_only: component_ref.is_read_only(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDEditorSpinSlider {
    
}

fn sync_bevy_owned(query: Query<(&GDEditorSpinSlider, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<EditorSpinSlider>().unwrap();
        component_ref.set_flat(component.flat);
component_ref.set_hide_slider(component.hide_slider);
component_ref.set_label(component.label.clone());
component_ref.set_read_only(component.read_only);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDEditorSpinSlider, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<EditorSpinSlider>().unwrap();
        component.flat = component_ref.is_flat();
component.hide_slider = component_ref.is_hiding_slider();
component.label = component_ref.label().to_string();
component.read_only = component_ref.is_read_only();
    }
}