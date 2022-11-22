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

pub struct ToolButtonPlugin;

impl Plugin for ToolButtonPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ToolButton>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a tool_button.
pub fn is_tool_button(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ToolButton>().is_some()
}

/// A bundle for ToolButtons.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDToolButtonBundle {
    pub tool_button: GDToolButton,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub base_button: GDBaseButton,
pub button: GDButton,
    pub true_type: TrueNodeType,
}

impl Default for GDToolButtonBundle {
    fn default() -> Self {
        Self {
            tool_button: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
base_button: Default::default(),
button: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ToolButton".to_string()
            }
        }
    }
}

/// Represents a ToolButton.
#[derive(Component)]
pub struct GDToolButton {
    
}

impl Default for GDToolButton {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDToolButton {
    type Parent = GDButton;
    type GodotClass = ToolButton;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ToolButton>().unwrap();
        world_commands.insert(entity, GDToolButton {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDToolButton {
    
}

fn sync_bevy_owned(query: Query<(&GDToolButton, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ToolButton>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDToolButton, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ToolButton>().unwrap();
        
    }
}