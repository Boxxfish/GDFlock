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

pub struct InterpolatedCameraPlugin;

impl Plugin for InterpolatedCameraPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<InterpolatedCamera>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a interpolated_camera.
pub fn is_interpolated_camera(node: &gdnative::prelude::Node) -> bool {
    node.cast::<InterpolatedCamera>().is_some()
}

/// A bundle for InterpolatedCameras.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDInterpolatedCameraBundle {
    pub interpolated_camera: GDInterpolatedCamera,
    pub node: GDNode,
pub spatial: GDSpatial,
pub camera: GDCamera,
    pub true_type: TrueNodeType,
}

impl Default for GDInterpolatedCameraBundle {
    fn default() -> Self {
        Self {
            interpolated_camera: Default::default(),
            node: Default::default(),
spatial: Default::default(),
camera: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "InterpolatedCamera".to_string()
            }
        }
    }
}

/// Represents a InterpolatedCamera.
#[derive(Component)]
pub struct GDInterpolatedCamera {
    pub enabled: bool,
pub speed: f64,
pub target: NodePath,
}

impl Default for GDInterpolatedCamera {
    fn default() -> Self {
        Self {
            enabled: Default::default(),
speed: Default::default(),
target: Default::default(),
        }
    }
}

impl NodeClass for GDInterpolatedCamera {
    type Parent = GDCamera;
    type GodotClass = InterpolatedCamera;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<InterpolatedCamera>().unwrap();
        world_commands.insert(entity, GDInterpolatedCamera {
            enabled: component_ref.is_interpolation_enabled(),
speed: component_ref.speed(),
target: component_ref.target_path(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDInterpolatedCamera {
    
}

fn sync_bevy_owned(query: Query<(&GDInterpolatedCamera, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<InterpolatedCamera>().unwrap();
        component_ref.set_interpolation_enabled(component.enabled);
component_ref.set_speed(component.speed);
component_ref.set_target_path(component.target.to_godot_string());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDInterpolatedCamera, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<InterpolatedCamera>().unwrap();
        component.enabled = component_ref.is_interpolation_enabled();
component.speed = component_ref.speed();
component.target = component_ref.target_path();
    }
}