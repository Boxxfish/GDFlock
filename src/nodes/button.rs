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

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Button>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a button.
pub fn is_button(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Button>().is_some()
}

/// A bundle for Buttons.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDButtonBundle {
    pub button: GDButton,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub base_button: GDBaseButton,
    pub true_type: TrueNodeType,
}

impl Default for GDButtonBundle {
    fn default() -> Self {
        Self {
            button: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
base_button: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Button".to_string()
            }
        }
    }
}

/// Represents a Button.
#[derive(Component)]
pub struct GDButton {
    pub clip_text: bool,
pub expand_icon: bool,
pub flat: bool,
pub icon: Option<Ref<Texture>>,
pub text: String,
}

impl Default for GDButton {
    fn default() -> Self {
        Self {
            clip_text: Default::default(),
expand_icon: Default::default(),
flat: Default::default(),
icon: Default::default(),
text: Default::default(),
        }
    }
}

impl NodeClass for GDButton {
    type Parent = GDBaseButton;
    type GodotClass = Button;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Button>().unwrap();
        world_commands.insert(entity, GDButton {
            clip_text: component_ref.clip_text(),
expand_icon: component_ref.is_expand_icon(),
flat: component_ref.is_flat(),
icon: component_ref.button_icon(),
text: component_ref.text().to_string(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDButton {
    
}

fn sync_bevy_owned(query: Query<(&GDButton, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Button>().unwrap();
        component_ref.set_clip_text(component.clip_text);
component_ref.set_expand_icon(component.expand_icon);
component_ref.set_flat(component.flat);
component_ref.set_button_icon(component.icon.as_ref().unwrap().clone());
component_ref.set_text(component.text.clone());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDButton, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Button>().unwrap();
        component.clip_text = component_ref.clip_text();
component.expand_icon = component_ref.is_expand_icon();
component.flat = component_ref.is_flat();
component.icon = component_ref.button_icon();
component.text = component_ref.text().to_string();
    }
}