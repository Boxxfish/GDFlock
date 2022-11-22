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

pub struct EditorFileSystemPlugin;

impl Plugin for EditorFileSystemPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a editor_file_system.
pub fn is_editor_file_system(node: &gdnative::prelude::Node) -> bool {
    node.cast::<EditorFileSystem>().is_some()
}

/// A bundle for EditorFileSystems.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDEditorFileSystemBundle {
    pub editor_file_system: GDEditorFileSystem,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDEditorFileSystemBundle {
    fn default() -> Self {
        Self {
            editor_file_system: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "EditorFileSystem".to_string()
            }
        }
    }
}

/// Represents a EditorFileSystem.
#[derive(Component)]
pub struct GDEditorFileSystem {
    
}

impl Default for GDEditorFileSystem {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDEditorFileSystem {
    type Parent = GDNode;
    type GodotClass = EditorFileSystem;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<EditorFileSystem>().unwrap();
        world_commands.insert(entity, GDEditorFileSystem {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDEditorFileSystem {
    
}

fn sync_bevy_owned(query: Query<(&GDEditorFileSystem, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<EditorFileSystem>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDEditorFileSystem, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<EditorFileSystem>().unwrap();
        
    }
}