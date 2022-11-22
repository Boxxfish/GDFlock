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

pub struct SoftBodyPlugin;

impl Plugin for SoftBodyPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<SoftBody>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a soft_body.
pub fn is_soft_body(node: &gdnative::prelude::Node) -> bool {
    node.cast::<SoftBody>().is_some()
}

/// A bundle for SoftBodys.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDSoftBodyBundle {
    pub soft_body: GDSoftBody,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub geometry_instance: GDGeometryInstance,
pub mesh_instance: GDMeshInstance,
    pub true_type: TrueNodeType,
}

impl Default for GDSoftBodyBundle {
    fn default() -> Self {
        Self {
            soft_body: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
geometry_instance: Default::default(),
mesh_instance: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "SoftBody".to_string()
            }
        }
    }
}

/// Represents a SoftBody.
#[derive(Component)]
pub struct GDSoftBody {
    pub areaAngular_stiffness: f64,
pub collision_layer: i64,
pub collision_mask: i64,
pub damping_coefficient: f64,
pub drag_coefficient: f64,
pub linear_stiffness: f64,
pub parent_collision_ignore: NodePath,
pub physics_enabled: bool,
pub pose_matching_coefficient: f64,
pub pressure_coefficient: f64,
pub ray_pickable: bool,
pub simulation_precision: i64,
pub total_mass: f64,
pub volume_stiffness: f64,
}

impl Default for GDSoftBody {
    fn default() -> Self {
        Self {
            areaAngular_stiffness: Default::default(),
collision_layer: Default::default(),
collision_mask: Default::default(),
damping_coefficient: Default::default(),
drag_coefficient: Default::default(),
linear_stiffness: Default::default(),
parent_collision_ignore: Default::default(),
physics_enabled: Default::default(),
pose_matching_coefficient: Default::default(),
pressure_coefficient: Default::default(),
ray_pickable: Default::default(),
simulation_precision: Default::default(),
total_mass: Default::default(),
volume_stiffness: Default::default(),
        }
    }
}

impl NodeClass for GDSoftBody {
    type Parent = GDMeshInstance;
    type GodotClass = SoftBody;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<SoftBody>().unwrap();
        world_commands.insert(entity, GDSoftBody {
            areaAngular_stiffness: component_ref.areaAngular_stiffness(),
collision_layer: component_ref.collision_layer(),
collision_mask: component_ref.collision_mask(),
damping_coefficient: component_ref.damping_coefficient(),
drag_coefficient: component_ref.drag_coefficient(),
linear_stiffness: component_ref.linear_stiffness(),
parent_collision_ignore: component_ref.parent_collision_ignore(),
physics_enabled: component_ref.is_physics_enabled(),
pose_matching_coefficient: component_ref.pose_matching_coefficient(),
pressure_coefficient: component_ref.pressure_coefficient(),
ray_pickable: component_ref.is_ray_pickable(),
simulation_precision: component_ref.simulation_precision(),
total_mass: component_ref.total_mass(),
volume_stiffness: component_ref.volume_stiffness(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDSoftBody {
    
}

fn sync_bevy_owned(query: Query<(&GDSoftBody, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<SoftBody>().unwrap();
        component_ref.set_areaAngular_stiffness(component.areaAngular_stiffness);
component_ref.set_collision_layer(component.collision_layer);
component_ref.set_collision_mask(component.collision_mask);
component_ref.set_damping_coefficient(component.damping_coefficient);
component_ref.set_drag_coefficient(component.drag_coefficient);
component_ref.set_linear_stiffness(component.linear_stiffness);
component_ref.set_parent_collision_ignore(component.parent_collision_ignore.to_godot_string());
component_ref.set_physics_enabled(component.physics_enabled);
component_ref.set_pose_matching_coefficient(component.pose_matching_coefficient);
component_ref.set_pressure_coefficient(component.pressure_coefficient);
component_ref.set_ray_pickable(component.ray_pickable);
component_ref.set_simulation_precision(component.simulation_precision);
component_ref.set_total_mass(component.total_mass);
component_ref.set_volume_stiffness(component.volume_stiffness);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDSoftBody, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<SoftBody>().unwrap();
        component.areaAngular_stiffness = component_ref.areaAngular_stiffness();
component.collision_layer = component_ref.collision_layer();
component.collision_mask = component_ref.collision_mask();
component.damping_coefficient = component_ref.damping_coefficient();
component.drag_coefficient = component_ref.drag_coefficient();
component.linear_stiffness = component_ref.linear_stiffness();
component.parent_collision_ignore = component_ref.parent_collision_ignore();
component.physics_enabled = component_ref.is_physics_enabled();
component.pose_matching_coefficient = component_ref.pose_matching_coefficient();
component.pressure_coefficient = component_ref.pressure_coefficient();
component.ray_pickable = component_ref.is_ray_pickable();
component.simulation_precision = component_ref.simulation_precision();
component.total_mass = component_ref.total_mass();
component.volume_stiffness = component_ref.volume_stiffness();
    }
}