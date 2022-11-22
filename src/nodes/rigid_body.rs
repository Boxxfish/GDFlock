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

pub struct RigidBodyPlugin;

impl Plugin for RigidBodyPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<RigidBody>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a rigid_body.
pub fn is_rigid_body(node: &gdnative::prelude::Node) -> bool {
    node.cast::<RigidBody>().is_some()
}

/// A bundle for RigidBodys.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDRigidBodyBundle {
    pub rigid_body: GDRigidBody,
    pub node: GDNode,
pub spatial: GDSpatial,
pub collision_object: GDCollisionObject,
pub physics_body: GDPhysicsBody,
    pub true_type: TrueNodeType,
}

impl Default for GDRigidBodyBundle {
    fn default() -> Self {
        Self {
            rigid_body: Default::default(),
            node: Default::default(),
spatial: Default::default(),
collision_object: Default::default(),
physics_body: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "RigidBody".to_string()
            }
        }
    }
}

/// Represents a RigidBody.
#[derive(Component)]
pub struct GDRigidBody {
    pub angular_damp: f64,
pub angular_velocity: Vector3,
pub bounce: f64,
pub can_sleep: bool,
pub contact_monitor: bool,
pub contacts_reported: i64,
pub continuous_cd: bool,
pub custom_integrator: bool,
pub friction: f64,
pub gravity_scale: f64,
pub linear_damp: f64,
pub linear_velocity: Vector3,
pub mass: f64,
pub sleeping: bool,
pub weight: f64,
}

impl Default for GDRigidBody {
    fn default() -> Self {
        Self {
            angular_damp: Default::default(),
angular_velocity: Default::default(),
bounce: Default::default(),
can_sleep: Default::default(),
contact_monitor: Default::default(),
contacts_reported: Default::default(),
continuous_cd: Default::default(),
custom_integrator: Default::default(),
friction: Default::default(),
gravity_scale: Default::default(),
linear_damp: Default::default(),
linear_velocity: Default::default(),
mass: Default::default(),
sleeping: Default::default(),
weight: Default::default(),
        }
    }
}

impl NodeClass for GDRigidBody {
    type Parent = GDPhysicsBody;
    type GodotClass = RigidBody;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<RigidBody>().unwrap();
        world_commands.insert(entity, GDRigidBody {
            angular_damp: component_ref.angular_damp(),
angular_velocity: component_ref.angular_velocity(),
bounce: component_ref.bounce(),
can_sleep: component_ref.is_able_to_sleep(),
contact_monitor: component_ref.is_contact_monitor_enabled(),
contacts_reported: component_ref.max_contacts_reported(),
continuous_cd: component_ref.is_using_continuous_collision_detection(),
custom_integrator: component_ref.is_using_custom_integrator(),
friction: component_ref.friction(),
gravity_scale: component_ref.gravity_scale(),
linear_damp: component_ref.linear_damp(),
linear_velocity: component_ref.linear_velocity(),
mass: component_ref.mass(),
sleeping: component_ref.is_sleeping(),
weight: component_ref.weight(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDRigidBody {
    
}

fn sync_bevy_owned(query: Query<(&GDRigidBody, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<RigidBody>().unwrap();
        component_ref.set_angular_damp(component.angular_damp);
component_ref.set_angular_velocity(component.angular_velocity);
component_ref.set_bounce(component.bounce);
component_ref.set_can_sleep(component.can_sleep);
component_ref.set_contact_monitor(component.contact_monitor);
component_ref.set_max_contacts_reported(component.contacts_reported);
component_ref.set_use_continuous_collision_detection(component.continuous_cd);
component_ref.set_use_custom_integrator(component.custom_integrator);
component_ref.set_friction(component.friction);
component_ref.set_gravity_scale(component.gravity_scale);
component_ref.set_linear_damp(component.linear_damp);
component_ref.set_linear_velocity(component.linear_velocity);
component_ref.set_mass(component.mass);
component_ref.set_sleeping(component.sleeping);
component_ref.set_weight(component.weight);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDRigidBody, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<RigidBody>().unwrap();
        component.angular_damp = component_ref.angular_damp();
component.angular_velocity = component_ref.angular_velocity();
component.bounce = component_ref.bounce();
component.can_sleep = component_ref.is_able_to_sleep();
component.contact_monitor = component_ref.is_contact_monitor_enabled();
component.contacts_reported = component_ref.max_contacts_reported();
component.continuous_cd = component_ref.is_using_continuous_collision_detection();
component.custom_integrator = component_ref.is_using_custom_integrator();
component.friction = component_ref.friction();
component.gravity_scale = component_ref.gravity_scale();
component.linear_damp = component_ref.linear_damp();
component.linear_velocity = component_ref.linear_velocity();
component.mass = component_ref.mass();
component.sleeping = component_ref.is_sleeping();
component.weight = component_ref.weight();
    }
}