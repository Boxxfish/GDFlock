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

pub struct CollisionShape2DPlugin;

impl Plugin for CollisionShape2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<CollisionShape2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a collision_shape_2_d.
pub fn is_collision_shape_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CollisionShape2D>().is_some()
}

/// A bundle for CollisionShape2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCollisionShape2DBundle {
    pub collision_shape_2_d: GDCollisionShape2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDCollisionShape2DBundle {
    fn default() -> Self {
        Self {
            collision_shape_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CollisionShape2D".to_string()
            }
        }
    }
}

/// Represents a CollisionShape2D.
#[derive(Component)]
pub struct GDCollisionShape2D {
    pub disabled: bool,
pub one_way_collision: bool,
pub one_way_collision_margin: f64,
}

impl Default for GDCollisionShape2D {
    fn default() -> Self {
        Self {
            disabled: Default::default(),
one_way_collision: Default::default(),
one_way_collision_margin: Default::default(),
        }
    }
}

impl NodeClass for GDCollisionShape2D {
    type Parent = GDNode2D;
    type GodotClass = CollisionShape2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CollisionShape2D>().unwrap();
        world_commands.insert(entity, GDCollisionShape2D {
            disabled: component_ref.is_disabled(),
one_way_collision: component_ref.is_one_way_collision_enabled(),
one_way_collision_margin: component_ref.one_way_collision_margin(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCollisionShape2D {
    
}

fn sync_bevy_owned(query: Query<(&GDCollisionShape2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CollisionShape2D>().unwrap();
        component_ref.set_disabled(component.disabled);
component_ref.set_one_way_collision(component.one_way_collision);
component_ref.set_one_way_collision_margin(component.one_way_collision_margin);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCollisionShape2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CollisionShape2D>().unwrap();
        component.disabled = component_ref.is_disabled();
component.one_way_collision = component_ref.is_one_way_collision_enabled();
component.one_way_collision_margin = component_ref.one_way_collision_margin();
    }
}