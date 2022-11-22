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

pub struct RayCast2DPlugin;

impl Plugin for RayCast2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<RayCast2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a ray_cast_2_d.
pub fn is_ray_cast_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<RayCast2D>().is_some()
}

/// A bundle for RayCast2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDRayCast2DBundle {
    pub ray_cast_2_d: GDRayCast2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDRayCast2DBundle {
    fn default() -> Self {
        Self {
            ray_cast_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "RayCast2D".to_string()
            }
        }
    }
}

/// Represents a RayCast2D.
#[derive(Component)]
pub struct GDRayCast2D {
    pub cast_to: Vector2,
pub collide_with_areas: bool,
pub collide_with_bodies: bool,
pub collision_mask: i64,
pub enabled: bool,
pub exclude_parent: bool,
}

impl Default for GDRayCast2D {
    fn default() -> Self {
        Self {
            cast_to: Default::default(),
collide_with_areas: Default::default(),
collide_with_bodies: Default::default(),
collision_mask: Default::default(),
enabled: Default::default(),
exclude_parent: Default::default(),
        }
    }
}

impl NodeClass for GDRayCast2D {
    type Parent = GDNode2D;
    type GodotClass = RayCast2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<RayCast2D>().unwrap();
        world_commands.insert(entity, GDRayCast2D {
            cast_to: component_ref.cast_to(),
collide_with_areas: component_ref.is_collide_with_areas_enabled(),
collide_with_bodies: component_ref.is_collide_with_bodies_enabled(),
collision_mask: component_ref.collision_mask(),
enabled: component_ref.is_enabled(),
exclude_parent: component_ref.exclude_parent_body(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDRayCast2D {
    
}

fn sync_bevy_owned(query: Query<(&GDRayCast2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<RayCast2D>().unwrap();
        component_ref.set_cast_to(component.cast_to);
component_ref.set_collide_with_areas(component.collide_with_areas);
component_ref.set_collide_with_bodies(component.collide_with_bodies);
component_ref.set_collision_mask(component.collision_mask);
component_ref.set_enabled(component.enabled);
component_ref.set_exclude_parent_body(component.exclude_parent);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDRayCast2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<RayCast2D>().unwrap();
        component.cast_to = component_ref.cast_to();
component.collide_with_areas = component_ref.is_collide_with_areas_enabled();
component.collide_with_bodies = component_ref.is_collide_with_bodies_enabled();
component.collision_mask = component_ref.collision_mask();
component.enabled = component_ref.is_enabled();
component.exclude_parent = component_ref.exclude_parent_body();
    }
}