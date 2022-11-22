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

pub struct NavigationAgent2DPlugin;

impl Plugin for NavigationAgent2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<NavigationAgent2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a navigation_agent_2_d.
pub fn is_navigation_agent_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<NavigationAgent2D>().is_some()
}

/// A bundle for NavigationAgent2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDNavigationAgent2DBundle {
    pub navigation_agent_2_d: GDNavigationAgent2D,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDNavigationAgent2DBundle {
    fn default() -> Self {
        Self {
            navigation_agent_2_d: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "NavigationAgent2D".to_string()
            }
        }
    }
}

/// Represents a NavigationAgent2D.
#[derive(Component)]
pub struct GDNavigationAgent2D {
    pub avoidance_enabled: bool,
pub max_neighbors: i64,
pub max_speed: f64,
pub navigation_layers: i64,
pub neighbor_dist: f64,
pub path_desired_distance: f64,
pub path_max_distance: f64,
pub radius: f64,
pub target_desired_distance: f64,
pub time_horizon: f64,
}

impl Default for GDNavigationAgent2D {
    fn default() -> Self {
        Self {
            avoidance_enabled: Default::default(),
max_neighbors: Default::default(),
max_speed: Default::default(),
navigation_layers: Default::default(),
neighbor_dist: Default::default(),
path_desired_distance: Default::default(),
path_max_distance: Default::default(),
radius: Default::default(),
target_desired_distance: Default::default(),
time_horizon: Default::default(),
        }
    }
}

impl NodeClass for GDNavigationAgent2D {
    type Parent = GDNode;
    type GodotClass = NavigationAgent2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<NavigationAgent2D>().unwrap();
        world_commands.insert(entity, GDNavigationAgent2D {
            avoidance_enabled: component_ref.avoidance_enabled(),
max_neighbors: component_ref.max_neighbors(),
max_speed: component_ref.max_speed(),
navigation_layers: component_ref.navigation_layers(),
neighbor_dist: component_ref.neighbor_dist(),
path_desired_distance: component_ref.path_desired_distance(),
path_max_distance: component_ref.path_max_distance(),
radius: component_ref.radius(),
target_desired_distance: component_ref.target_desired_distance(),
time_horizon: component_ref.time_horizon(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDNavigationAgent2D {
    
}

fn sync_bevy_owned(query: Query<(&GDNavigationAgent2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<NavigationAgent2D>().unwrap();
        component_ref.set_avoidance_enabled(component.avoidance_enabled);
component_ref.set_max_neighbors(component.max_neighbors);
component_ref.set_max_speed(component.max_speed);
component_ref.set_navigation_layers(component.navigation_layers);
component_ref.set_neighbor_dist(component.neighbor_dist);
component_ref.set_path_desired_distance(component.path_desired_distance);
component_ref.set_path_max_distance(component.path_max_distance);
component_ref.set_radius(component.radius);
component_ref.set_target_desired_distance(component.target_desired_distance);
component_ref.set_time_horizon(component.time_horizon);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDNavigationAgent2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<NavigationAgent2D>().unwrap();
        component.avoidance_enabled = component_ref.avoidance_enabled();
component.max_neighbors = component_ref.max_neighbors();
component.max_speed = component_ref.max_speed();
component.navigation_layers = component_ref.navigation_layers();
component.neighbor_dist = component_ref.neighbor_dist();
component.path_desired_distance = component_ref.path_desired_distance();
component.path_max_distance = component_ref.path_max_distance();
component.radius = component_ref.radius();
component.target_desired_distance = component_ref.target_desired_distance();
component.time_horizon = component_ref.time_horizon();
    }
}