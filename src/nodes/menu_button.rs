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

pub struct MenuButtonPlugin;

impl Plugin for MenuButtonPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<MenuButton>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a menu_button.
pub fn is_menu_button(node: &gdnative::prelude::Node) -> bool {
    node.cast::<MenuButton>().is_some()
}

/// A bundle for MenuButtons.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDMenuButtonBundle {
    pub menu_button: GDMenuButton,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub base_button: GDBaseButton,
pub button: GDButton,
    pub true_type: TrueNodeType,
}

impl Default for GDMenuButtonBundle {
    fn default() -> Self {
        Self {
            menu_button: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
base_button: Default::default(),
button: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "MenuButton".to_string()
            }
        }
    }
}

/// Represents a MenuButton.
#[derive(Component)]
pub struct GDMenuButton {
    pub switch_on_hover: bool,
}

impl Default for GDMenuButton {
    fn default() -> Self {
        Self {
            switch_on_hover: Default::default(),
        }
    }
}

impl NodeClass for GDMenuButton {
    type Parent = GDButton;
    type GodotClass = MenuButton;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<MenuButton>().unwrap();
        world_commands.insert(entity, GDMenuButton {
            switch_on_hover: component_ref.is_switch_on_hover(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDMenuButton {
    
}

fn sync_bevy_owned(query: Query<(&GDMenuButton, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<MenuButton>().unwrap();
        component_ref.set_switch_on_hover(component.switch_on_hover);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDMenuButton, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<MenuButton>().unwrap();
        component.switch_on_hover = component_ref.is_switch_on_hover();
    }
}