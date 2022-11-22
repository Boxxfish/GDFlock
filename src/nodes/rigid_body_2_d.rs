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

pub struct RigidBody2DPlugin;

impl Plugin for RigidBody2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<RigidBody2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a rigid_body_2_d.
pub fn is_rigid_body_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<RigidBody2D>().is_some()
}

/// A bundle for RigidBody2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDRigidBody2DBundle {
    pub rigid_body_2_d: GDRigidBody2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
pub collision_object_2_d: GDCollisionObject2D,
pub physics_body_2_d: GDPhysicsBody2D,
    pub true_type: TrueNodeType,
}

impl Default for GDRigidBody2DBundle {
    fn default() -> Self {
        Self {
            rigid_body_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
collision_object_2_d: Default::default(),
physics_body_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "RigidBody2D".to_string()
            }
        }
    }
}

/// Represents a RigidBody2D.
#[derive(Component)]
pub struct GDRigidBody2D {
    pub angular_damp: f64,
pub angular_velocity: f64,
pub applied_force: Vector2,
pub applied_torque: f64,
pub bounce: f64,
pub can_sleep: bool,
pub contact_monitor: bool,
pub contacts_reported: i64,
pub custom_integrator: bool,
pub friction: f64,
pub gravity_scale: f64,
pub inertia: f64,
pub linear_damp: f64,
pub linear_velocity: Vector2,
pub mass: f64,
pub sleeping: bool,
pub weight: f64,
}

impl Default for GDRigidBody2D {
    fn default() -> Self {
        Self {
            angular_damp: Default::default(),
angular_velocity: Default::default(),
applied_force: Default::default(),
applied_torque: Default::default(),
bounce: Default::default(),
can_sleep: Default::default(),
contact_monitor: Default::default(),
contacts_reported: Default::default(),
custom_integrator: Default::default(),
friction: Default::default(),
gravity_scale: Default::default(),
inertia: Default::default(),
linear_damp: Default::default(),
linear_velocity: Default::default(),
mass: Default::default(),
sleeping: Default::default(),
weight: Default::default(),
        }
    }
}

impl NodeClass for GDRigidBody2D {
    type Parent = GDPhysicsBody2D;
    type GodotClass = RigidBody2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<RigidBody2D>().unwrap();
        world_commands.insert(entity, GDRigidBody2D {
            angular_damp: component_ref.angular_damp(),
angular_velocity: component_ref.angular_velocity(),
applied_force: component_ref.applied_force(),
applied_torque: component_ref.applied_torque(),
bounce: component_ref.bounce(),
can_sleep: component_ref.is_able_to_sleep(),
contact_monitor: component_ref.is_contact_monitor_enabled(),
contacts_reported: component_ref.max_contacts_reported(),
custom_integrator: component_ref.is_using_custom_integrator(),
friction: component_ref.friction(),
gravity_scale: component_ref.gravity_scale(),
inertia: component_ref.inertia(),
linear_damp: component_ref.linear_damp(),
linear_velocity: component_ref.linear_velocity(),
mass: component_ref.mass(),
sleeping: component_ref.is_sleeping(),
weight: component_ref.weight(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDRigidBody2D {
    
}

fn sync_bevy_owned(query: Query<(&GDRigidBody2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<RigidBody2D>().unwrap();
        component_ref.set_angular_damp(component.angular_damp);
component_ref.set_angular_velocity(component.angular_velocity);
component_ref.set_applied_force(component.applied_force);
component_ref.set_applied_torque(component.applied_torque);
component_ref.set_bounce(component.bounce);
component_ref.set_can_sleep(component.can_sleep);
component_ref.set_contact_monitor(component.contact_monitor);
component_ref.set_max_contacts_reported(component.contacts_reported);
component_ref.set_use_custom_integrator(component.custom_integrator);
component_ref.set_friction(component.friction);
component_ref.set_gravity_scale(component.gravity_scale);
component_ref.set_inertia(component.inertia);
component_ref.set_linear_damp(component.linear_damp);
component_ref.set_linear_velocity(component.linear_velocity);
component_ref.set_mass(component.mass);
component_ref.set_sleeping(component.sleeping);
component_ref.set_weight(component.weight);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDRigidBody2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<RigidBody2D>().unwrap();
        component.angular_damp = component_ref.angular_damp();
component.angular_velocity = component_ref.angular_velocity();
component.applied_force = component_ref.applied_force();
component.applied_torque = component_ref.applied_torque();
component.bounce = component_ref.bounce();
component.can_sleep = component_ref.is_able_to_sleep();
component.contact_monitor = component_ref.is_contact_monitor_enabled();
component.contacts_reported = component_ref.max_contacts_reported();
component.custom_integrator = component_ref.is_using_custom_integrator();
component.friction = component_ref.friction();
component.gravity_scale = component_ref.gravity_scale();
component.inertia = component_ref.inertia();
component.linear_damp = component_ref.linear_damp();
component.linear_velocity = component_ref.linear_velocity();
component.mass = component_ref.mass();
component.sleeping = component_ref.is_sleeping();
component.weight = component_ref.weight();
    }
}