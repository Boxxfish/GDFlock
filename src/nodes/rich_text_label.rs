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

pub struct RichTextLabelPlugin;

impl Plugin for RichTextLabelPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<RichTextLabel>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a rich_text_label.
pub fn is_rich_text_label(node: &gdnative::prelude::Node) -> bool {
    node.cast::<RichTextLabel>().is_some()
}

/// A bundle for RichTextLabels.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDRichTextLabelBundle {
    pub rich_text_label: GDRichTextLabel,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
    pub true_type: TrueNodeType,
}

impl Default for GDRichTextLabelBundle {
    fn default() -> Self {
        Self {
            rich_text_label: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "RichTextLabel".to_string()
            }
        }
    }
}

/// Represents a RichTextLabel.
#[derive(Component)]
pub struct GDRichTextLabel {
    pub bbcode_enabled: bool,
pub bbcode_text: String,
pub deselect_on_focus_loss_enabled: bool,
pub fit_content_height: bool,
pub meta_underlined: bool,
pub override_selected_font_color: bool,
pub percent_visible: f64,
pub scroll_active: bool,
pub scroll_following: bool,
pub selection_enabled: bool,
pub tab_size: i64,
pub text: String,
pub visible_characters: i64,
}

impl Default for GDRichTextLabel {
    fn default() -> Self {
        Self {
            bbcode_enabled: Default::default(),
bbcode_text: Default::default(),
deselect_on_focus_loss_enabled: Default::default(),
fit_content_height: Default::default(),
meta_underlined: Default::default(),
override_selected_font_color: Default::default(),
percent_visible: Default::default(),
scroll_active: Default::default(),
scroll_following: Default::default(),
selection_enabled: Default::default(),
tab_size: Default::default(),
text: Default::default(),
visible_characters: Default::default(),
        }
    }
}

impl NodeClass for GDRichTextLabel {
    type Parent = GDControl;
    type GodotClass = RichTextLabel;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<RichTextLabel>().unwrap();
        world_commands.insert(entity, GDRichTextLabel {
            bbcode_enabled: component_ref.is_using_bbcode(),
bbcode_text: component_ref.bbcode().to_string(),
deselect_on_focus_loss_enabled: component_ref.is_deselect_on_focus_loss_enabled(),
fit_content_height: component_ref.is_fit_content_height_enabled(),
meta_underlined: component_ref.is_meta_underlined(),
override_selected_font_color: component_ref.is_overriding_selected_font_color(),
percent_visible: component_ref.percent_visible(),
scroll_active: component_ref.is_scroll_active(),
scroll_following: component_ref.is_scroll_following(),
selection_enabled: component_ref.is_selection_enabled(),
tab_size: component_ref.tab_size(),
text: component_ref.text().to_string(),
visible_characters: component_ref.visible_characters(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDRichTextLabel {
    
}

fn sync_bevy_owned(query: Query<(&GDRichTextLabel, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<RichTextLabel>().unwrap();
        component_ref.set_use_bbcode(component.bbcode_enabled);
component_ref.set_bbcode(component.bbcode_text.clone());
component_ref.set_deselect_on_focus_loss_enabled(component.deselect_on_focus_loss_enabled);
component_ref.set_fit_content_height(component.fit_content_height);
component_ref.set_meta_underline(component.meta_underlined);
component_ref.set_override_selected_font_color(component.override_selected_font_color);
component_ref.set_percent_visible(component.percent_visible);
component_ref.set_scroll_active(component.scroll_active);
component_ref.set_scroll_follow(component.scroll_following);
component_ref.set_selection_enabled(component.selection_enabled);
component_ref.set_tab_size(component.tab_size);
component_ref.set_text(component.text.clone());
component_ref.set_visible_characters(component.visible_characters);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDRichTextLabel, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<RichTextLabel>().unwrap();
        component.bbcode_enabled = component_ref.is_using_bbcode();
component.bbcode_text = component_ref.bbcode().to_string();
component.deselect_on_focus_loss_enabled = component_ref.is_deselect_on_focus_loss_enabled();
component.fit_content_height = component_ref.is_fit_content_height_enabled();
component.meta_underlined = component_ref.is_meta_underlined();
component.override_selected_font_color = component_ref.is_overriding_selected_font_color();
component.percent_visible = component_ref.percent_visible();
component.scroll_active = component_ref.is_scroll_active();
component.scroll_following = component_ref.is_scroll_following();
component.selection_enabled = component_ref.is_selection_enabled();
component.tab_size = component_ref.tab_size();
component.text = component_ref.text().to_string();
component.visible_characters = component_ref.visible_characters();
    }
}