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

pub struct VehicleWheelPlugin;

impl Plugin for VehicleWheelPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<VehicleWheel>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a vehicle_wheel.
pub fn is_vehicle_wheel(node: &gdnative::prelude::Node) -> bool {
    node.cast::<VehicleWheel>().is_some()
}

/// A bundle for VehicleWheels.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDVehicleWheelBundle {
    pub vehicle_wheel: GDVehicleWheel,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDVehicleWheelBundle {
    fn default() -> Self {
        Self {
            vehicle_wheel: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "VehicleWheel".to_string()
            }
        }
    }
}

/// Represents a VehicleWheel.
#[derive(Component)]
pub struct GDVehicleWheel {
    pub brake: f64,
pub damping_compression: f64,
pub damping_relaxation: f64,
pub engine_force: f64,
pub steering: f64,
pub suspension_max_force: f64,
pub suspension_stiffness: f64,
pub suspension_travel: f64,
pub use_as_steering: bool,
pub use_as_traction: bool,
pub wheel_friction_slip: f64,
pub wheel_radius: f64,
pub wheel_rest_length: f64,
pub wheel_roll_influence: f64,
}

impl Default for GDVehicleWheel {
    fn default() -> Self {
        Self {
            brake: Default::default(),
damping_compression: Default::default(),
damping_relaxation: Default::default(),
engine_force: Default::default(),
steering: Default::default(),
suspension_max_force: Default::default(),
suspension_stiffness: Default::default(),
suspension_travel: Default::default(),
use_as_steering: Default::default(),
use_as_traction: Default::default(),
wheel_friction_slip: Default::default(),
wheel_radius: Default::default(),
wheel_rest_length: Default::default(),
wheel_roll_influence: Default::default(),
        }
    }
}

impl NodeClass for GDVehicleWheel {
    type Parent = GDSpatial;
    type GodotClass = VehicleWheel;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<VehicleWheel>().unwrap();
        world_commands.insert(entity, GDVehicleWheel {
            brake: component_ref.brake(),
damping_compression: component_ref.damping_compression(),
damping_relaxation: component_ref.damping_relaxation(),
engine_force: component_ref.engine_force(),
steering: component_ref.steering(),
suspension_max_force: component_ref.suspension_max_force(),
suspension_stiffness: component_ref.suspension_stiffness(),
suspension_travel: component_ref.suspension_travel(),
use_as_steering: component_ref.is_used_as_steering(),
use_as_traction: component_ref.is_used_as_traction(),
wheel_friction_slip: component_ref.friction_slip(),
wheel_radius: component_ref.radius(),
wheel_rest_length: component_ref.suspension_rest_length(),
wheel_roll_influence: component_ref.roll_influence(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDVehicleWheel {
    
}

fn sync_bevy_owned(query: Query<(&GDVehicleWheel, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VehicleWheel>().unwrap();
        component_ref.set_brake(component.brake);
component_ref.set_damping_compression(component.damping_compression);
component_ref.set_damping_relaxation(component.damping_relaxation);
component_ref.set_engine_force(component.engine_force);
component_ref.set_steering(component.steering);
component_ref.set_suspension_max_force(component.suspension_max_force);
component_ref.set_suspension_stiffness(component.suspension_stiffness);
component_ref.set_suspension_travel(component.suspension_travel);
component_ref.set_use_as_steering(component.use_as_steering);
component_ref.set_use_as_traction(component.use_as_traction);
component_ref.set_friction_slip(component.wheel_friction_slip);
component_ref.set_radius(component.wheel_radius);
component_ref.set_suspension_rest_length(component.wheel_rest_length);
component_ref.set_roll_influence(component.wheel_roll_influence);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDVehicleWheel, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VehicleWheel>().unwrap();
        component.brake = component_ref.brake();
component.damping_compression = component_ref.damping_compression();
component.damping_relaxation = component_ref.damping_relaxation();
component.engine_force = component_ref.engine_force();
component.steering = component_ref.steering();
component.suspension_max_force = component_ref.suspension_max_force();
component.suspension_stiffness = component_ref.suspension_stiffness();
component.suspension_travel = component_ref.suspension_travel();
component.use_as_steering = component_ref.is_used_as_steering();
component.use_as_traction = component_ref.is_used_as_traction();
component.wheel_friction_slip = component_ref.friction_slip();
component.wheel_radius = component_ref.radius();
component.wheel_rest_length = component_ref.suspension_rest_length();
component.wheel_roll_influence = component_ref.roll_influence();
    }
}