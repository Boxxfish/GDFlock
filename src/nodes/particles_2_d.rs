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

pub struct Particles2DPlugin;

impl Plugin for Particles2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Particles2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a particles_2_d.
pub fn is_particles_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Particles2D>().is_some()
}

/// A bundle for Particles2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDParticles2DBundle {
    pub particles_2_d: GDParticles2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDParticles2DBundle {
    fn default() -> Self {
        Self {
            particles_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Particles2D".to_string()
            }
        }
    }
}

/// Represents a Particles2D.
#[derive(Component)]
pub struct GDParticles2D {
    pub amount: i64,
pub emitting: bool,
pub explosiveness: f64,
pub fixed_fps: i64,
pub fract_delta: bool,
pub lifetime: f64,
pub local_coords: bool,
pub normal_map: Option<Ref<Texture>>,
pub one_shot: bool,
pub preprocess: f64,
pub randomness: f64,
pub speed_scale: f64,
pub texture: Option<Ref<Texture>>,
pub visibility_rect: Rect2,
}

impl Default for GDParticles2D {
    fn default() -> Self {
        Self {
            amount: Default::default(),
emitting: Default::default(),
explosiveness: Default::default(),
fixed_fps: Default::default(),
fract_delta: Default::default(),
lifetime: Default::default(),
local_coords: Default::default(),
normal_map: Default::default(),
one_shot: Default::default(),
preprocess: Default::default(),
randomness: Default::default(),
speed_scale: Default::default(),
texture: Default::default(),
visibility_rect: Rect2::from_components(0.0, 0.0, 0.0, 0.0),
        }
    }
}

impl NodeClass for GDParticles2D {
    type Parent = GDNode2D;
    type GodotClass = Particles2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Particles2D>().unwrap();
        world_commands.insert(entity, GDParticles2D {
            amount: component_ref.amount(),
emitting: component_ref.is_emitting(),
explosiveness: component_ref.explosiveness_ratio(),
fixed_fps: component_ref.fixed_fps(),
fract_delta: component_ref.fractional_delta(),
lifetime: component_ref.lifetime(),
local_coords: component_ref.use_local_coordinates(),
normal_map: component_ref.normal_map(),
one_shot: component_ref.one_shot(),
preprocess: component_ref.pre_process_time(),
randomness: component_ref.randomness_ratio(),
speed_scale: component_ref.speed_scale(),
texture: component_ref.texture(),
visibility_rect: component_ref.visibility_rect(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDParticles2D {
    
}

fn sync_bevy_owned(query: Query<(&GDParticles2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Particles2D>().unwrap();
        component_ref.set_amount(component.amount);
component_ref.set_emitting(component.emitting);
component_ref.set_explosiveness_ratio(component.explosiveness);
component_ref.set_fixed_fps(component.fixed_fps);
component_ref.set_fractional_delta(component.fract_delta);
component_ref.set_lifetime(component.lifetime);
component_ref.set_use_local_coordinates(component.local_coords);
component_ref.set_normal_map(component.normal_map.as_ref().unwrap().clone());
component_ref.set_one_shot(component.one_shot);
component_ref.set_pre_process_time(component.preprocess);
component_ref.set_randomness_ratio(component.randomness);
component_ref.set_speed_scale(component.speed_scale);
component_ref.set_texture(component.texture.as_ref().unwrap().clone());
component_ref.set_visibility_rect(component.visibility_rect);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDParticles2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Particles2D>().unwrap();
        component.amount = component_ref.amount();
component.emitting = component_ref.is_emitting();
component.explosiveness = component_ref.explosiveness_ratio();
component.fixed_fps = component_ref.fixed_fps();
component.fract_delta = component_ref.fractional_delta();
component.lifetime = component_ref.lifetime();
component.local_coords = component_ref.use_local_coordinates();
component.normal_map = component_ref.normal_map();
component.one_shot = component_ref.one_shot();
component.preprocess = component_ref.pre_process_time();
component.randomness = component_ref.randomness_ratio();
component.speed_scale = component_ref.speed_scale();
component.texture = component_ref.texture();
component.visibility_rect = component_ref.visibility_rect();
    }
}