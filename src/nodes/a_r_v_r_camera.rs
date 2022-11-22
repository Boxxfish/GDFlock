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

pub struct ARVRCameraPlugin;

impl Plugin for ARVRCameraPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ARVRCamera>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a a_r_v_r_camera.
pub fn is_a_r_v_r_camera(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ARVRCamera>().is_some()
}

/// A bundle for ARVRCameras.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDARVRCameraBundle {
    pub a_r_v_r_camera: GDARVRCamera,
    pub node: GDNode,
pub spatial: GDSpatial,
pub camera: GDCamera,
    pub true_type: TrueNodeType,
}

impl Default for GDARVRCameraBundle {
    fn default() -> Self {
        Self {
            a_r_v_r_camera: Default::default(),
            node: Default::default(),
spatial: Default::default(),
camera: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ARVRCamera".to_string()
            }
        }
    }
}

/// Represents a ARVRCamera.
#[derive(Component)]
pub struct GDARVRCamera {
    
}

impl Default for GDARVRCamera {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDARVRCamera {
    type Parent = GDCamera;
    type GodotClass = ARVRCamera;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ARVRCamera>().unwrap();
        world_commands.insert(entity, GDARVRCamera {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDARVRCamera {
    
}

fn sync_bevy_owned(query: Query<(&GDARVRCamera, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ARVRCamera>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDARVRCamera, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ARVRCamera>().unwrap();
        
    }
}