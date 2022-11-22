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

pub struct LineEditPlugin;

impl Plugin for LineEditPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<LineEdit>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a line_edit.
pub fn is_line_edit(node: &gdnative::prelude::Node) -> bool {
    node.cast::<LineEdit>().is_some()
}

/// A bundle for LineEdits.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDLineEditBundle {
    pub line_edit: GDLineEdit,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
    pub true_type: TrueNodeType,
}

impl Default for GDLineEditBundle {
    fn default() -> Self {
        Self {
            line_edit: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "LineEdit".to_string()
            }
        }
    }
}

/// Represents a LineEdit.
#[derive(Component)]
pub struct GDLineEdit {
    pub caret_blink: bool,
pub caret_blink_speed: f64,
pub caret_position: i64,
pub clear_button_enabled: bool,
pub context_menu_enabled: bool,
pub deselect_on_focus_loss_enabled: bool,
pub editable: bool,
pub expand_to_text_length: bool,
pub max_length: i64,
pub middle_mouse_paste_enabled: bool,
pub placeholder_alpha: f64,
pub placeholder_text: String,
pub right_icon: Option<Ref<Texture>>,
pub secret: bool,
pub secret_character: String,
pub selecting_enabled: bool,
pub shortcut_keys_enabled: bool,
pub text: String,
pub virtual_keyboard_enabled: bool,
}

impl Default for GDLineEdit {
    fn default() -> Self {
        Self {
            caret_blink: Default::default(),
caret_blink_speed: Default::default(),
caret_position: Default::default(),
clear_button_enabled: Default::default(),
context_menu_enabled: Default::default(),
deselect_on_focus_loss_enabled: Default::default(),
editable: Default::default(),
expand_to_text_length: Default::default(),
max_length: Default::default(),
middle_mouse_paste_enabled: Default::default(),
placeholder_alpha: Default::default(),
placeholder_text: Default::default(),
right_icon: Default::default(),
secret: Default::default(),
secret_character: Default::default(),
selecting_enabled: Default::default(),
shortcut_keys_enabled: Default::default(),
text: Default::default(),
virtual_keyboard_enabled: Default::default(),
        }
    }
}

impl NodeClass for GDLineEdit {
    type Parent = GDControl;
    type GodotClass = LineEdit;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<LineEdit>().unwrap();
        world_commands.insert(entity, GDLineEdit {
            caret_blink: component_ref.cursor_get_blink_enabled(),
caret_blink_speed: component_ref.cursor_get_blink_speed(),
caret_position: component_ref.cursor_position(),
clear_button_enabled: component_ref.is_clear_button_enabled(),
context_menu_enabled: component_ref.is_context_menu_enabled(),
deselect_on_focus_loss_enabled: component_ref.is_deselect_on_focus_loss_enabled(),
editable: component_ref.is_editable(),
expand_to_text_length: component_ref.expand_to_text_length(),
max_length: component_ref.max_length(),
middle_mouse_paste_enabled: component_ref.is_middle_mouse_paste_enabled(),
placeholder_alpha: component_ref.placeholder_alpha(),
placeholder_text: component_ref.placeholder().to_string(),
right_icon: component_ref.right_icon(),
secret: component_ref.is_secret(),
secret_character: component_ref.secret_character().to_string(),
selecting_enabled: component_ref.is_selecting_enabled(),
shortcut_keys_enabled: component_ref.is_shortcut_keys_enabled(),
text: component_ref.text().to_string(),
virtual_keyboard_enabled: component_ref.is_virtual_keyboard_enabled(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDLineEdit {
    
}

fn sync_bevy_owned(query: Query<(&GDLineEdit, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<LineEdit>().unwrap();
        component_ref.cursor_set_blink_enabled(component.caret_blink);
component_ref.cursor_set_blink_speed(component.caret_blink_speed);
component_ref.set_cursor_position(component.caret_position);
component_ref.set_clear_button_enabled(component.clear_button_enabled);
component_ref.set_context_menu_enabled(component.context_menu_enabled);
component_ref.set_deselect_on_focus_loss_enabled(component.deselect_on_focus_loss_enabled);
component_ref.set_editable(component.editable);
component_ref.set_expand_to_text_length(component.expand_to_text_length);
component_ref.set_max_length(component.max_length);
component_ref.set_middle_mouse_paste_enabled(component.middle_mouse_paste_enabled);
component_ref.set_placeholder_alpha(component.placeholder_alpha);
component_ref.set_placeholder(component.placeholder_text.clone());
component_ref.set_right_icon(component.right_icon.as_ref().unwrap().clone());
component_ref.set_secret(component.secret);
component_ref.set_secret_character(component.secret_character.clone());
component_ref.set_selecting_enabled(component.selecting_enabled);
component_ref.set_shortcut_keys_enabled(component.shortcut_keys_enabled);
component_ref.set_text(component.text.clone());
component_ref.set_virtual_keyboard_enabled(component.virtual_keyboard_enabled);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDLineEdit, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<LineEdit>().unwrap();
        component.caret_blink = component_ref.cursor_get_blink_enabled();
component.caret_blink_speed = component_ref.cursor_get_blink_speed();
component.caret_position = component_ref.cursor_position();
component.clear_button_enabled = component_ref.is_clear_button_enabled();
component.context_menu_enabled = component_ref.is_context_menu_enabled();
component.deselect_on_focus_loss_enabled = component_ref.is_deselect_on_focus_loss_enabled();
component.editable = component_ref.is_editable();
component.expand_to_text_length = component_ref.expand_to_text_length();
component.max_length = component_ref.max_length();
component.middle_mouse_paste_enabled = component_ref.is_middle_mouse_paste_enabled();
component.placeholder_alpha = component_ref.placeholder_alpha();
component.placeholder_text = component_ref.placeholder().to_string();
component.right_icon = component_ref.right_icon();
component.secret = component_ref.is_secret();
component.secret_character = component_ref.secret_character().to_string();
component.selecting_enabled = component_ref.is_selecting_enabled();
component.shortcut_keys_enabled = component_ref.is_shortcut_keys_enabled();
component.text = component_ref.text().to_string();
component.virtual_keyboard_enabled = component_ref.is_virtual_keyboard_enabled();
    }
}