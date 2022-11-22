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

pub struct NavigationMeshInstancePlugin;

impl Plugin for NavigationMeshInstancePlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<NavigationMeshInstance>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a navigation_mesh_instance.
pub fn is_navigation_mesh_instance(node: &gdnative::prelude::Node) -> bool {
    node.cast::<NavigationMeshInstance>().is_some()
}

/// A bundle for NavigationMeshInstances.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDNavigationMeshInstanceBundle {
    pub navigation_mesh_instance: GDNavigationMeshInstance,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDNavigationMeshInstanceBundle {
    fn default() -> Self {
        Self {
            navigation_mesh_instance: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "NavigationMeshInstance".to_string()
            }
        }
    }
}

/// Represents a NavigationMeshInstance.
#[derive(Component)]
pub struct GDNavigationMeshInstance {
    pub enabled: bool,
pub enter_cost: f64,
pub navigation_layers: i64,
pub travel_cost: f64,
}

impl Default for GDNavigationMeshInstance {
    fn default() -> Self {
        Self {
            enabled: Default::default(),
enter_cost: Default::default(),
navigation_layers: Default::default(),
travel_cost: Default::default(),
        }
    }
}

impl NodeClass for GDNavigationMeshInstance {
    type Parent = GDSpatial;
    type GodotClass = NavigationMeshInstance;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<NavigationMeshInstance>().unwrap();
        world_commands.insert(entity, GDNavigationMeshInstance {
            enabled: component_ref.is_enabled(),
enter_cost: component_ref.enter_cost(),
navigation_layers: component_ref.navigation_layers(),
travel_cost: component_ref.travel_cost(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDNavigationMeshInstance {
    
}

fn sync_bevy_owned(query: Query<(&GDNavigationMeshInstance, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<NavigationMeshInstance>().unwrap();
        component_ref.set_enabled(component.enabled);
component_ref.set_enter_cost(component.enter_cost);
component_ref.set_navigation_layers(component.navigation_layers);
component_ref.set_travel_cost(component.travel_cost);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDNavigationMeshInstance, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<NavigationMeshInstance>().unwrap();
        component.enabled = component_ref.is_enabled();
component.enter_cost = component_ref.enter_cost();
component.navigation_layers = component_ref.navigation_layers();
component.travel_cost = component_ref.travel_cost();
    }
}