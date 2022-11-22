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

pub struct ClippedCameraPlugin;

impl Plugin for ClippedCameraPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ClippedCamera>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a clipped_camera.
pub fn is_clipped_camera(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ClippedCamera>().is_some()
}

/// A bundle for ClippedCameras.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDClippedCameraBundle {
    pub clipped_camera: GDClippedCamera,
    pub node: GDNode,
pub spatial: GDSpatial,
pub camera: GDCamera,
    pub true_type: TrueNodeType,
}

impl Default for GDClippedCameraBundle {
    fn default() -> Self {
        Self {
            clipped_camera: Default::default(),
            node: Default::default(),
spatial: Default::default(),
camera: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ClippedCamera".to_string()
            }
        }
    }
}

/// Represents a ClippedCamera.
#[derive(Component)]
pub struct GDClippedCamera {
    pub clip_to_areas: bool,
pub clip_to_bodies: bool,
pub collision_mask: i64,
pub margin: f64,
}

impl Default for GDClippedCamera {
    fn default() -> Self {
        Self {
            clip_to_areas: Default::default(),
clip_to_bodies: Default::default(),
collision_mask: Default::default(),
margin: Default::default(),
        }
    }
}

impl NodeClass for GDClippedCamera {
    type Parent = GDCamera;
    type GodotClass = ClippedCamera;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ClippedCamera>().unwrap();
        world_commands.insert(entity, GDClippedCamera {
            clip_to_areas: component_ref.is_clip_to_areas_enabled(),
clip_to_bodies: component_ref.is_clip_to_bodies_enabled(),
collision_mask: component_ref.collision_mask(),
margin: component_ref.margin(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDClippedCamera {
    
}

fn sync_bevy_owned(query: Query<(&GDClippedCamera, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ClippedCamera>().unwrap();
        component_ref.set_clip_to_areas(component.clip_to_areas);
component_ref.set_clip_to_bodies(component.clip_to_bodies);
component_ref.set_collision_mask(component.collision_mask);
component_ref.set_margin(component.margin);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDClippedCamera, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ClippedCamera>().unwrap();
        component.clip_to_areas = component_ref.is_clip_to_areas_enabled();
component.clip_to_bodies = component_ref.is_clip_to_bodies_enabled();
component.collision_mask = component_ref.collision_mask();
component.margin = component_ref.margin();
    }
}