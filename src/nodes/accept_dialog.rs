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

pub struct AcceptDialogPlugin;

impl Plugin for AcceptDialogPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<AcceptDialog>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a accept_dialog.
pub fn is_accept_dialog(node: &gdnative::prelude::Node) -> bool {
    node.cast::<AcceptDialog>().is_some()
}

/// A bundle for AcceptDialogs.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDAcceptDialogBundle {
    pub accept_dialog: GDAcceptDialog,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub popup: GDPopup,
pub window_dialog: GDWindowDialog,
    pub true_type: TrueNodeType,
}

impl Default for GDAcceptDialogBundle {
    fn default() -> Self {
        Self {
            accept_dialog: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
popup: Default::default(),
window_dialog: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "AcceptDialog".to_string()
            }
        }
    }
}

/// Represents a AcceptDialog.
#[derive(Component)]
pub struct GDAcceptDialog {
    pub dialog_autowrap: bool,
pub dialog_hide_on_ok: bool,
pub dialog_text: String,
}

impl Default for GDAcceptDialog {
    fn default() -> Self {
        Self {
            dialog_autowrap: Default::default(),
dialog_hide_on_ok: Default::default(),
dialog_text: Default::default(),
        }
    }
}

impl NodeClass for GDAcceptDialog {
    type Parent = GDWindowDialog;
    type GodotClass = AcceptDialog;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<AcceptDialog>().unwrap();
        world_commands.insert(entity, GDAcceptDialog {
            dialog_autowrap: component_ref.has_autowrap(),
dialog_hide_on_ok: component_ref.hide_on_ok(),
dialog_text: component_ref.text().to_string(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDAcceptDialog {
    
}

fn sync_bevy_owned(query: Query<(&GDAcceptDialog, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AcceptDialog>().unwrap();
        component_ref.set_autowrap(component.dialog_autowrap);
component_ref.set_hide_on_ok(component.dialog_hide_on_ok);
component_ref.set_text(component.dialog_text.clone());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDAcceptDialog, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AcceptDialog>().unwrap();
        component.dialog_autowrap = component_ref.has_autowrap();
component.dialog_hide_on_ok = component_ref.hide_on_ok();
component.dialog_text = component_ref.text().to_string();
    }
}