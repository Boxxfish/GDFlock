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

pub struct GridContainerPlugin;

impl Plugin for GridContainerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<GridContainer>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a grid_container.
pub fn is_grid_container(node: &gdnative::prelude::Node) -> bool {
    node.cast::<GridContainer>().is_some()
}

/// A bundle for GridContainers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDGridContainerBundle {
    pub grid_container: GDGridContainer,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDGridContainerBundle {
    fn default() -> Self {
        Self {
            grid_container: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "GridContainer".to_string()
            }
        }
    }
}

/// Represents a GridContainer.
#[derive(Component)]
pub struct GDGridContainer {
    pub columns: i64,
}

impl Default for GDGridContainer {
    fn default() -> Self {
        Self {
            columns: Default::default(),
        }
    }
}

impl NodeClass for GDGridContainer {
    type Parent = GDContainer;
    type GodotClass = GridContainer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<GridContainer>().unwrap();
        world_commands.insert(entity, GDGridContainer {
            columns: component_ref.columns(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDGridContainer {
    
}

fn sync_bevy_owned(query: Query<(&GDGridContainer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<GridContainer>().unwrap();
        component_ref.set_columns(component.columns);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDGridContainer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<GridContainer>().unwrap();
        component.columns = component_ref.columns();
    }
}