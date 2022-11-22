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

pub struct OptionButtonPlugin;

impl Plugin for OptionButtonPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<OptionButton>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a option_button.
pub fn is_option_button(node: &gdnative::prelude::Node) -> bool {
    node.cast::<OptionButton>().is_some()
}

/// A bundle for OptionButtons.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDOptionButtonBundle {
    pub option_button: GDOptionButton,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub base_button: GDBaseButton,
pub button: GDButton,
    pub true_type: TrueNodeType,
}

impl Default for GDOptionButtonBundle {
    fn default() -> Self {
        Self {
            option_button: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
base_button: Default::default(),
button: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "OptionButton".to_string()
            }
        }
    }
}

/// Represents a OptionButton.
#[derive(Component)]
pub struct GDOptionButton {
    
}

impl Default for GDOptionButton {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDOptionButton {
    type Parent = GDButton;
    type GodotClass = OptionButton;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<OptionButton>().unwrap();
        world_commands.insert(entity, GDOptionButton {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDOptionButton {
    
}

fn sync_bevy_owned(query: Query<(&GDOptionButton, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<OptionButton>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDOptionButton, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<OptionButton>().unwrap();
        
    }
}