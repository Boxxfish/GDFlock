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

pub struct NavigationObstacle2DPlugin;

impl Plugin for NavigationObstacle2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<NavigationObstacle2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a navigation_obstacle_2_d.
pub fn is_navigation_obstacle_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<NavigationObstacle2D>().is_some()
}

/// A bundle for NavigationObstacle2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDNavigationObstacle2DBundle {
    pub navigation_obstacle_2_d: GDNavigationObstacle2D,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDNavigationObstacle2DBundle {
    fn default() -> Self {
        Self {
            navigation_obstacle_2_d: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "NavigationObstacle2D".to_string()
            }
        }
    }
}

/// Represents a NavigationObstacle2D.
#[derive(Component)]
pub struct GDNavigationObstacle2D {
    pub estimate_radius: bool,
pub radius: f64,
}

impl Default for GDNavigationObstacle2D {
    fn default() -> Self {
        Self {
            estimate_radius: Default::default(),
radius: Default::default(),
        }
    }
}

impl NodeClass for GDNavigationObstacle2D {
    type Parent = GDNode;
    type GodotClass = NavigationObstacle2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<NavigationObstacle2D>().unwrap();
        world_commands.insert(entity, GDNavigationObstacle2D {
            estimate_radius: component_ref.is_radius_estimated(),
radius: component_ref.radius(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDNavigationObstacle2D {
    
}

fn sync_bevy_owned(query: Query<(&GDNavigationObstacle2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<NavigationObstacle2D>().unwrap();
        component_ref.set_estimate_radius(component.estimate_radius);
component_ref.set_radius(component.radius);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDNavigationObstacle2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<NavigationObstacle2D>().unwrap();
        component.estimate_radius = component_ref.is_radius_estimated();
component.radius = component_ref.radius();
    }
}