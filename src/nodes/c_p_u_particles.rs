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

pub struct CPUParticlesPlugin;

impl Plugin for CPUParticlesPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<CPUParticles>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a c_p_u_particles.
pub fn is_c_p_u_particles(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CPUParticles>().is_some()
}

/// A bundle for CPUParticless.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCPUParticlesBundle {
    pub c_p_u_particles: GDCPUParticles,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub geometry_instance: GDGeometryInstance,
    pub true_type: TrueNodeType,
}

impl Default for GDCPUParticlesBundle {
    fn default() -> Self {
        Self {
            c_p_u_particles: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
geometry_instance: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CPUParticles".to_string()
            }
        }
    }
}

/// Represents a CPUParticles.
#[derive(Component)]
pub struct GDCPUParticles {
    pub amount: i64,
pub color: Color,
pub direction: Vector3,
pub emission_box_extents: Vector3,
pub emission_colors: Vec<Color>,
pub emission_normals: Vec<Vector3>,
pub emission_points: Vec<Vector3>,
pub emission_ring_axis: Vector3,
pub emission_ring_height: f64,
pub emission_ring_inner_radius: f64,
pub emission_ring_radius: f64,
pub emission_sphere_radius: f64,
pub emitting: bool,
pub explosiveness: f64,
pub fixed_fps: i64,
pub flatness: f64,
pub fract_delta: bool,
pub gravity: Vector3,
pub lifetime: f64,
pub lifetime_randomness: f64,
pub local_coords: bool,
pub mesh: Option<Ref<Mesh>>,
pub one_shot: bool,
pub preprocess: f64,
pub randomness: f64,
pub speed_scale: f64,
pub spread: f64,
}

impl Default for GDCPUParticles {
    fn default() -> Self {
        Self {
            amount: Default::default(),
color: Color::from_rgb(0.0, 0.0, 0.0),
direction: Default::default(),
emission_box_extents: Default::default(),
emission_colors: Default::default(),
emission_normals: Default::default(),
emission_points: Default::default(),
emission_ring_axis: Default::default(),
emission_ring_height: Default::default(),
emission_ring_inner_radius: Default::default(),
emission_ring_radius: Default::default(),
emission_sphere_radius: Default::default(),
emitting: Default::default(),
explosiveness: Default::default(),
fixed_fps: Default::default(),
flatness: Default::default(),
fract_delta: Default::default(),
gravity: Default::default(),
lifetime: Default::default(),
lifetime_randomness: Default::default(),
local_coords: Default::default(),
mesh: Default::default(),
one_shot: Default::default(),
preprocess: Default::default(),
randomness: Default::default(),
speed_scale: Default::default(),
spread: Default::default(),
        }
    }
}

impl NodeClass for GDCPUParticles {
    type Parent = GDGeometryInstance;
    type GodotClass = CPUParticles;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CPUParticles>().unwrap();
        world_commands.insert(entity, GDCPUParticles {
            amount: component_ref.amount(),
color: component_ref.color(),
direction: component_ref.direction(),
emission_box_extents: component_ref.emission_box_extents(),
emission_colors: component_ref.emission_colors().to_vec(),
emission_normals: component_ref.emission_normals().to_vec(),
emission_points: component_ref.emission_points().to_vec(),
emission_ring_axis: component_ref.emission_ring_axis(),
emission_ring_height: component_ref.emission_ring_height(),
emission_ring_inner_radius: component_ref.emission_ring_inner_radius(),
emission_ring_radius: component_ref.emission_ring_radius(),
emission_sphere_radius: component_ref.emission_sphere_radius(),
emitting: component_ref.is_emitting(),
explosiveness: component_ref.explosiveness_ratio(),
fixed_fps: component_ref.fixed_fps(),
flatness: component_ref.flatness(),
fract_delta: component_ref.fractional_delta(),
gravity: component_ref.gravity(),
lifetime: component_ref.lifetime(),
lifetime_randomness: component_ref.lifetime_randomness(),
local_coords: component_ref.use_local_coordinates(),
mesh: component_ref.mesh(),
one_shot: component_ref.one_shot(),
preprocess: component_ref.pre_process_time(),
randomness: component_ref.randomness_ratio(),
speed_scale: component_ref.speed_scale(),
spread: component_ref.spread(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCPUParticles {
    
}

fn sync_bevy_owned(query: Query<(&GDCPUParticles, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CPUParticles>().unwrap();
        component_ref.set_amount(component.amount);
component_ref.set_color(component.color);
component_ref.set_direction(component.direction);
component_ref.set_emission_box_extents(component.emission_box_extents);
component_ref.set_emission_colors(ColorArray::from_vec(component.emission_colors.clone()));
component_ref.set_emission_normals(Vector3Array::from_vec(component.emission_normals.clone()));
component_ref.set_emission_points(Vector3Array::from_vec(component.emission_points.clone()));
component_ref.set_emission_ring_axis(component.emission_ring_axis);
component_ref.set_emission_ring_height(component.emission_ring_height);
component_ref.set_emission_ring_inner_radius(component.emission_ring_inner_radius);
component_ref.set_emission_ring_radius(component.emission_ring_radius);
component_ref.set_emission_sphere_radius(component.emission_sphere_radius);
component_ref.set_emitting(component.emitting);
component_ref.set_explosiveness_ratio(component.explosiveness);
component_ref.set_fixed_fps(component.fixed_fps);
component_ref.set_flatness(component.flatness);
component_ref.set_fractional_delta(component.fract_delta);
component_ref.set_gravity(component.gravity);
component_ref.set_lifetime(component.lifetime);
component_ref.set_lifetime_randomness(component.lifetime_randomness);
component_ref.set_use_local_coordinates(component.local_coords);
component_ref.set_mesh(component.mesh.as_ref().unwrap().clone());
component_ref.set_one_shot(component.one_shot);
component_ref.set_pre_process_time(component.preprocess);
component_ref.set_randomness_ratio(component.randomness);
component_ref.set_speed_scale(component.speed_scale);
component_ref.set_spread(component.spread);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCPUParticles, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CPUParticles>().unwrap();
        component.amount = component_ref.amount();
component.color = component_ref.color();
component.direction = component_ref.direction();
component.emission_box_extents = component_ref.emission_box_extents();
component.emission_colors = component_ref.emission_colors().to_vec();
component.emission_normals = component_ref.emission_normals().to_vec();
component.emission_points = component_ref.emission_points().to_vec();
component.emission_ring_axis = component_ref.emission_ring_axis();
component.emission_ring_height = component_ref.emission_ring_height();
component.emission_ring_inner_radius = component_ref.emission_ring_inner_radius();
component.emission_ring_radius = component_ref.emission_ring_radius();
component.emission_sphere_radius = component_ref.emission_sphere_radius();
component.emitting = component_ref.is_emitting();
component.explosiveness = component_ref.explosiveness_ratio();
component.fixed_fps = component_ref.fixed_fps();
component.flatness = component_ref.flatness();
component.fract_delta = component_ref.fractional_delta();
component.gravity = component_ref.gravity();
component.lifetime = component_ref.lifetime();
component.lifetime_randomness = component_ref.lifetime_randomness();
component.local_coords = component_ref.use_local_coordinates();
component.mesh = component_ref.mesh();
component.one_shot = component_ref.one_shot();
component.preprocess = component_ref.pre_process_time();
component.randomness = component_ref.randomness_ratio();
component.speed_scale = component_ref.speed_scale();
component.spread = component_ref.spread();
    }
}