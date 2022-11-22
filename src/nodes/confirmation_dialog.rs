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

pub struct ConfirmationDialogPlugin;

impl Plugin for ConfirmationDialogPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ConfirmationDialog>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a confirmation_dialog.
pub fn is_confirmation_dialog(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ConfirmationDialog>().is_some()
}

/// A bundle for ConfirmationDialogs.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDConfirmationDialogBundle {
    pub confirmation_dialog: GDConfirmationDialog,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub popup: GDPopup,
pub window_dialog: GDWindowDialog,
pub accept_dialog: GDAcceptDialog,
    pub true_type: TrueNodeType,
}

impl Default for GDConfirmationDialogBundle {
    fn default() -> Self {
        Self {
            confirmation_dialog: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
popup: Default::default(),
window_dialog: Default::default(),
accept_dialog: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ConfirmationDialog".to_string()
            }
        }
    }
}

/// Represents a ConfirmationDialog.
#[derive(Component)]
pub struct GDConfirmationDialog {
    
}

impl Default for GDConfirmationDialog {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDConfirmationDialog {
    type Parent = GDAcceptDialog;
    type GodotClass = ConfirmationDialog;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ConfirmationDialog>().unwrap();
        world_commands.insert(entity, GDConfirmationDialog {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDConfirmationDialog {
    
}

fn sync_bevy_owned(query: Query<(&GDConfirmationDialog, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ConfirmationDialog>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDConfirmationDialog, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ConfirmationDialog>().unwrap();
        
    }
}