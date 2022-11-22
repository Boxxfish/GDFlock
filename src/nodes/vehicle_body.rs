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

pub struct VehicleBodyPlugin;

impl Plugin for VehicleBodyPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<VehicleBody>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a vehicle_body.
pub fn is_vehicle_body(node: &gdnative::prelude::Node) -> bool {
    node.cast::<VehicleBody>().is_some()
}

/// A bundle for VehicleBodys.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDVehicleBodyBundle {
    pub vehicle_body: GDVehicleBody,
    pub node: GDNode,
pub spatial: GDSpatial,
pub collision_object: GDCollisionObject,
pub physics_body: GDPhysicsBody,
pub rigid_body: GDRigidBody,
    pub true_type: TrueNodeType,
}

impl Default for GDVehicleBodyBundle {
    fn default() -> Self {
        Self {
            vehicle_body: Default::default(),
            node: Default::default(),
spatial: Default::default(),
collision_object: Default::default(),
physics_body: Default::default(),
rigid_body: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "VehicleBody".to_string()
            }
        }
    }
}

/// Represents a VehicleBody.
#[derive(Component)]
pub struct GDVehicleBody {
    pub brake: f64,
pub engine_force: f64,
pub steering: f64,
}

impl Default for GDVehicleBody {
    fn default() -> Self {
        Self {
            brake: Default::default(),
engine_force: Default::default(),
steering: Default::default(),
        }
    }
}

impl NodeClass for GDVehicleBody {
    type Parent = GDRigidBody;
    type GodotClass = VehicleBody;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<VehicleBody>().unwrap();
        world_commands.insert(entity, GDVehicleBody {
            brake: component_ref.brake(),
engine_force: component_ref.engine_force(),
steering: component_ref.steering(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDVehicleBody {
    
}

fn sync_bevy_owned(query: Query<(&GDVehicleBody, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VehicleBody>().unwrap();
        component_ref.set_brake(component.brake);
component_ref.set_engine_force(component.engine_force);
component_ref.set_steering(component.steering);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDVehicleBody, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VehicleBody>().unwrap();
        component.brake = component_ref.brake();
component.engine_force = component_ref.engine_force();
component.steering = component_ref.steering();
    }
}