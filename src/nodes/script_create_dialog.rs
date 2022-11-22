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

pub struct ScriptCreateDialogPlugin;

impl Plugin for ScriptCreateDialogPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a script_create_dialog.
pub fn is_script_create_dialog(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ScriptCreateDialog>().is_some()
}

/// A bundle for ScriptCreateDialogs.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDScriptCreateDialogBundle {
    pub script_create_dialog: GDScriptCreateDialog,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub popup: GDPopup,
pub window_dialog: GDWindowDialog,
pub accept_dialog: GDAcceptDialog,
pub confirmation_dialog: GDConfirmationDialog,
    pub true_type: TrueNodeType,
}

impl Default for GDScriptCreateDialogBundle {
    fn default() -> Self {
        Self {
            script_create_dialog: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
popup: Default::default(),
window_dialog: Default::default(),
accept_dialog: Default::default(),
confirmation_dialog: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ScriptCreateDialog".to_string()
            }
        }
    }
}

/// Represents a ScriptCreateDialog.
#[derive(Component)]
pub struct GDScriptCreateDialog {
    
}

impl Default for GDScriptCreateDialog {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDScriptCreateDialog {
    type Parent = GDConfirmationDialog;
    type GodotClass = ScriptCreateDialog;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ScriptCreateDialog>().unwrap();
        world_commands.insert(entity, GDScriptCreateDialog {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDScriptCreateDialog {
    
}

fn sync_bevy_owned(query: Query<(&GDScriptCreateDialog, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ScriptCreateDialog>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDScriptCreateDialog, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ScriptCreateDialog>().unwrap();
        
    }
}