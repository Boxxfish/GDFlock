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

pub struct BoxContainerPlugin;

impl Plugin for BoxContainerPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a box_container.
pub fn is_box_container(node: &gdnative::prelude::Node) -> bool {
    node.cast::<BoxContainer>().is_some()
}

/// A bundle for BoxContainers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDBoxContainerBundle {
    pub box_container: GDBoxContainer,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDBoxContainerBundle {
    fn default() -> Self {
        Self {
            box_container: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "BoxContainer".to_string()
            }
        }
    }
}

/// Represents a BoxContainer.
#[derive(Component)]
pub struct GDBoxContainer {
    
}

impl Default for GDBoxContainer {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDBoxContainer {
    type Parent = GDContainer;
    type GodotClass = BoxContainer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<BoxContainer>().unwrap();
        world_commands.insert(entity, GDBoxContainer {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDBoxContainer {
    
}

fn sync_bevy_owned(query: Query<(&GDBoxContainer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<BoxContainer>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDBoxContainer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<BoxContainer>().unwrap();
        
    }
}