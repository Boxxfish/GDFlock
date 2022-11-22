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

pub struct BaseButtonPlugin;

impl Plugin for BaseButtonPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a base_button.
pub fn is_base_button(node: &gdnative::prelude::Node) -> bool {
    node.cast::<BaseButton>().is_some()
}

/// A bundle for BaseButtons.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDBaseButtonBundle {
    pub base_button: GDBaseButton,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
    pub true_type: TrueNodeType,
}

impl Default for GDBaseButtonBundle {
    fn default() -> Self {
        Self {
            base_button: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "BaseButton".to_string()
            }
        }
    }
}

/// Represents a BaseButton.
#[derive(Component)]
pub struct GDBaseButton {
    pub button_mask: i64,
pub disabled: bool,
pub keep_pressed_outside: bool,
pub pressed: bool,
pub shortcut_in_tooltip: bool,
pub toggle_mode: bool,
}

impl Default for GDBaseButton {
    fn default() -> Self {
        Self {
            button_mask: Default::default(),
disabled: Default::default(),
keep_pressed_outside: Default::default(),
pressed: Default::default(),
shortcut_in_tooltip: Default::default(),
toggle_mode: Default::default(),
        }
    }
}

impl NodeClass for GDBaseButton {
    type Parent = GDControl;
    type GodotClass = BaseButton;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<BaseButton>().unwrap();
        world_commands.insert(entity, GDBaseButton {
            button_mask: component_ref.button_mask(),
disabled: component_ref.is_disabled(),
keep_pressed_outside: component_ref.is_keep_pressed_outside(),
pressed: component_ref.is_pressed(),
shortcut_in_tooltip: component_ref.is_shortcut_in_tooltip_enabled(),
toggle_mode: component_ref.is_toggle_mode(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDBaseButton {
    
}

fn sync_bevy_owned(query: Query<(&GDBaseButton, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<BaseButton>().unwrap();
        component_ref.set_button_mask(component.button_mask);
component_ref.set_disabled(component.disabled);
component_ref.set_keep_pressed_outside(component.keep_pressed_outside);
component_ref.set_pressed(component.pressed);
component_ref.set_shortcut_in_tooltip(component.shortcut_in_tooltip);
component_ref.set_toggle_mode(component.toggle_mode);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDBaseButton, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<BaseButton>().unwrap();
        component.button_mask = component_ref.button_mask();
component.disabled = component_ref.is_disabled();
component.keep_pressed_outside = component_ref.is_keep_pressed_outside();
component.pressed = component_ref.is_pressed();
component.shortcut_in_tooltip = component_ref.is_shortcut_in_tooltip_enabled();
component.toggle_mode = component_ref.is_toggle_mode();
    }
}