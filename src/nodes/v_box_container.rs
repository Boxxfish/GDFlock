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

pub struct VBoxContainerPlugin;

impl Plugin for VBoxContainerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<VBoxContainer>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a v_box_container.
pub fn is_v_box_container(node: &gdnative::prelude::Node) -> bool {
    node.cast::<VBoxContainer>().is_some()
}

/// A bundle for VBoxContainers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDVBoxContainerBundle {
    pub v_box_container: GDVBoxContainer,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
pub box_container: GDBoxContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDVBoxContainerBundle {
    fn default() -> Self {
        Self {
            v_box_container: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
box_container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "VBoxContainer".to_string()
            }
        }
    }
}

/// Represents a VBoxContainer.
#[derive(Component)]
pub struct GDVBoxContainer {
    
}

impl Default for GDVBoxContainer {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDVBoxContainer {
    type Parent = GDBoxContainer;
    type GodotClass = VBoxContainer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<VBoxContainer>().unwrap();
        world_commands.insert(entity, GDVBoxContainer {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDVBoxContainer {
    
}

fn sync_bevy_owned(query: Query<(&GDVBoxContainer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VBoxContainer>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDVBoxContainer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VBoxContainer>().unwrap();
        
    }
}