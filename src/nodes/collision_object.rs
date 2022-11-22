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

pub struct CollisionObjectPlugin;

impl Plugin for CollisionObjectPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a collision_object.
pub fn is_collision_object(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CollisionObject>().is_some()
}

/// A bundle for CollisionObjects.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCollisionObjectBundle {
    pub collision_object: GDCollisionObject,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDCollisionObjectBundle {
    fn default() -> Self {
        Self {
            collision_object: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CollisionObject".to_string()
            }
        }
    }
}

/// Represents a CollisionObject.
#[derive(Component)]
pub struct GDCollisionObject {
    pub collision_layer: i64,
pub collision_mask: i64,
pub input_capture_on_drag: bool,
pub input_ray_pickable: bool,
}

impl Default for GDCollisionObject {
    fn default() -> Self {
        Self {
            collision_layer: Default::default(),
collision_mask: Default::default(),
input_capture_on_drag: Default::default(),
input_ray_pickable: Default::default(),
        }
    }
}

impl NodeClass for GDCollisionObject {
    type Parent = GDSpatial;
    type GodotClass = CollisionObject;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CollisionObject>().unwrap();
        world_commands.insert(entity, GDCollisionObject {
            collision_layer: component_ref.collision_layer(),
collision_mask: component_ref.collision_mask(),
input_capture_on_drag: component_ref.capture_input_on_drag(),
input_ray_pickable: component_ref.is_ray_pickable(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCollisionObject {
    
}

fn sync_bevy_owned(query: Query<(&GDCollisionObject, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CollisionObject>().unwrap();
        component_ref.set_collision_layer(component.collision_layer);
component_ref.set_collision_mask(component.collision_mask);
component_ref.set_capture_input_on_drag(component.input_capture_on_drag);
component_ref.set_ray_pickable(component.input_ray_pickable);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCollisionObject, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CollisionObject>().unwrap();
        component.collision_layer = component_ref.collision_layer();
component.collision_mask = component_ref.collision_mask();
component.input_capture_on_drag = component_ref.capture_input_on_drag();
component.input_ray_pickable = component_ref.is_ray_pickable();
    }
}