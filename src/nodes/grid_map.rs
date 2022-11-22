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

pub struct GridMapPlugin;

impl Plugin for GridMapPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<GridMap>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a grid_map.
pub fn is_grid_map(node: &gdnative::prelude::Node) -> bool {
    node.cast::<GridMap>().is_some()
}

/// A bundle for GridMaps.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDGridMapBundle {
    pub grid_map: GDGridMap,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDGridMapBundle {
    fn default() -> Self {
        Self {
            grid_map: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "GridMap".to_string()
            }
        }
    }
}

/// Represents a GridMap.
#[derive(Component)]
pub struct GDGridMap {
    pub bake_navigation: bool,
pub cell_center_x: bool,
pub cell_center_y: bool,
pub cell_center_z: bool,
pub cell_octant_size: i64,
pub cell_scale: f64,
pub cell_size: Vector3,
pub collision_layer: i64,
pub collision_mask: i64,
pub navigation_layers: i64,
pub use_in_baked_light: bool,
}

impl Default for GDGridMap {
    fn default() -> Self {
        Self {
            bake_navigation: Default::default(),
cell_center_x: Default::default(),
cell_center_y: Default::default(),
cell_center_z: Default::default(),
cell_octant_size: Default::default(),
cell_scale: Default::default(),
cell_size: Default::default(),
collision_layer: Default::default(),
collision_mask: Default::default(),
navigation_layers: Default::default(),
use_in_baked_light: Default::default(),
        }
    }
}

impl NodeClass for GDGridMap {
    type Parent = GDSpatial;
    type GodotClass = GridMap;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<GridMap>().unwrap();
        world_commands.insert(entity, GDGridMap {
            bake_navigation: component_ref.is_baking_navigation(),
cell_center_x: component_ref.center_x(),
cell_center_y: component_ref.center_y(),
cell_center_z: component_ref.center_z(),
cell_octant_size: component_ref.octant_size(),
cell_scale: component_ref.cell_scale(),
cell_size: component_ref.cell_size(),
collision_layer: component_ref.collision_layer(),
collision_mask: component_ref.collision_mask(),
navigation_layers: component_ref.navigation_layers(),
use_in_baked_light: component_ref.use_in_baked_light(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDGridMap {
    
}

fn sync_bevy_owned(query: Query<(&GDGridMap, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<GridMap>().unwrap();
        component_ref.set_bake_navigation(component.bake_navigation);
component_ref.set_center_x(component.cell_center_x);
component_ref.set_center_y(component.cell_center_y);
component_ref.set_center_z(component.cell_center_z);
component_ref.set_octant_size(component.cell_octant_size);
component_ref.set_cell_scale(component.cell_scale);
component_ref.set_cell_size(component.cell_size);
component_ref.set_collision_layer(component.collision_layer);
component_ref.set_collision_mask(component.collision_mask);
component_ref.set_navigation_layers(component.navigation_layers);
component_ref.set_use_in_baked_light(component.use_in_baked_light);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDGridMap, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<GridMap>().unwrap();
        component.bake_navigation = component_ref.is_baking_navigation();
component.cell_center_x = component_ref.center_x();
component.cell_center_y = component_ref.center_y();
component.cell_center_z = component_ref.center_z();
component.cell_octant_size = component_ref.octant_size();
component.cell_scale = component_ref.cell_scale();
component.cell_size = component_ref.cell_size();
component.collision_layer = component_ref.collision_layer();
component.collision_mask = component_ref.collision_mask();
component.navigation_layers = component_ref.navigation_layers();
component.use_in_baked_light = component_ref.use_in_baked_light();
    }
}