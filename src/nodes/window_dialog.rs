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

pub struct WindowDialogPlugin;

impl Plugin for WindowDialogPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<WindowDialog>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a window_dialog.
pub fn is_window_dialog(node: &gdnative::prelude::Node) -> bool {
    node.cast::<WindowDialog>().is_some()
}

/// A bundle for WindowDialogs.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDWindowDialogBundle {
    pub window_dialog: GDWindowDialog,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub popup: GDPopup,
    pub true_type: TrueNodeType,
}

impl Default for GDWindowDialogBundle {
    fn default() -> Self {
        Self {
            window_dialog: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
popup: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "WindowDialog".to_string()
            }
        }
    }
}

/// Represents a WindowDialog.
#[derive(Component)]
pub struct GDWindowDialog {
    pub resizable: bool,
pub window_title: String,
}

impl Default for GDWindowDialog {
    fn default() -> Self {
        Self {
            resizable: Default::default(),
window_title: Default::default(),
        }
    }
}

impl NodeClass for GDWindowDialog {
    type Parent = GDPopup;
    type GodotClass = WindowDialog;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<WindowDialog>().unwrap();
        world_commands.insert(entity, GDWindowDialog {
            resizable: component_ref.resizable(),
window_title: component_ref.title().to_string(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDWindowDialog {
    
}

fn sync_bevy_owned(query: Query<(&GDWindowDialog, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<WindowDialog>().unwrap();
        component_ref.set_resizable(component.resizable);
component_ref.set_title(component.window_title.clone());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDWindowDialog, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<WindowDialog>().unwrap();
        component.resizable = component_ref.resizable();
component.window_title = component_ref.title().to_string();
    }
}