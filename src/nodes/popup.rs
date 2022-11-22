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

pub struct PopupPlugin;

impl Plugin for PopupPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Popup>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a popup.
pub fn is_popup(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Popup>().is_some()
}

/// A bundle for Popups.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDPopupBundle {
    pub popup: GDPopup,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
    pub true_type: TrueNodeType,
}

impl Default for GDPopupBundle {
    fn default() -> Self {
        Self {
            popup: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Popup".to_string()
            }
        }
    }
}

/// Represents a Popup.
#[derive(Component)]
pub struct GDPopup {
    pub popup_exclusive: bool,
}

impl Default for GDPopup {
    fn default() -> Self {
        Self {
            popup_exclusive: Default::default(),
        }
    }
}

impl NodeClass for GDPopup {
    type Parent = GDControl;
    type GodotClass = Popup;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Popup>().unwrap();
        world_commands.insert(entity, GDPopup {
            popup_exclusive: component_ref.is_exclusive(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDPopup {
    
}

fn sync_bevy_owned(query: Query<(&GDPopup, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Popup>().unwrap();
        component_ref.set_exclusive(component.popup_exclusive);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDPopup, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Popup>().unwrap();
        component.popup_exclusive = component_ref.is_exclusive();
    }
}