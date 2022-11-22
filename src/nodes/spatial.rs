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

pub struct SpatialPlugin;

impl Plugin for SpatialPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Spatial>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a spatial.
pub fn is_spatial(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Spatial>().is_some()
}

/// A bundle for Spatials.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDSpatialBundle {
    pub spatial: GDSpatial,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDSpatialBundle {
    fn default() -> Self {
        Self {
            spatial: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Spatial".to_string()
            }
        }
    }
}

/// Represents a Spatial.
#[derive(Component)]
pub struct GDSpatial {
    pub global_rotation: Vector3,
pub global_transform: Transform,
pub global_translation: Vector3,
pub rotation: Vector3,
pub rotation_degrees: Vector3,
pub scale: Vector3,
pub transform: Transform,
pub translation: Vector3,
pub visible: bool,
}

impl Default for GDSpatial {
    fn default() -> Self {
        Self {
            global_rotation: Default::default(),
global_transform: Transform::IDENTITY,
global_translation: Default::default(),
rotation: Default::default(),
rotation_degrees: Default::default(),
scale: Default::default(),
transform: Transform::IDENTITY,
translation: Default::default(),
visible: Default::default(),
        }
    }
}

impl NodeClass for GDSpatial {
    type Parent = GDNode;
    type GodotClass = Spatial;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Spatial>().unwrap();
        world_commands.insert(entity, GDSpatial {
            global_rotation: component_ref.global_rotation(),
global_transform: component_ref.global_transform(),
global_translation: component_ref.global_translation(),
rotation: component_ref.rotation(),
rotation_degrees: component_ref.rotation_degrees(),
scale: component_ref.scale(),
transform: component_ref.transform(),
translation: component_ref.translation(),
visible: component_ref.is_visible(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDSpatial {
    
}

fn sync_bevy_owned(query: Query<(&GDSpatial, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Spatial>().unwrap();
        component_ref.set_global_rotation(component.global_rotation);
component_ref.set_global_transform(component.global_transform);
component_ref.set_global_translation(component.global_translation);
component_ref.set_rotation(component.rotation);
component_ref.set_rotation_degrees(component.rotation_degrees);
component_ref.set_scale(component.scale);
component_ref.set_transform(component.transform);
component_ref.set_translation(component.translation);
component_ref.set_visible(component.visible);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDSpatial, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Spatial>().unwrap();
        component.global_rotation = component_ref.global_rotation();
component.global_transform = component_ref.global_transform();
component.global_translation = component_ref.global_translation();
component.rotation = component_ref.rotation();
component.rotation_degrees = component_ref.rotation_degrees();
component.scale = component_ref.scale();
component.transform = component_ref.transform();
component.translation = component_ref.translation();
component.visible = component_ref.is_visible();
    }
}