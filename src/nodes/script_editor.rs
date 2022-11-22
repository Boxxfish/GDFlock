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

pub struct ScriptEditorPlugin;

impl Plugin for ScriptEditorPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a script_editor.
pub fn is_script_editor(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ScriptEditor>().is_some()
}

/// A bundle for ScriptEditors.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDScriptEditorBundle {
    pub script_editor: GDScriptEditor,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
pub panel_container: GDPanelContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDScriptEditorBundle {
    fn default() -> Self {
        Self {
            script_editor: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
panel_container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ScriptEditor".to_string()
            }
        }
    }
}

/// Represents a ScriptEditor.
#[derive(Component)]
pub struct GDScriptEditor {
    
}

impl Default for GDScriptEditor {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDScriptEditor {
    type Parent = GDPanelContainer;
    type GodotClass = ScriptEditor;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ScriptEditor>().unwrap();
        world_commands.insert(entity, GDScriptEditor {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDScriptEditor {
    
}

fn sync_bevy_owned(query: Query<(&GDScriptEditor, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ScriptEditor>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDScriptEditor, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ScriptEditor>().unwrap();
        
    }
}