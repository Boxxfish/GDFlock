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

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Particles>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a particles.
pub fn is_particles(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Particles>().is_some()
}

/// A bundle for Particless.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDParticlesBundle {
    pub particles: GDParticles,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub geometry_instance: GDGeometryInstance,
    pub true_type: TrueNodeType,
}

impl Default for GDParticlesBundle {
    fn default() -> Self {
        Self {
            particles: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
geometry_instance: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Particles".to_string()
            }
        }
    }
}

/// Represents a Particles.
#[derive(Component)]
pub struct GDParticles {
    pub amount: i64,
pub draw_passes: i64,
pub emitting: bool,
pub explosiveness: f64,
pub fixed_fps: i64,
pub fract_delta: bool,
pub lifetime: f64,
pub local_coords: bool,
pub one_shot: bool,
pub preprocess: f64,
pub randomness: f64,
pub speed_scale: f64,
pub visibility_aabb: Aabb,
}

impl Default for GDParticles {
    fn default() -> Self {
        Self {
            amount: Default::default(),
draw_passes: Default::default(),
emitting: Default::default(),
explosiveness: Default::default(),
fixed_fps: Default::default(),
fract_delta: Default::default(),
lifetime: Default::default(),
local_coords: Default::default(),
one_shot: Default::default(),
preprocess: Default::default(),
randomness: Default::default(),
speed_scale: Default::default(),
visibility_aabb: Default::default(),
        }
    }
}

impl NodeClass for GDParticles {
    type Parent = GDGeometryInstance;
    type GodotClass = Particles;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Particles>().unwrap();
        world_commands.insert(entity, GDParticles {
            amount: component_ref.amount(),
draw_passes: component_ref.draw_passes(),
emitting: component_ref.is_emitting(),
explosiveness: component_ref.explosiveness_ratio(),
fixed_fps: component_ref.fixed_fps(),
fract_delta: component_ref.fractional_delta(),
lifetime: component_ref.lifetime(),
local_coords: component_ref.use_local_coordinates(),
one_shot: component_ref.one_shot(),
preprocess: component_ref.pre_process_time(),
randomness: component_ref.randomness_ratio(),
speed_scale: component_ref.speed_scale(),
visibility_aabb: component_ref.visibility_aabb(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDParticles {
    
}

fn sync_bevy_owned(query: Query<(&GDParticles, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Particles>().unwrap();
        component_ref.set_amount(component.amount);
component_ref.set_draw_passes(component.draw_passes);
component_ref.set_emitting(component.emitting);
component_ref.set_explosiveness_ratio(component.explosiveness);
component_ref.set_fixed_fps(component.fixed_fps);
component_ref.set_fractional_delta(component.fract_delta);
component_ref.set_lifetime(component.lifetime);
component_ref.set_use_local_coordinates(component.local_coords);
component_ref.set_one_shot(component.one_shot);
component_ref.set_pre_process_time(component.preprocess);
component_ref.set_randomness_ratio(component.randomness);
component_ref.set_speed_scale(component.speed_scale);
component_ref.set_visibility_aabb(component.visibility_aabb);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDParticles, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Particles>().unwrap();
        component.amount = component_ref.amount();
component.draw_passes = component_ref.draw_passes();
component.emitting = component_ref.is_emitting();
component.explosiveness = component_ref.explosiveness_ratio();
component.fixed_fps = component_ref.fixed_fps();
component.fract_delta = component_ref.fractional_delta();
component.lifetime = component_ref.lifetime();
component.local_coords = component_ref.use_local_coordinates();
component.one_shot = component_ref.one_shot();
component.preprocess = component_ref.pre_process_time();
component.randomness = component_ref.randomness_ratio();
component.speed_scale = component_ref.speed_scale();
component.visibility_aabb = component_ref.visibility_aabb();
    }
}