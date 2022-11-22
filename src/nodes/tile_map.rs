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

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<TileMap>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a tile_map.
pub fn is_tile_map(node: &gdnative::prelude::Node) -> bool {
    node.cast::<TileMap>().is_some()
}

/// A bundle for TileMaps.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDTileMapBundle {
    pub tile_map: GDTileMap,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDTileMapBundle {
    fn default() -> Self {
        Self {
            tile_map: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "TileMap".to_string()
            }
        }
    }
}

/// Represents a TileMap.
#[derive(Component)]
pub struct GDTileMap {
    pub bake_navigation: bool,
pub cell_clip_uv: bool,
pub cell_custom_transform: Transform2D,
pub cell_quadrant_size: i64,
pub cell_size: Vector2,
pub cell_y_sort: bool,
pub centered_textures: bool,
pub collision_bounce: f64,
pub collision_friction: f64,
pub collision_layer: i64,
pub collision_mask: i64,
pub collision_use_kinematic: bool,
pub collision_use_parent: bool,
pub compatibility_mode: bool,
pub navigation_layers: i64,
pub occluder_light_mask: i64,
pub show_collision: bool,
}

impl Default for GDTileMap {
    fn default() -> Self {
        Self {
            bake_navigation: Default::default(),
cell_clip_uv: Default::default(),
cell_custom_transform: Transform2D::IDENTITY,
cell_quadrant_size: Default::default(),
cell_size: Default::default(),
cell_y_sort: Default::default(),
centered_textures: Default::default(),
collision_bounce: Default::default(),
collision_friction: Default::default(),
collision_layer: Default::default(),
collision_mask: Default::default(),
collision_use_kinematic: Default::default(),
collision_use_parent: Default::default(),
compatibility_mode: Default::default(),
navigation_layers: Default::default(),
occluder_light_mask: Default::default(),
show_collision: Default::default(),
        }
    }
}

impl NodeClass for GDTileMap {
    type Parent = GDNode2D;
    type GodotClass = TileMap;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<TileMap>().unwrap();
        world_commands.insert(entity, GDTileMap {
            bake_navigation: component_ref.is_baking_navigation(),
cell_clip_uv: component_ref.clip_uv(),
cell_custom_transform: component_ref.custom_transform(),
cell_quadrant_size: component_ref.quadrant_size(),
cell_size: component_ref.cell_size(),
cell_y_sort: component_ref.is_y_sort_mode_enabled(),
centered_textures: component_ref.is_centered_textures_enabled(),
collision_bounce: component_ref.collision_bounce(),
collision_friction: component_ref.collision_friction(),
collision_layer: component_ref.collision_layer(),
collision_mask: component_ref.collision_mask(),
collision_use_kinematic: component_ref.collision_use_kinematic(),
collision_use_parent: component_ref.collision_use_parent(),
compatibility_mode: component_ref.is_compatibility_mode_enabled(),
navigation_layers: component_ref.navigation_layers(),
occluder_light_mask: component_ref.occluder_light_mask(),
show_collision: component_ref.is_show_collision_enabled(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDTileMap {
    
}

fn sync_bevy_owned(query: Query<(&GDTileMap, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<TileMap>().unwrap();
        component_ref.set_bake_navigation(component.bake_navigation);
component_ref.set_clip_uv(component.cell_clip_uv);
component_ref.set_custom_transform(component.cell_custom_transform);
component_ref.set_quadrant_size(component.cell_quadrant_size);
component_ref.set_cell_size(component.cell_size);
component_ref.set_y_sort_mode(component.cell_y_sort);
component_ref.set_centered_textures(component.centered_textures);
component_ref.set_collision_bounce(component.collision_bounce);
component_ref.set_collision_friction(component.collision_friction);
component_ref.set_collision_layer(component.collision_layer);
component_ref.set_collision_mask(component.collision_mask);
component_ref.set_collision_use_kinematic(component.collision_use_kinematic);
component_ref.set_collision_use_parent(component.collision_use_parent);
component_ref.set_compatibility_mode(component.compatibility_mode);
component_ref.set_navigation_layers(component.navigation_layers);
component_ref.set_occluder_light_mask(component.occluder_light_mask);
component_ref.set_show_collision(component.show_collision);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDTileMap, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<TileMap>().unwrap();
        component.bake_navigation = component_ref.is_baking_navigation();
component.cell_clip_uv = component_ref.clip_uv();
component.cell_custom_transform = component_ref.custom_transform();
component.cell_quadrant_size = component_ref.quadrant_size();
component.cell_size = component_ref.cell_size();
component.cell_y_sort = component_ref.is_y_sort_mode_enabled();
component.centered_textures = component_ref.is_centered_textures_enabled();
component.collision_bounce = component_ref.collision_bounce();
component.collision_friction = component_ref.collision_friction();
component.collision_layer = component_ref.collision_layer();
component.collision_mask = component_ref.collision_mask();
component.collision_use_kinematic = component_ref.collision_use_kinematic();
component.collision_use_parent = component_ref.collision_use_parent();
component.compatibility_mode = component_ref.is_compatibility_mode_enabled();
component.navigation_layers = component_ref.navigation_layers();
component.occluder_light_mask = component_ref.occluder_light_mask();
component.show_collision = component_ref.is_show_collision_enabled();
    }
}