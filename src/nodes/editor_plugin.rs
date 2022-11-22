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

pub struct EditorPluginPlugin;

impl Plugin for EditorPluginPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a editor_plugin.
pub fn is_editor_plugin(node: &gdnative::prelude::Node) -> bool {
    node.cast::<EditorPlugin>().is_some()
}

/// A bundle for EditorPlugins.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDEditorPluginBundle {
    pub editor_plugin: GDEditorPlugin,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDEditorPluginBundle {
    fn default() -> Self {
        Self {
            editor_plugin: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "EditorPlugin".to_string()
            }
        }
    }
}

/// Represents a EditorPlugin.
#[derive(Component)]
pub struct GDEditorPlugin {
    
}

impl Default for GDEditorPlugin {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDEditorPlugin {
    type Parent = GDNode;
    type GodotClass = EditorPlugin;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<EditorPlugin>().unwrap();
        world_commands.insert(entity, GDEditorPlugin {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDEditorPlugin {
    
}

fn sync_bevy_owned(query: Query<(&GDEditorPlugin, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<EditorPlugin>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDEditorPlugin, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<EditorPlugin>().unwrap();
        
    }
}