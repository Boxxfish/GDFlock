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

pub struct EditorInterfacePlugin;

impl Plugin for EditorInterfacePlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a editor_interface.
pub fn is_editor_interface(node: &gdnative::prelude::Node) -> bool {
    node.cast::<EditorInterface>().is_some()
}

/// A bundle for EditorInterfaces.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDEditorInterfaceBundle {
    pub editor_interface: GDEditorInterface,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDEditorInterfaceBundle {
    fn default() -> Self {
        Self {
            editor_interface: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "EditorInterface".to_string()
            }
        }
    }
}

/// Represents a EditorInterface.
#[derive(Component)]
pub struct GDEditorInterface {
    pub distraction_free_mode: bool,
}

impl Default for GDEditorInterface {
    fn default() -> Self {
        Self {
            distraction_free_mode: Default::default(),
        }
    }
}

impl NodeClass for GDEditorInterface {
    type Parent = GDNode;
    type GodotClass = EditorInterface;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<EditorInterface>().unwrap();
        world_commands.insert(entity, GDEditorInterface {
            distraction_free_mode: component_ref.is_distraction_free_mode_enabled(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDEditorInterface {
    
}

fn sync_bevy_owned(query: Query<(&GDEditorInterface, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<EditorInterface>().unwrap();
        component_ref.set_distraction_free_mode(component.distraction_free_mode);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDEditorInterface, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<EditorInterface>().unwrap();
        component.distraction_free_mode = component_ref.is_distraction_free_mode_enabled();
    }
}