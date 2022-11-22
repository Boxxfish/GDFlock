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

pub struct TextEditPlugin;

impl Plugin for TextEditPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<TextEdit>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a text_edit.
pub fn is_text_edit(node: &gdnative::prelude::Node) -> bool {
    node.cast::<TextEdit>().is_some()
}

/// A bundle for TextEdits.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDTextEditBundle {
    pub text_edit: GDTextEdit,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
    pub true_type: TrueNodeType,
}

impl Default for GDTextEditBundle {
    fn default() -> Self {
        Self {
            text_edit: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "TextEdit".to_string()
            }
        }
    }
}

/// Represents a TextEdit.
#[derive(Component)]
pub struct GDTextEdit {
    pub bookmark_gutter: bool,
pub breakpoint_gutter: bool,
pub caret_blink: bool,
pub caret_blink_speed: f64,
pub caret_block_mode: bool,
pub caret_moving_by_right_click: bool,
pub context_menu_enabled: bool,
pub deselect_on_focus_loss_enabled: bool,
pub drag_and_drop_selection_enabled: bool,
pub draw_spaces: bool,
pub draw_tabs: bool,
pub fold_gutter: bool,
pub hiding_enabled: bool,
pub highlight_all_occurrences: bool,
pub highlight_current_line: bool,
pub middle_mouse_paste_enabled: bool,
pub minimap_draw: bool,
pub minimap_width: i64,
pub override_selected_font_color: bool,
pub readonly: bool,
pub scroll_horizontal: i64,
pub scroll_vertical: f64,
pub selecting_enabled: bool,
pub shortcut_keys_enabled: bool,
pub show_line_numbers: bool,
pub smooth_scrolling: bool,
pub syntax_highlighting: bool,
pub text: String,
pub v_scroll_speed: f64,
pub virtual_keyboard_enabled: bool,
pub wrap_enabled: bool,
}

impl Default for GDTextEdit {
    fn default() -> Self {
        Self {
            bookmark_gutter: Default::default(),
breakpoint_gutter: Default::default(),
caret_blink: Default::default(),
caret_blink_speed: Default::default(),
caret_block_mode: Default::default(),
caret_moving_by_right_click: Default::default(),
context_menu_enabled: Default::default(),
deselect_on_focus_loss_enabled: Default::default(),
drag_and_drop_selection_enabled: Default::default(),
draw_spaces: Default::default(),
draw_tabs: Default::default(),
fold_gutter: Default::default(),
hiding_enabled: Default::default(),
highlight_all_occurrences: Default::default(),
highlight_current_line: Default::default(),
middle_mouse_paste_enabled: Default::default(),
minimap_draw: Default::default(),
minimap_width: Default::default(),
override_selected_font_color: Default::default(),
readonly: Default::default(),
scroll_horizontal: Default::default(),
scroll_vertical: Default::default(),
selecting_enabled: Default::default(),
shortcut_keys_enabled: Default::default(),
show_line_numbers: Default::default(),
smooth_scrolling: Default::default(),
syntax_highlighting: Default::default(),
text: Default::default(),
v_scroll_speed: Default::default(),
virtual_keyboard_enabled: Default::default(),
wrap_enabled: Default::default(),
        }
    }
}

