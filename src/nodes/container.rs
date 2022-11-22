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

pub struct ContainerPlugin;

impl Plugin for ContainerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Container>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a container.
pub fn is_container(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Container>().is_some()
}

/// A bundle for Containers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDContainerBundle {
    pub container: GDContainer,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
    pub true_type: TrueNodeType,
}

impl Default for GDContainerBundle {
    fn default() -> Self {
        Self {
            container: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Container".to_string()
            }
        }
    }
}

/// Represents a Container.
#[derive(Component)]
pub struct GDContainer {
    
}

impl Default for GDContainer {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDContainer {
    type Parent = GDControl;
    type GodotClass = Container;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Container>().unwrap();
        world_commands.insert(entity, GDContainer {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDContainer {
    
}

fn sync_bevy_owned(query: Query<(&GDContainer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Container>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDContainer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Container>().unwrap();
        
    }
}