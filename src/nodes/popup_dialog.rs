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

pub struct PopupDialogPlugin;

impl Plugin for PopupDialogPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<PopupDialog>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a popup_dialog.
pub fn is_popup_dialog(node: &gdnative::prelude::Node) -> bool {
    node.cast::<PopupDialog>().is_some()
}

/// A bundle for PopupDialogs.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDPopupDialogBundle {
    pub popup_dialog: GDPopupDialog,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub popup: GDPopup,
    pub true_type: TrueNodeType,
}

impl Default for GDPopupDialogBundle {
    fn default() -> Self {
        Self {
            popup_dialog: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
popup: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "PopupDialog".to_string()
            }
        }
    }
}

/// Represents a PopupDialog.
#[derive(Component)]
pub struct GDPopupDialog {
    
}

impl Default for GDPopupDialog {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDPopupDialog {
    type Parent = GDPopup;
    type GodotClass = PopupDialog;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<PopupDialog>().unwrap();
        world_commands.insert(entity, GDPopupDialog {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDPopupDialog {
    
}

fn sync_bevy_owned(query: Query<(&GDPopupDialog, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PopupDialog>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDPopupDialog, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PopupDialog>().unwrap();
        
    }
}