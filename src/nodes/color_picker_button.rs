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

pub struct ColorPickerButtonPlugin;

impl Plugin for ColorPickerButtonPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ColorPickerButton>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a color_picker_button.
pub fn is_color_picker_button(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ColorPickerButton>().is_some()
}

/// A bundle for ColorPickerButtons.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDColorPickerButtonBundle {
    pub color_picker_button: GDColorPickerButton,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub base_button: GDBaseButton,
pub button: GDButton,
    pub true_type: TrueNodeType,
}

impl Default for GDColorPickerButtonBundle {
    fn default() -> Self {
        Self {
            color_picker_button: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
base_button: Default::default(),
button: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ColorPickerButton".to_string()
            }
        }
    }
}

/// Represents a ColorPickerButton.
#[derive(Component)]
pub struct GDColorPickerButton {
    pub color: Color,
pub edit_alpha: bool,
}

impl Default for GDColorPickerButton {
    fn default() -> Self {
        Self {
            color: Color::from_rgb(0.0, 0.0, 0.0),
edit_alpha: Default::default(),
        }
    }
}

impl NodeClass for GDColorPickerButton {
    type Parent = GDButton;
    type GodotClass = ColorPickerButton;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ColorPickerButton>().unwrap();
        world_commands.insert(entity, GDColorPickerButton {
            color: component_ref.pick_color(),
edit_alpha: component_ref.is_editing_alpha(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDColorPickerButton {
    
}

fn sync_bevy_owned(query: Query<(&GDColorPickerButton, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ColorPickerButton>().unwrap();
        component_ref.set_pick_color(component.color);
component_ref.set_edit_alpha(component.edit_alpha);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDColorPickerButton, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ColorPickerButton>().unwrap();
        component.color = component_ref.pick_color();
component.edit_alpha = component_ref.is_editing_alpha();
    }
}