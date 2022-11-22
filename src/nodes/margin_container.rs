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

pub struct MarginContainerPlugin;

impl Plugin for MarginContainerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<MarginContainer>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a margin_container.
pub fn is_margin_container(node: &gdnative::prelude::Node) -> bool {
    node.cast::<MarginContainer>().is_some()
}

/// A bundle for MarginContainers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDMarginContainerBundle {
    pub margin_container: GDMarginContainer,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDMarginContainerBundle {
    fn default() -> Self {
        Self {
            margin_container: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "MarginContainer".to_string()
            }
        }
    }
}

/// Represents a MarginContainer.
#[derive(Component)]
pub struct GDMarginContainer {
    
}

impl Default for GDMarginContainer {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDMarginContainer {
    type Parent = GDContainer;
    type GodotClass = MarginContainer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<MarginContainer>().unwrap();
        world_commands.insert(entity, GDMarginContainer {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDMarginContainer {
    
}

fn sync_bevy_owned(query: Query<(&GDMarginContainer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<MarginContainer>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDMarginContainer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<MarginContainer>().unwrap();
        
    }
}