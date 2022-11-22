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

pub struct Navigation2DPlugin;

impl Plugin for Navigation2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Navigation2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a navigation_2_d.
pub fn is_navigation_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Navigation2D>().is_some()
}

/// A bundle for Navigation2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDNavigation2DBundle {
    pub navigation_2_d: GDNavigation2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDNavigation2DBundle {
    fn default() -> Self {
        Self {
            navigation_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Navigation2D".to_string()
            }
        }
    }
}

/// Represents a Navigation2D.
#[derive(Component)]
pub struct GDNavigation2D {
    pub cell_size: f64,
pub edge_connection_margin: f64,
pub navigation_layers: i64,
}

impl Default for GDNavigation2D {
    fn default() -> Self {
        Self {
            cell_size: Default::default(),
edge_connection_margin: Default::default(),
navigation_layers: Default::default(),
        }
    }
}

impl NodeClass for GDNavigation2D {
    type Parent = GDNode2D;
    type GodotClass = Navigation2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Navigation2D>().unwrap();
        world_commands.insert(entity, GDNavigation2D {
            cell_size: component_ref.cell_size(),
edge_connection_margin: component_ref.edge_connection_margin(),
navigation_layers: component_ref.navigation_layers(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDNavigation2D {
    
}

fn sync_bevy_owned(query: Query<(&GDNavigation2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Navigation2D>().unwrap();
        component_ref.set_cell_size(component.cell_size);
component_ref.set_edge_connection_margin(component.edge_connection_margin);
component_ref.set_navigation_layers(component.navigation_layers);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDNavigation2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Navigation2D>().unwrap();
        component.cell_size = component_ref.cell_size();
component.edge_connection_margin = component_ref.edge_connection_margin();
component.navigation_layers = component_ref.navigation_layers();
    }
}