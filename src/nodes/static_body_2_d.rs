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

pub struct StaticBody2DPlugin;

impl Plugin for StaticBody2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<StaticBody2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a static_body_2_d.
pub fn is_static_body_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<StaticBody2D>().is_some()
}

/// A bundle for StaticBody2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDStaticBody2DBundle {
    pub static_body_2_d: GDStaticBody2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
pub collision_object_2_d: GDCollisionObject2D,
pub physics_body_2_d: GDPhysicsBody2D,
    pub true_type: TrueNodeType,
}

impl Default for GDStaticBody2DBundle {
    fn default() -> Self {
        Self {
            static_body_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
collision_object_2_d: Default::default(),
physics_body_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "StaticBody2D".to_string()
            }
        }
    }
}

/// Represents a StaticBody2D.
#[derive(Component)]
pub struct GDStaticBody2D {
    pub bounce: f64,
pub constant_angular_velocity: f64,
pub constant_linear_velocity: Vector2,
pub friction: f64,
}

impl Default for GDStaticBody2D {
    fn default() -> Self {
        Self {
            bounce: Default::default(),
constant_angular_velocity: Default::default(),
constant_linear_velocity: Default::default(),
friction: Default::default(),
        }
    }
}

impl NodeClass for GDStaticBody2D {
    type Parent = GDPhysicsBody2D;
    type GodotClass = StaticBody2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<StaticBody2D>().unwrap();
        world_commands.insert(entity, GDStaticBody2D {
            bounce: component_ref.bounce(),
constant_angular_velocity: component_ref.constant_angular_velocity(),
constant_linear_velocity: component_ref.constant_linear_velocity(),
friction: component_ref.friction(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDStaticBody2D {
    
}

fn sync_bevy_owned(query: Query<(&GDStaticBody2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<StaticBody2D>().unwrap();
        component_ref.set_bounce(component.bounce);
component_ref.set_constant_angular_velocity(component.constant_angular_velocity);
component_ref.set_constant_linear_velocity(component.constant_linear_velocity);
component_ref.set_friction(component.friction);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDStaticBody2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<StaticBody2D>().unwrap();
        component.bounce = component_ref.bounce();
component.constant_angular_velocity = component_ref.constant_angular_velocity();
component.constant_linear_velocity = component_ref.constant_linear_velocity();
component.friction = component_ref.friction();
    }
}