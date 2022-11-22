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

pub struct DirectionalLightPlugin;

impl Plugin for DirectionalLightPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<DirectionalLight>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a directional_light.
pub fn is_directional_light(node: &gdnative::prelude::Node) -> bool {
    node.cast::<DirectionalLight>().is_some()
}

/// A bundle for DirectionalLights.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDDirectionalLightBundle {
    pub directional_light: GDDirectionalLight,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub light: GDLight,
    pub true_type: TrueNodeType,
}

impl Default for GDDirectionalLightBundle {
    fn default() -> Self {
        Self {
            directional_light: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
light: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "DirectionalLight".to_string()
            }
        }
    }
}

/// Represents a DirectionalLight.
#[derive(Component)]
pub struct GDDirectionalLight {
    pub directional_shadow_blend_splits: bool,
}

impl Default for GDDirectionalLight {
    fn default() -> Self {
        Self {
            directional_shadow_blend_splits: Default::default(),
        }
    }
}

impl NodeClass for GDDirectionalLight {
    type Parent = GDLight;
    type GodotClass = DirectionalLight;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<DirectionalLight>().unwrap();
        world_commands.insert(entity, GDDirectionalLight {
            directional_shadow_blend_splits: component_ref.is_blend_splits_enabled(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDDirectionalLight {
    
}

fn sync_bevy_owned(query: Query<(&GDDirectionalLight, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<DirectionalLight>().unwrap();
        component_ref.set_blend_splits(component.directional_shadow_blend_splits);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDDirectionalLight, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<DirectionalLight>().unwrap();
        component.directional_shadow_blend_splits = component_ref.is_blend_splits_enabled();
    }
}