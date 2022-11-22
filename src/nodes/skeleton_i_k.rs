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

pub struct SkeletonIKPlugin;

impl Plugin for SkeletonIKPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<SkeletonIK>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a skeleton_i_k.
pub fn is_skeleton_i_k(node: &gdnative::prelude::Node) -> bool {
    node.cast::<SkeletonIK>().is_some()
}

/// A bundle for SkeletonIKs.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDSkeletonIKBundle {
    pub skeleton_i_k: GDSkeletonIK,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDSkeletonIKBundle {
    fn default() -> Self {
        Self {
            skeleton_i_k: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "SkeletonIK".to_string()
            }
        }
    }
}

/// Represents a SkeletonIK.
#[derive(Component)]
pub struct GDSkeletonIK {
    pub interpolation: f64,
pub magnet: Vector3,
pub max_iterations: i64,
pub min_distance: f64,
pub override_tip_basis: bool,
pub root_bone: String,
pub target: Transform,
pub target_node: NodePath,
pub tip_bone: String,
pub use_magnet: bool,
}

impl Default for GDSkeletonIK {
    fn default() -> Self {
        Self {
            interpolation: Default::default(),
magnet: Default::default(),
max_iterations: Default::default(),
min_distance: Default::default(),
override_tip_basis: Default::default(),
root_bone: Default::default(),
target: Transform::IDENTITY,
target_node: Default::default(),
tip_bone: Default::default(),
use_magnet: Default::default(),
        }
    }
}

impl NodeClass for GDSkeletonIK {
    type Parent = GDNode;
    type GodotClass = SkeletonIK;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<SkeletonIK>().unwrap();
        world_commands.insert(entity, GDSkeletonIK {
            interpolation: component_ref.interpolation(),
magnet: component_ref.magnet_position(),
max_iterations: component_ref.max_iterations(),
min_distance: component_ref.min_distance(),
override_tip_basis: component_ref.is_override_tip_basis(),
root_bone: component_ref.root_bone().to_string(),
target: component_ref.target_transform(),
target_node: component_ref.target_node(),
tip_bone: component_ref.tip_bone().to_string(),
use_magnet: component_ref.is_using_magnet(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDSkeletonIK {
    
}

fn sync_bevy_owned(query: Query<(&GDSkeletonIK, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<SkeletonIK>().unwrap();
        component_ref.set_interpolation(component.interpolation);
component_ref.set_magnet_position(component.magnet);
component_ref.set_max_iterations(component.max_iterations);
component_ref.set_min_distance(component.min_distance);
component_ref.set_override_tip_basis(component.override_tip_basis);
component_ref.set_root_bone(component.root_bone.clone());
component_ref.set_target_transform(component.target);
component_ref.set_target_node(component.target_node.to_godot_string());
component_ref.set_tip_bone(component.tip_bone.clone());
component_ref.set_use_magnet(component.use_magnet);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDSkeletonIK, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<SkeletonIK>().unwrap();
        component.interpolation = component_ref.interpolation();
component.magnet = component_ref.magnet_position();
component.max_iterations = component_ref.max_iterations();
component.min_distance = component_ref.min_distance();
component.override_tip_basis = component_ref.is_override_tip_basis();
component.root_bone = component_ref.root_bone().to_string();
component.target = component_ref.target_transform();
component.target_node = component_ref.target_node();
component.tip_bone = component_ref.tip_bone().to_string();
component.use_magnet = component_ref.is_using_magnet();
    }
}