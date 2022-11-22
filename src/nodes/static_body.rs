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

pub struct StaticBodyPlugin;

impl Plugin for StaticBodyPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<StaticBody>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a static_body.
pub fn is_static_body(node: &gdnative::prelude::Node) -> bool {
    node.cast::<StaticBody>().is_some()
}

/// A bundle for StaticBodys.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDStaticBodyBundle {
    pub static_body: GDStaticBody,
    pub node: GDNode,
pub spatial: GDSpatial,
pub collision_object: GDCollisionObject,
pub physics_body: GDPhysicsBody,
    pub true_type: TrueNodeType,
}

impl Default for GDStaticBodyBundle {
    fn default() -> Self {
        Self {
            static_body: Default::default(),
            node: Default::default(),
spatial: Default::default(),
collision_object: Default::default(),
physics_body: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "StaticBody".to_string()
            }
        }
    }
}

/// Represents a StaticBody.
#[derive(Component)]
pub struct GDStaticBody {
    pub bounce: f64,
pub constant_angular_velocity: Vector3,
pub constant_linear_velocity: Vector3,
pub friction: f64,
}

impl Default for GDStaticBody {
    fn default() -> Self {
        Self {
            bounce: Default::default(),
constant_angular_velocity: Default::default(),
constant_linear_velocity: Default::default(),
friction: Default::default(),
        }
    }
}

impl NodeClass for GDStaticBody {
    type Parent = GDPhysicsBody;
    type GodotClass = StaticBody;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<StaticBody>().unwrap();
        world_commands.insert(entity, GDStaticBody {
            bounce: component_ref.bounce(),
constant_angular_velocity: component_ref.constant_angular_velocity(),
constant_linear_velocity: component_ref.constant_linear_velocity(),
friction: component_ref.friction(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDStaticBody {
    
}

fn sync_bevy_owned(query: Query<(&GDStaticBody, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<StaticBody>().unwrap();
        component_ref.set_bounce(component.bounce);
component_ref.set_constant_angular_velocity(component.constant_angular_velocity);
component_ref.set_constant_linear_velocity(component.constant_linear_velocity);
component_ref.set_friction(component.friction);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDStaticBody, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<StaticBody>().unwrap();
        component.bounce = component_ref.bounce();
component.constant_angular_velocity = component_ref.constant_angular_velocity();
component.constant_linear_velocity = component_ref.constant_linear_velocity();
component.friction = component_ref.friction();
    }
}