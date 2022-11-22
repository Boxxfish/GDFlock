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

pub struct NavigationPolygonInstancePlugin;

impl Plugin for NavigationPolygonInstancePlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<NavigationPolygonInstance>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a navigation_polygon_instance.
pub fn is_navigation_polygon_instance(node: &gdnative::prelude::Node) -> bool {
    node.cast::<NavigationPolygonInstance>().is_some()
}

/// A bundle for NavigationPolygonInstances.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDNavigationPolygonInstanceBundle {
    pub navigation_polygon_instance: GDNavigationPolygonInstance,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDNavigationPolygonInstanceBundle {
    fn default() -> Self {
        Self {
            navigation_polygon_instance: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "NavigationPolygonInstance".to_string()
            }
        }
    }
}

/// Represents a NavigationPolygonInstance.
#[derive(Component)]
pub struct GDNavigationPolygonInstance {
    pub enabled: bool,
pub enter_cost: f64,
pub navigation_layers: i64,
pub travel_cost: f64,
}

impl Default for GDNavigationPolygonInstance {
    fn default() -> Self {
        Self {
            enabled: Default::default(),
enter_cost: Default::default(),
navigation_layers: Default::default(),
travel_cost: Default::default(),
        }
    }
}

impl NodeClass for GDNavigationPolygonInstance {
    type Parent = GDNode2D;
    type GodotClass = NavigationPolygonInstance;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<NavigationPolygonInstance>().unwrap();
        world_commands.insert(entity, GDNavigationPolygonInstance {
            enabled: component_ref.is_enabled(),
enter_cost: component_ref.enter_cost(),
navigation_layers: component_ref.navigation_layers(),
travel_cost: component_ref.travel_cost(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDNavigationPolygonInstance {
    
}

fn sync_bevy_owned(query: Query<(&GDNavigationPolygonInstance, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<NavigationPolygonInstance>().unwrap();
        component_ref.set_enabled(component.enabled);
component_ref.set_enter_cost(component.enter_cost);
component_ref.set_navigation_layers(component.navigation_layers);
component_ref.set_travel_cost(component.travel_cost);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDNavigationPolygonInstance, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<NavigationPolygonInstance>().unwrap();
        component.enabled = component_ref.is_enabled();
component.enter_cost = component_ref.enter_cost();
component.navigation_layers = component_ref.navigation_layers();
component.travel_cost = component_ref.travel_cost();
    }
}