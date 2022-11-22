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

pub struct HFlowContainerPlugin;

impl Plugin for HFlowContainerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<HFlowContainer>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a h_flow_container.
pub fn is_h_flow_container(node: &gdnative::prelude::Node) -> bool {
    node.cast::<HFlowContainer>().is_some()
}

/// A bundle for HFlowContainers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDHFlowContainerBundle {
    pub h_flow_container: GDHFlowContainer,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
pub flow_container: GDFlowContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDHFlowContainerBundle {
    fn default() -> Self {
        Self {
            h_flow_container: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
flow_container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "HFlowContainer".to_string()
            }
        }
    }
}

/// Represents a HFlowContainer.
#[derive(Component)]
pub struct GDHFlowContainer {
    
}

impl Default for GDHFlowContainer {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDHFlowContainer {
    type Parent = GDFlowContainer;
    type GodotClass = HFlowContainer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<HFlowContainer>().unwrap();
        world_commands.insert(entity, GDHFlowContainer {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDHFlowContainer {
    
}

fn sync_bevy_owned(query: Query<(&GDHFlowContainer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<HFlowContainer>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDHFlowContainer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<HFlowContainer>().unwrap();
        
    }
}