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

pub struct PanelContainerPlugin;

impl Plugin for PanelContainerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<PanelContainer>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a panel_container.
pub fn is_panel_container(node: &gdnative::prelude::Node) -> bool {
    node.cast::<PanelContainer>().is_some()
}

/// A bundle for PanelContainers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDPanelContainerBundle {
    pub panel_container: GDPanelContainer,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDPanelContainerBundle {
    fn default() -> Self {
        Self {
            panel_container: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "PanelContainer".to_string()
            }
        }
    }
}

/// Represents a PanelContainer.
#[derive(Component)]
pub struct GDPanelContainer {
    
}

impl Default for GDPanelContainer {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDPanelContainer {
    type Parent = GDContainer;
    type GodotClass = PanelContainer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<PanelContainer>().unwrap();
        world_commands.insert(entity, GDPanelContainer {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDPanelContainer {
    
}

fn sync_bevy_owned(query: Query<(&GDPanelContainer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PanelContainer>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDPanelContainer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PanelContainer>().unwrap();
        
    }
}