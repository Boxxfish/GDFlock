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

pub struct FileDialogPlugin;

impl Plugin for FileDialogPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<FileDialog>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a file_dialog.
pub fn is_file_dialog(node: &gdnative::prelude::Node) -> bool {
    node.cast::<FileDialog>().is_some()
}

/// A bundle for FileDialogs.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDFileDialogBundle {
    pub file_dialog: GDFileDialog,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub popup: GDPopup,
pub window_dialog: GDWindowDialog,
pub accept_dialog: GDAcceptDialog,
pub confirmation_dialog: GDConfirmationDialog,
    pub true_type: TrueNodeType,
}

impl Default for GDFileDialogBundle {
    fn default() -> Self {
        Self {
            file_dialog: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
popup: Default::default(),
window_dialog: Default::default(),
accept_dialog: Default::default(),
confirmation_dialog: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "FileDialog".to_string()
            }
        }
    }
}

/// Represents a FileDialog.
#[derive(Component)]
pub struct GDFileDialog {
    pub current_dir: String,
pub current_file: String,
pub current_path: String,
pub filters: Vec<String>,
pub mode_overrides_title: bool,
pub show_hidden_files: bool,
}

impl Default for GDFileDialog {
    fn default() -> Self {
        Self {
            current_dir: Default::default(),
current_file: Default::default(),
current_path: Default::default(),
filters: Default::default(),
mode_overrides_title: Default::default(),
show_hidden_files: Default::default(),
        }
    }
}

impl NodeClass for GDFileDialog {
    type Parent = GDConfirmationDialog;
    type GodotClass = FileDialog;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<FileDialog>().unwrap();
        world_commands.insert(entity, GDFileDialog {
            current_dir: component_ref.current_dir().to_string(),
current_file: component_ref.current_file().to_string(),
current_path: component_ref.current_path().to_string(),
filters: component_ref.filters().to_vec().iter().map(|x| x.to_string()).collect(),
mode_overrides_title: component_ref.is_mode_overriding_title(),
show_hidden_files: component_ref.is_showing_hidden_files(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDFileDialog {
    
}

fn sync_bevy_owned(query: Query<(&GDFileDialog, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<FileDialog>().unwrap();
        component_ref.set_current_dir(component.current_dir.clone());
component_ref.set_current_file(component.current_file.clone());
component_ref.set_current_path(component.current_path.clone());
component_ref.set_filters(StringArray::from_vec(component.filters.iter().map(GodotString::from_str).collect()));
component_ref.set_mode_overrides_title(component.mode_overrides_title);
component_ref.set_show_hidden_files(component.show_hidden_files);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDFileDialog, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<FileDialog>().unwrap();
        component.current_dir = component_ref.current_dir().to_string();
component.current_file = component_ref.current_file().to_string();
component.current_path = component_ref.current_path().to_string();
component.filters = component_ref.filters().to_vec().iter().map(|x| x.to_string()).collect();
component.mode_overrides_title = component_ref.is_mode_overriding_title();
component.show_hidden_files = component_ref.is_showing_hidden_files();
    }
}