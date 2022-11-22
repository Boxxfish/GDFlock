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

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Camera>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a camera.
pub fn is_camera(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Camera>().is_some()
}

/// A bundle for Cameras.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCameraBundle {
    pub camera: GDCamera,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDCameraBundle {
    fn default() -> Self {
        Self {
            camera: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Camera".to_string()
            }
        }
    }
}

/// Represents a Camera.
#[derive(Component)]
pub struct GDCamera {
    pub cull_mask: i64,
pub current: bool,
pub far: f64,
pub fov: f64,
pub frustum_offset: Vector2,
pub h_offset: f64,
pub near: f64,
pub size: f64,
pub v_offset: f64,
}

impl Default for GDCamera {
    fn default() -> Self {
        Self {
            cull_mask: Default::default(),
current: Default::default(),
far: Default::default(),
fov: Default::default(),
frustum_offset: Default::default(),
h_offset: Default::default(),
near: Default::default(),
size: Default::default(),
v_offset: Default::default(),
        }
    }
}

impl NodeClass for GDCamera {
    type Parent = GDSpatial;
    type GodotClass = Camera;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Camera>().unwrap();
        world_commands.insert(entity, GDCamera {
            cull_mask: component_ref.cull_mask(),
current: component_ref.is_current(),
far: component_ref.zfar(),
fov: component_ref.fov(),
frustum_offset: component_ref.frustum_offset(),
h_offset: component_ref.h_offset(),
near: component_ref.znear(),
size: component_ref.size(),
v_offset: component_ref.v_offset(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCamera {
    
}

fn sync_bevy_owned(query: Query<(&GDCamera, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Camera>().unwrap();
        component_ref.set_cull_mask(component.cull_mask);
component_ref.set_current(component.current);
component_ref.set_zfar(component.far);
component_ref.set_fov(component.fov);
component_ref.set_frustum_offset(component.frustum_offset);
component_ref.set_h_offset(component.h_offset);
component_ref.set_znear(component.near);
component_ref.set_size(component.size);
component_ref.set_v_offset(component.v_offset);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCamera, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Camera>().unwrap();
        component.cull_mask = component_ref.cull_mask();
component.current = component_ref.is_current();
component.far = component_ref.zfar();
component.fov = component_ref.fov();
component.frustum_offset = component_ref.frustum_offset();
component.h_offset = component_ref.h_offset();
component.near = component_ref.znear();
component.size = component_ref.size();
component.v_offset = component_ref.v_offset();
    }
}