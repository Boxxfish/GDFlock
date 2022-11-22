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

pub struct NavigationObstaclePlugin;

impl Plugin for NavigationObstaclePlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<NavigationObstacle>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a navigation_obstacle.
pub fn is_navigation_obstacle(node: &gdnative::prelude::Node) -> bool {
    node.cast::<NavigationObstacle>().is_some()
}

/// A bundle for NavigationObstacles.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDNavigationObstacleBundle {
    pub navigation_obstacle: GDNavigationObstacle,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDNavigationObstacleBundle {
    fn default() -> Self {
        Self {
            navigation_obstacle: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "NavigationObstacle".to_string()
            }
        }
    }
}

/// Represents a NavigationObstacle.
#[derive(Component)]
pub struct GDNavigationObstacle {
    pub estimate_radius: bool,
pub radius: f64,
}

impl Default for GDNavigationObstacle {
    fn default() -> Self {
        Self {
            estimate_radius: Default::default(),
radius: Default::default(),
        }
    }
}

impl NodeClass for GDNavigationObstacle {
    type Parent = GDNode;
    type GodotClass = NavigationObstacle;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<NavigationObstacle>().unwrap();
        world_commands.insert(entity, GDNavigationObstacle {
            estimate_radius: component_ref.is_radius_estimated(),
radius: component_ref.radius(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDNavigationObstacle {
    
}

fn sync_bevy_owned(query: Query<(&GDNavigationObstacle, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<NavigationObstacle>().unwrap();
        component_ref.set_estimate_radius(component.estimate_radius);
component_ref.set_radius(component.radius);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDNavigationObstacle, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<NavigationObstacle>().unwrap();
        component.estimate_radius = component_ref.is_radius_estimated();
component.radius = component_ref.radius();
    }
}