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

pub struct EditorInspectorPlugin;

impl Plugin for EditorInspectorPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a editor_inspector.
pub fn is_editor_inspector(node: &gdnative::prelude::Node) -> bool {
    node.cast::<EditorInspector>().is_some()
}

/// A bundle for EditorInspectors.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDEditorInspectorBundle {
    pub editor_inspector: GDEditorInspector,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
pub scroll_container: GDScrollContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDEditorInspectorBundle {
    fn default() -> Self {
        Self {
            editor_inspector: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
scroll_container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "EditorInspector".to_string()
            }
        }
    }
}

/// Represents a EditorInspector.
#[derive(Component)]
pub struct GDEditorInspector {
    
}

impl Default for GDEditorInspector {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDEditorInspector {
    type Parent = GDScrollContainer;
    type GodotClass = EditorInspector;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<EditorInspector>().unwrap();
        world_commands.insert(entity, GDEditorInspector {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDEditorInspector {
    
}

fn sync_bevy_owned(query: Query<(&GDEditorInspector, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<EditorInspector>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDEditorInspector, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<EditorInspector>().unwrap();
        
    }
}