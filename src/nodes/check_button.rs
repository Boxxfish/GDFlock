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

pub struct CheckButtonPlugin;

impl Plugin for CheckButtonPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<CheckButton>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a check_button.
pub fn is_check_button(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CheckButton>().is_some()
}

/// A bundle for CheckButtons.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCheckButtonBundle {
    pub check_button: GDCheckButton,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub base_button: GDBaseButton,
pub button: GDButton,
    pub true_type: TrueNodeType,
}

impl Default for GDCheckButtonBundle {
    fn default() -> Self {
        Self {
            check_button: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
base_button: Default::default(),
button: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CheckButton".to_string()
            }
        }
    }
}

/// Represents a CheckButton.
#[derive(Component)]
pub struct GDCheckButton {
    
}

impl Default for GDCheckButton {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDCheckButton {
    type Parent = GDButton;
    type GodotClass = CheckButton;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CheckButton>().unwrap();
        world_commands.insert(entity, GDCheckButton {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCheckButton {
    
}

fn sync_bevy_owned(query: Query<(&GDCheckButton, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CheckButton>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCheckButton, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CheckButton>().unwrap();
        
    }
}