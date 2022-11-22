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

pub struct VFlowContainerPlugin;

impl Plugin for VFlowContainerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<VFlowContainer>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a v_flow_container.
pub fn is_v_flow_container(node: &gdnative::prelude::Node) -> bool {
    node.cast::<VFlowContainer>().is_some()
}

/// A bundle for VFlowContainers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDVFlowContainerBundle {
    pub v_flow_container: GDVFlowContainer,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
pub flow_container: GDFlowContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDVFlowContainerBundle {
    fn default() -> Self {
        Self {
            v_flow_container: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
flow_container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "VFlowContainer".to_string()
            }
        }
    }
}

/// Represents a VFlowContainer.
#[derive(Component)]
pub struct GDVFlowContainer {
    
}

impl Default for GDVFlowContainer {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDVFlowContainer {
    type Parent = GDFlowContainer;
    type GodotClass = VFlowContainer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<VFlowContainer>().unwrap();
        world_commands.insert(entity, GDVFlowContainer {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDVFlowContainer {
    
}

fn sync_bevy_owned(query: Query<(&GDVFlowContainer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VFlowContainer>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDVFlowContainer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VFlowContainer>().unwrap();
        
    }
}