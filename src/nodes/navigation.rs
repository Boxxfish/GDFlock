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

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Navigation>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a navigation.
pub fn is_navigation(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Navigation>().is_some()
}

/// A bundle for Navigations.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDNavigationBundle {
    pub navigation: GDNavigation,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDNavigationBundle {
    fn default() -> Self {
        Self {
            navigation: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Navigation".to_string()
            }
        }
    }
}

/// Represents a Navigation.
#[derive(Component)]
pub struct GDNavigation {
    pub cell_height: f64,
pub cell_size: f64,
pub edge_connection_margin: f64,
pub navigation_layers: i64,
pub up_vector: Vector3,
}

impl Default for GDNavigation {
    fn default() -> Self {
        Self {
            cell_height: Default::default(),
cell_size: Default::default(),
edge_connection_margin: Default::default(),
navigation_layers: Default::default(),
up_vector: Default::default(),
        }
    }
}

impl NodeClass for GDNavigation {
    type Parent = GDSpatial;
    type GodotClass = Navigation;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Navigation>().unwrap();
        world_commands.insert(entity, GDNavigation {
            cell_height: component_ref.cell_height(),
cell_size: component_ref.cell_size(),
edge_connection_margin: component_ref.edge_connection_margin(),
navigation_layers: component_ref.navigation_layers(),
up_vector: component_ref.up_vector(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDNavigation {
    
}

fn sync_bevy_owned(query: Query<(&GDNavigation, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Navigation>().unwrap();
        component_ref.set_cell_height(component.cell_height);
component_ref.set_cell_size(component.cell_size);
component_ref.set_edge_connection_margin(component.edge_connection_margin);
component_ref.set_navigation_layers(component.navigation_layers);
component_ref.set_up_vector(component.up_vector);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDNavigation, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Navigation>().unwrap();
        component.cell_height = component_ref.cell_height();
component.cell_size = component_ref.cell_size();
component.edge_connection_margin = component_ref.edge_connection_margin();
component.navigation_layers = component_ref.navigation_layers();
component.up_vector = component_ref.up_vector();
    }
}