impl NodeClass for GDTextEdit {
    type Parent = GDControl;
    type GodotClass = TextEdit;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<TextEdit>().unwrap();
        world_commands.insert(entity, GDTextEdit {
            bookmark_gutter: component_ref.is_bookmark_gutter_enabled(),
breakpoint_gutter: component_ref.is_breakpoint_gutter_enabled(),
caret_blink: component_ref.cursor_get_blink_enabled(),
caret_blink_speed: component_ref.cursor_get_blink_speed(),
caret_block_mode: component_ref.cursor_is_block_mode(),
caret_moving_by_right_click: component_ref.is_right_click_moving_caret(),
context_menu_enabled: component_ref.is_context_menu_enabled(),
deselect_on_focus_loss_enabled: component_ref.is_deselect_on_focus_loss_enabled(),
drag_and_drop_selection_enabled: component_ref.is_drag_and_drop_selection_enabled(),
draw_spaces: component_ref.is_drawing_spaces(),
draw_tabs: component_ref.is_drawing_tabs(),
fold_gutter: component_ref.is_drawing_fold_gutter(),
hiding_enabled: component_ref.is_hiding_enabled(),
highlight_all_occurrences: component_ref.is_highlight_all_occurrences_enabled(),
highlight_current_line: component_ref.is_highlight_current_line_enabled(),
middle_mouse_paste_enabled: component_ref.is_middle_mouse_paste_enabled(),
minimap_draw: component_ref.is_drawing_minimap(),
minimap_width: component_ref.minimap_width(),
override_selected_font_color: component_ref.is_overriding_selected_font_color(),
readonly: component_ref.is_readonly(),
scroll_horizontal: component_ref.h_scroll(),
scroll_vertical: component_ref.v_scroll(),
selecting_enabled: component_ref.is_selecting_enabled(),
shortcut_keys_enabled: component_ref.is_shortcut_keys_enabled(),
show_line_numbers: component_ref.is_show_line_numbers_enabled(),
smooth_scrolling: component_ref.is_smooth_scroll_enabled(),
syntax_highlighting: component_ref.is_syntax_coloring_enabled(),
text: component_ref.text().to_string(),
v_scroll_speed: component_ref.v_scroll_speed(),
virtual_keyboard_enabled: component_ref.is_virtual_keyboard_enabled(),
wrap_enabled: component_ref.is_wrap_enabled(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDTextEdit {
    
}

fn sync_bevy_owned(query: Query<(&GDTextEdit, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<TextEdit>().unwrap();
        component_ref.set_bookmark_gutter_enabled(component.bookmark_gutter);
component_ref.set_breakpoint_gutter_enabled(component.breakpoint_gutter);
component_ref.cursor_set_blink_enabled(component.caret_blink);
component_ref.cursor_set_blink_speed(component.caret_blink_speed);
component_ref.cursor_set_block_mode(component.caret_block_mode);
component_ref.set_right_click_moves_caret(component.caret_moving_by_right_click);
component_ref.set_context_menu_enabled(component.context_menu_enabled);
component_ref.set_deselect_on_focus_loss_enabled(component.deselect_on_focus_loss_enabled);
component_ref.set_drag_and_drop_selection_enabled(component.drag_and_drop_selection_enabled);
component_ref.set_draw_spaces(component.draw_spaces);
component_ref.set_draw_tabs(component.draw_tabs);
component_ref.set_draw_fold_gutter(component.fold_gutter);
component_ref.set_hiding_enabled(component.hiding_enabled);
component_ref.set_highlight_all_occurrences(component.highlight_all_occurrences);
component_ref.set_highlight_current_line(component.highlight_current_line);
component_ref.set_middle_mouse_paste_enabled(component.middle_mouse_paste_enabled);
component_ref.draw_minimap(component.minimap_draw);
component_ref.set_minimap_width(component.minimap_width);
component_ref.set_override_selected_font_color(component.override_selected_font_color);
component_ref.set_readonly(component.readonly);
component_ref.set_h_scroll(component.scroll_horizontal);
component_ref.set_v_scroll(component.scroll_vertical);
component_ref.set_selecting_enabled(component.selecting_enabled);
component_ref.set_shortcut_keys_enabled(component.shortcut_keys_enabled);
component_ref.set_show_line_numbers(component.show_line_numbers);
component_ref.set_smooth_scroll_enable(component.smooth_scrolling);
component_ref.set_syntax_coloring(component.syntax_highlighting);
component_ref.set_text(component.text.clone());
component_ref.set_v_scroll_speed(component.v_scroll_speed);
component_ref.set_virtual_keyboard_enabled(component.virtual_keyboard_enabled);
component_ref.set_wrap_enabled(component.wrap_enabled);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDTextEdit, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<TextEdit>().unwrap();
        component.bookmark_gutter = component_ref.is_bookmark_gutter_enabled();
component.breakpoint_gutter = component_ref.is_breakpoint_gutter_enabled();
component.caret_blink = component_ref.cursor_get_blink_enabled();
component.caret_blink_speed = component_ref.cursor_get_blink_speed();
component.caret_block_mode = component_ref.cursor_is_block_mode();
component.caret_moving_by_right_click = component_ref.is_right_click_moving_caret();
component.context_menu_enabled = component_ref.is_context_menu_enabled();
component.deselect_on_focus_loss_enabled = component_ref.is_deselect_on_focus_loss_enabled();
component.drag_and_drop_selection_enabled = component_ref.is_drag_and_drop_selection_enabled();
component.draw_spaces = component_ref.is_drawing_spaces();
component.draw_tabs = component_ref.is_drawing_tabs();
component.fold_gutter = component_ref.is_drawing_fold_gutter();
component.hiding_enabled = component_ref.is_hiding_enabled();
component.highlight_all_occurrences = component_ref.is_highlight_all_occurrences_enabled();
component.highlight_current_line = component_ref.is_highlight_current_line_enabled();
component.middle_mouse_paste_enabled = component_ref.is_middle_mouse_paste_enabled();
component.minimap_draw = component_ref.is_drawing_minimap();
component.minimap_width = component_ref.minimap_width();
component.override_selected_font_color = component_ref.is_overriding_selected_font_color();
component.readonly = component_ref.is_readonly();
component.scroll_horizontal = component_ref.h_scroll();
component.scroll_vertical = component_ref.v_scroll();
component.selecting_enabled = component_ref.is_selecting_enabled();
component.shortcut_keys_enabled = component_ref.is_shortcut_keys_enabled();
component.show_line_numbers = component_ref.is_show_line_numbers_enabled();
component.smooth_scrolling = component_ref.is_smooth_scroll_enabled();
component.syntax_highlighting = component_ref.is_syntax_coloring_enabled();
component.text = component_ref.text().to_string();
component.v_scroll_speed = component_ref.v_scroll_speed();
component.virtual_keyboard_enabled = component_ref.is_virtual_keyboard_enabled();
component.wrap_enabled = component_ref.is_wrap_enabled();
    }
}