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

pub struct PopupMenuPlugin;

impl Plugin for PopupMenuPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<PopupMenu>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a popup_menu.
pub fn is_popup_menu(node: &gdnative::prelude::Node) -> bool {
    node.cast::<PopupMenu>().is_some()
}

/// A bundle for PopupMenus.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDPopupMenuBundle {
    pub popup_menu: GDPopupMenu,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub popup: GDPopup,
    pub true_type: TrueNodeType,
}

impl Default for GDPopupMenuBundle {
    fn default() -> Self {
        Self {
            popup_menu: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
popup: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "PopupMenu".to_string()
            }
        }
    }
}

/// Represents a PopupMenu.
#[derive(Component)]
pub struct GDPopupMenu {
    pub allow_search: bool,
pub hide_on_checkable_item_selection: bool,
pub hide_on_item_selection: bool,
pub hide_on_state_item_selection: bool,
pub submenu_popup_delay: f64,
}

impl Default for GDPopupMenu {
    fn default() -> Self {
        Self {
            allow_search: Default::default(),
hide_on_checkable_item_selection: Default::default(),
hide_on_item_selection: Default::default(),
hide_on_state_item_selection: Default::default(),
submenu_popup_delay: Default::default(),
        }
    }
}

impl NodeClass for GDPopupMenu {
    type Parent = GDPopup;
    type GodotClass = PopupMenu;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<PopupMenu>().unwrap();
        world_commands.insert(entity, GDPopupMenu {
            allow_search: component_ref.allow_search(),
hide_on_checkable_item_selection: component_ref.is_hide_on_checkable_item_selection(),
hide_on_item_selection: component_ref.is_hide_on_item_selection(),
hide_on_state_item_selection: component_ref.is_hide_on_state_item_selection(),
submenu_popup_delay: component_ref.submenu_popup_delay(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDPopupMenu {
    
}

fn sync_bevy_owned(query: Query<(&GDPopupMenu, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PopupMenu>().unwrap();
        component_ref.set_allow_search(component.allow_search);
component_ref.set_hide_on_checkable_item_selection(component.hide_on_checkable_item_selection);
component_ref.set_hide_on_item_selection(component.hide_on_item_selection);
component_ref.set_hide_on_state_item_selection(component.hide_on_state_item_selection);
component_ref.set_submenu_popup_delay(component.submenu_popup_delay);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDPopupMenu, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PopupMenu>().unwrap();
        component.allow_search = component_ref.allow_search();
component.hide_on_checkable_item_selection = component_ref.is_hide_on_checkable_item_selection();
component.hide_on_item_selection = component_ref.is_hide_on_item_selection();
component.hide_on_state_item_selection = component_ref.is_hide_on_state_item_selection();
component.submenu_popup_delay = component_ref.submenu_popup_delay();
    }
}