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

pub struct CPUParticles2DPlugin;

impl Plugin for CPUParticles2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<CPUParticles2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a c_p_u_particles_2_d.
pub fn is_c_p_u_particles_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CPUParticles2D>().is_some()
}

/// A bundle for CPUParticles2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCPUParticles2DBundle {
    pub c_p_u_particles_2_d: GDCPUParticles2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDCPUParticles2DBundle {
    fn default() -> Self {
        Self {
            c_p_u_particles_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CPUParticles2D".to_string()
            }
        }
    }
}

/// Represents a CPUParticles2D.
#[derive(Component)]
pub struct GDCPUParticles2D {
    pub amount: i64,
pub color: Color,
pub direction: Vector2,
pub emission_colors: Vec<Color>,
pub emission_normals: Vec<Vector2>,
pub emission_points: Vec<Vector2>,
pub emission_rect_extents: Vector2,
pub emission_sphere_radius: f64,
pub emitting: bool,
pub explosiveness: f64,
pub fixed_fps: i64,
pub fract_delta: bool,
pub gravity: Vector2,
pub lifetime: f64,
pub lifetime_randomness: f64,
pub local_coords: bool,
pub normalmap: Option<Ref<Texture>>,
pub one_shot: bool,
pub preprocess: f64,
pub randomness: f64,
pub speed_scale: f64,
pub spread: f64,
pub texture: Option<Ref<Texture>>,
}

impl Default for GDCPUParticles2D {
    fn default() -> Self {
        Self {
            amount: Default::default(),
color: Color::from_rgb(0.0, 0.0, 0.0),
direction: Default::default(),
emission_colors: Default::default(),
emission_normals: Default::default(),
emission_points: Default::default(),
emission_rect_extents: Default::default(),
emission_sphere_radius: Default::default(),
emitting: Default::default(),
explosiveness: Default::default(),
fixed_fps: Default::default(),
fract_delta: Default::default(),
gravity: Default::default(),
lifetime: Default::default(),
lifetime_randomness: Default::default(),
local_coords: Default::default(),
normalmap: Default::default(),
one_shot: Default::default(),
preprocess: Default::default(),
randomness: Default::default(),
speed_scale: Default::default(),
spread: Default::default(),
texture: Default::default(),
        }
    }
}

impl NodeClass for GDCPUParticles2D {
    type Parent = GDNode2D;
    type GodotClass = CPUParticles2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CPUParticles2D>().unwrap();
        world_commands.insert(entity, GDCPUParticles2D {
            amount: component_ref.amount(),
color: component_ref.color(),
direction: component_ref.direction(),
emission_colors: component_ref.emission_colors().to_vec(),
emission_normals: component_ref.emission_normals().to_vec(),
emission_points: component_ref.emission_points().to_vec(),
emission_rect_extents: component_ref.emission_rect_extents(),
emission_sphere_radius: component_ref.emission_sphere_radius(),
emitting: component_ref.is_emitting(),
explosiveness: component_ref.explosiveness_ratio(),
fixed_fps: component_ref.fixed_fps(),
fract_delta: component_ref.fractional_delta(),
gravity: component_ref.gravity(),
lifetime: component_ref.lifetime(),
lifetime_randomness: component_ref.lifetime_randomness(),
local_coords: component_ref.use_local_coordinates(),
normalmap: component_ref.normalmap(),
one_shot: component_ref.one_shot(),
preprocess: component_ref.pre_process_time(),
randomness: component_ref.randomness_ratio(),
speed_scale: component_ref.speed_scale(),
spread: component_ref.spread(),
texture: component_ref.texture(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCPUParticles2D {
    
}

fn sync_bevy_owned(query: Query<(&GDCPUParticles2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CPUParticles2D>().unwrap();
        component_ref.set_amount(component.amount);
component_ref.set_color(component.color);
component_ref.set_direction(component.direction);
component_ref.set_emission_colors(ColorArray::from_vec(component.emission_colors.clone()));
component_ref.set_emission_normals(Vector2Array::from_vec(component.emission_normals.clone()));
component_ref.set_emission_points(Vector2Array::from_vec(component.emission_points.clone()));
component_ref.set_emission_rect_extents(component.emission_rect_extents);
component_ref.set_emission_sphere_radius(component.emission_sphere_radius);
component_ref.set_emitting(component.emitting);
component_ref.set_explosiveness_ratio(component.explosiveness);
component_ref.set_fixed_fps(component.fixed_fps);
component_ref.set_fractional_delta(component.fract_delta);
component_ref.set_gravity(component.gravity);
component_ref.set_lifetime(component.lifetime);
component_ref.set_lifetime_randomness(component.lifetime_randomness);
component_ref.set_use_local_coordinates(component.local_coords);
component_ref.set_normalmap(component.normalmap.as_ref().unwrap().clone());
component_ref.set_one_shot(component.one_shot);
component_ref.set_pre_process_time(component.preprocess);
component_ref.set_randomness_ratio(component.randomness);
component_ref.set_speed_scale(component.speed_scale);
component_ref.set_spread(component.spread);
component_ref.set_texture(component.texture.as_ref().unwrap().clone());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCPUParticles2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CPUParticles2D>().unwrap();
        component.amount = component_ref.amount();
component.color = component_ref.color();
component.direction = component_ref.direction();
component.emission_colors = component_ref.emission_colors().to_vec();
component.emission_normals = component_ref.emission_normals().to_vec();
component.emission_points = component_ref.emission_points().to_vec();
component.emission_rect_extents = component_ref.emission_rect_extents();
component.emission_sphere_radius = component_ref.emission_sphere_radius();
component.emitting = component_ref.is_emitting();
component.explosiveness = component_ref.explosiveness_ratio();
component.fixed_fps = component_ref.fixed_fps();
component.fract_delta = component_ref.fractional_delta();
component.gravity = component_ref.gravity();
component.lifetime = component_ref.lifetime();
component.lifetime_randomness = component_ref.lifetime_randomness();
component.local_coords = component_ref.use_local_coordinates();
component.normalmap = component_ref.normalmap();
component.one_shot = component_ref.one_shot();
component.preprocess = component_ref.pre_process_time();
component.randomness = component_ref.randomness_ratio();
component.speed_scale = component_ref.speed_scale();
component.spread = component_ref.spread();
component.texture = component_ref.texture();
    }
}