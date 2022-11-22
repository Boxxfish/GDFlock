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

pub struct LabelPlugin;

impl Plugin for LabelPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Label>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a label.
pub fn is_label(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Label>().is_some()
}

/// A bundle for Labels.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDLabelBundle {
    pub label: GDLabel,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
    pub true_type: TrueNodeType,
}

impl Default for GDLabelBundle {
    fn default() -> Self {
        Self {
            label: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Label".to_string()
            }
        }
    }
}

/// Represents a Label.
#[derive(Component)]
pub struct GDLabel {
    pub autowrap: bool,
pub clip_text: bool,
pub lines_skipped: i64,
pub max_lines_visible: i64,
pub percent_visible: f64,
pub text: String,
pub uppercase: bool,
pub visible_characters: i64,
}

impl Default for GDLabel {
    fn default() -> Self {
        Self {
            autowrap: Default::default(),
clip_text: Default::default(),
lines_skipped: Default::default(),
max_lines_visible: Default::default(),
percent_visible: Default::default(),
text: Default::default(),
uppercase: Default::default(),
visible_characters: Default::default(),
        }
    }
}

impl NodeClass for GDLabel {
    type Parent = GDControl;
    type GodotClass = Label;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Label>().unwrap();
        world_commands.insert(entity, GDLabel {
            autowrap: component_ref.has_autowrap(),
clip_text: component_ref.is_clipping_text(),
lines_skipped: component_ref.lines_skipped(),
max_lines_visible: component_ref.max_lines_visible(),
percent_visible: component_ref.percent_visible(),
text: component_ref.text().to_string(),
uppercase: component_ref.is_uppercase(),
visible_characters: component_ref.visible_characters(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDLabel {
    
}

fn sync_bevy_owned(query: Query<(&GDLabel, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Label>().unwrap();
        component_ref.set_autowrap(component.autowrap);
component_ref.set_clip_text(component.clip_text);
component_ref.set_lines_skipped(component.lines_skipped);
component_ref.set_max_lines_visible(component.max_lines_visible);
component_ref.set_percent_visible(component.percent_visible);
component_ref.set_text(component.text.clone());
component_ref.set_uppercase(component.uppercase);
component_ref.set_visible_characters(component.visible_characters);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDLabel, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Label>().unwrap();
        component.autowrap = component_ref.has_autowrap();
component.clip_text = component_ref.is_clipping_text();
component.lines_skipped = component_ref.lines_skipped();
component.max_lines_visible = component_ref.max_lines_visible();
component.percent_visible = component_ref.percent_visible();
component.text = component_ref.text().to_string();
component.uppercase = component_ref.is_uppercase();
component.visible_characters = component_ref.visible_characters();
    }
}