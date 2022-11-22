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

pub struct CollisionObject2DPlugin;

impl Plugin for CollisionObject2DPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a collision_object_2_d.
pub fn is_collision_object_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CollisionObject2D>().is_some()
}

/// A bundle for CollisionObject2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCollisionObject2DBundle {
    pub collision_object_2_d: GDCollisionObject2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDCollisionObject2DBundle {
    fn default() -> Self {
        Self {
            collision_object_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CollisionObject2D".to_string()
            }
        }
    }
}

/// Represents a CollisionObject2D.
#[derive(Component)]
pub struct GDCollisionObject2D {
    pub collision_layer: i64,
pub collision_mask: i64,
pub input_pickable: bool,
}

impl Default for GDCollisionObject2D {
    fn default() -> Self {
        Self {
            collision_layer: Default::default(),
collision_mask: Default::default(),
input_pickable: Default::default(),
        }
    }
}

impl NodeClass for GDCollisionObject2D {
    type Parent = GDNode2D;
    type GodotClass = CollisionObject2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CollisionObject2D>().unwrap();
        world_commands.insert(entity, GDCollisionObject2D {
            collision_layer: component_ref.collision_layer(),
collision_mask: component_ref.collision_mask(),
input_pickable: component_ref.is_pickable(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCollisionObject2D {
    
}

fn sync_bevy_owned(query: Query<(&GDCollisionObject2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CollisionObject2D>().unwrap();
        component_ref.set_collision_layer(component.collision_layer);
component_ref.set_collision_mask(component.collision_mask);
component_ref.set_pickable(component.input_pickable);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCollisionObject2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CollisionObject2D>().unwrap();
        component.collision_layer = component_ref.collision_layer();
component.collision_mask = component_ref.collision_mask();
component.input_pickable = component_ref.is_pickable();
    }
}