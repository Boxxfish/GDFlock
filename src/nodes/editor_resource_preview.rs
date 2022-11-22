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

pub struct EditorResourcePreviewPlugin;

impl Plugin for EditorResourcePreviewPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a editor_resource_preview.
pub fn is_editor_resource_preview(node: &gdnative::prelude::Node) -> bool {
    node.cast::<EditorResourcePreview>().is_some()
}

/// A bundle for EditorResourcePreviews.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDEditorResourcePreviewBundle {
    pub editor_resource_preview: GDEditorResourcePreview,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDEditorResourcePreviewBundle {
    fn default() -> Self {
        Self {
            editor_resource_preview: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "EditorResourcePreview".to_string()
            }
        }
    }
}

/// Represents a EditorResourcePreview.
#[derive(Component)]
pub struct GDEditorResourcePreview {
    
}

impl Default for GDEditorResourcePreview {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDEditorResourcePreview {
    type Parent = GDNode;
    type GodotClass = EditorResourcePreview;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<EditorResourcePreview>().unwrap();
        world_commands.insert(entity, GDEditorResourcePreview {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDEditorResourcePreview {
    
}

fn sync_bevy_owned(query: Query<(&GDEditorResourcePreview, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<EditorResourcePreview>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDEditorResourcePreview, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<EditorResourcePreview>().unwrap();
        
    }
}