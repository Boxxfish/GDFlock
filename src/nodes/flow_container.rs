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

pub struct FlowContainerPlugin;

impl Plugin for FlowContainerPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a flow_container.
pub fn is_flow_container(node: &gdnative::prelude::Node) -> bool {
    node.cast::<FlowContainer>().is_some()
}

/// A bundle for FlowContainers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDFlowContainerBundle {
    pub flow_container: GDFlowContainer,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDFlowContainerBundle {
    fn default() -> Self {
        Self {
            flow_container: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "FlowContainer".to_string()
            }
        }
    }
}

/// Represents a FlowContainer.
#[derive(Component)]
pub struct GDFlowContainer {
    
}

impl Default for GDFlowContainer {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDFlowContainer {
    type Parent = GDContainer;
    type GodotClass = FlowContainer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<FlowContainer>().unwrap();
        world_commands.insert(entity, GDFlowContainer {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDFlowContainer {
    
}

fn sync_bevy_owned(query: Query<(&GDFlowContainer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<FlowContainer>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDFlowContainer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<FlowContainer>().unwrap();
        
    }
}