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

pub struct AreaPlugin;

impl Plugin for AreaPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Area>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a area.
pub fn is_area(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Area>().is_some()
}

/// A bundle for Areas.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDAreaBundle {
    pub area: GDArea,
    pub node: GDNode,
pub spatial: GDSpatial,
pub collision_object: GDCollisionObject,
    pub true_type: TrueNodeType,
}

impl Default for GDAreaBundle {
    fn default() -> Self {
        Self {
            area: Default::default(),
            node: Default::default(),
spatial: Default::default(),
collision_object: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Area".to_string()
            }
        }
    }
}

/// Represents a Area.
#[derive(Component)]
pub struct GDArea {
    pub angular_damp: f64,
pub audio_bus_name: String,
pub audio_bus_override: bool,
pub gravity: f64,
pub gravity_distance_scale: f64,
pub gravity_point: bool,
pub gravity_vec: Vector3,
pub linear_damp: f64,
pub monitorable: bool,
pub monitoring: bool,
pub priority: f64,
pub reverb_bus_amount: f64,
pub reverb_bus_enable: bool,
pub reverb_bus_name: String,
pub reverb_bus_uniformity: f64,
}

impl Default for GDArea {
    fn default() -> Self {
        Self {
            angular_damp: Default::default(),
audio_bus_name: Default::default(),
audio_bus_override: Default::default(),
gravity: Default::default(),
gravity_distance_scale: Default::default(),
gravity_point: Default::default(),
gravity_vec: Default::default(),
linear_damp: Default::default(),
monitorable: Default::default(),
monitoring: Default::default(),
priority: Default::default(),
reverb_bus_amount: Default::default(),
reverb_bus_enable: Default::default(),
reverb_bus_name: Default::default(),
reverb_bus_uniformity: Default::default(),
        }
    }
}

impl NodeClass for GDArea {
    type Parent = GDCollisionObject;
    type GodotClass = Area;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Area>().unwrap();
        world_commands.insert(entity, GDArea {
            angular_damp: component_ref.angular_damp(),
audio_bus_name: component_ref.audio_bus().to_string(),
audio_bus_override: component_ref.is_overriding_audio_bus(),
gravity: component_ref.gravity(),
gravity_distance_scale: component_ref.gravity_distance_scale(),
gravity_point: component_ref.is_gravity_a_point(),
gravity_vec: component_ref.gravity_vector(),
linear_damp: component_ref.linear_damp(),
monitorable: component_ref.is_monitorable(),
monitoring: component_ref.is_monitoring(),
priority: component_ref.priority(),
reverb_bus_amount: component_ref.reverb_amount(),
reverb_bus_enable: component_ref.is_using_reverb_bus(),
reverb_bus_name: component_ref.reverb_bus().to_string(),
reverb_bus_uniformity: component_ref.reverb_uniformity(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDArea {
    
}

fn sync_bevy_owned(query: Query<(&GDArea, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Area>().unwrap();
        component_ref.set_angular_damp(component.angular_damp);
component_ref.set_audio_bus(component.audio_bus_name.clone());
component_ref.set_audio_bus_override(component.audio_bus_override);
component_ref.set_gravity(component.gravity);
component_ref.set_gravity_distance_scale(component.gravity_distance_scale);
component_ref.set_gravity_is_point(component.gravity_point);
component_ref.set_gravity_vector(component.gravity_vec);
component_ref.set_linear_damp(component.linear_damp);
component_ref.set_monitorable(component.monitorable);
component_ref.set_monitoring(component.monitoring);
component_ref.set_priority(component.priority);
component_ref.set_reverb_amount(component.reverb_bus_amount);
component_ref.set_use_reverb_bus(component.reverb_bus_enable);
component_ref.set_reverb_bus(component.reverb_bus_name.clone());
component_ref.set_reverb_uniformity(component.reverb_bus_uniformity);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDArea, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Area>().unwrap();
        component.angular_damp = component_ref.angular_damp();
component.audio_bus_name = component_ref.audio_bus().to_string();
component.audio_bus_override = component_ref.is_overriding_audio_bus();
component.gravity = component_ref.gravity();
component.gravity_distance_scale = component_ref.gravity_distance_scale();
component.gravity_point = component_ref.is_gravity_a_point();
component.gravity_vec = component_ref.gravity_vector();
component.linear_damp = component_ref.linear_damp();
component.monitorable = component_ref.is_monitorable();
component.monitoring = component_ref.is_monitoring();
component.priority = component_ref.priority();
component.reverb_bus_amount = component_ref.reverb_amount();
component.reverb_bus_enable = component_ref.is_using_reverb_bus();
component.reverb_bus_name = component_ref.reverb_bus().to_string();
component.reverb_bus_uniformity = component_ref.reverb_uniformity();
    }
}