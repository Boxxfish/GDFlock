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

pub struct RayCastPlugin;

impl Plugin for RayCastPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<RayCast>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a ray_cast.
pub fn is_ray_cast(node: &gdnative::prelude::Node) -> bool {
    node.cast::<RayCast>().is_some()
}

/// A bundle for RayCasts.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDRayCastBundle {
    pub ray_cast: GDRayCast,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDRayCastBundle {
    fn default() -> Self {
        Self {
            ray_cast: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "RayCast".to_string()
            }
        }
    }
}

/// Represents a RayCast.
#[derive(Component)]
pub struct GDRayCast {
    pub cast_to: Vector3,
pub collide_with_areas: bool,
pub collide_with_bodies: bool,
pub collision_mask: i64,
pub debug_shape_custom_color: Color,
pub debug_shape_thickness: i64,
pub enabled: bool,
pub exclude_parent: bool,
}

impl Default for GDRayCast {
    fn default() -> Self {
        Self {
            cast_to: Default::default(),
collide_with_areas: Default::default(),
collide_with_bodies: Default::default(),
collision_mask: Default::default(),
debug_shape_custom_color: Color::from_rgb(0.0, 0.0, 0.0),
debug_shape_thickness: Default::default(),
enabled: Default::default(),
exclude_parent: Default::default(),
        }
    }
}

impl NodeClass for GDRayCast {
    type Parent = GDSpatial;
    type GodotClass = RayCast;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<RayCast>().unwrap();
        world_commands.insert(entity, GDRayCast {
            cast_to: component_ref.cast_to(),
collide_with_areas: component_ref.is_collide_with_areas_enabled(),
collide_with_bodies: component_ref.is_collide_with_bodies_enabled(),
collision_mask: component_ref.collision_mask(),
debug_shape_custom_color: component_ref.debug_shape_custom_color(),
debug_shape_thickness: component_ref.debug_shape_thickness(),
enabled: component_ref.is_enabled(),
exclude_parent: component_ref.exclude_parent_body(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDRayCast {
    
}

fn sync_bevy_owned(query: Query<(&GDRayCast, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<RayCast>().unwrap();
        component_ref.set_cast_to(component.cast_to);
component_ref.set_collide_with_areas(component.collide_with_areas);
component_ref.set_collide_with_bodies(component.collide_with_bodies);
component_ref.set_collision_mask(component.collision_mask);
component_ref.set_debug_shape_custom_color(component.debug_shape_custom_color);
component_ref.set_debug_shape_thickness(component.debug_shape_thickness);
component_ref.set_enabled(component.enabled);
component_ref.set_exclude_parent_body(component.exclude_parent);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDRayCast, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<RayCast>().unwrap();
        component.cast_to = component_ref.cast_to();
component.collide_with_areas = component_ref.is_collide_with_areas_enabled();
component.collide_with_bodies = component_ref.is_collide_with_bodies_enabled();
component.collision_mask = component_ref.collision_mask();
component.debug_shape_custom_color = component_ref.debug_shape_custom_color();
component.debug_shape_thickness = component_ref.debug_shape_thickness();
component.enabled = component_ref.is_enabled();
component.exclude_parent = component_ref.exclude_parent_body();
    }
}