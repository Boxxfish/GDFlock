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

pub struct MultiMeshInstancePlugin;

impl Plugin for MultiMeshInstancePlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<MultiMeshInstance>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a multi_mesh_instance.
pub fn is_multi_mesh_instance(node: &gdnative::prelude::Node) -> bool {
    node.cast::<MultiMeshInstance>().is_some()
}

/// A bundle for MultiMeshInstances.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDMultiMeshInstanceBundle {
    pub multi_mesh_instance: GDMultiMeshInstance,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub geometry_instance: GDGeometryInstance,
    pub true_type: TrueNodeType,
}

impl Default for GDMultiMeshInstanceBundle {
    fn default() -> Self {
        Self {
            multi_mesh_instance: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
geometry_instance: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "MultiMeshInstance".to_string()
            }
        }
    }
}

/// Represents a MultiMeshInstance.
#[derive(Component)]
pub struct GDMultiMeshInstance {
    pub multimesh: Option<Ref<MultiMesh>>,
}

impl Default for GDMultiMeshInstance {
    fn default() -> Self {
        Self {
            multimesh: Default::default(),
        }
    }
}

impl NodeClass for GDMultiMeshInstance {
    type Parent = GDGeometryInstance;
    type GodotClass = MultiMeshInstance;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<MultiMeshInstance>().unwrap();
        world_commands.insert(entity, GDMultiMeshInstance {
            multimesh: component_ref.multimesh(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDMultiMeshInstance {
    
}

fn sync_bevy_owned(query: Query<(&GDMultiMeshInstance, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<MultiMeshInstance>().unwrap();
        component_ref.set_multimesh(component.multimesh.as_ref().unwrap().clone());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDMultiMeshInstance, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<MultiMeshInstance>().unwrap();
        component.multimesh = component_ref.multimesh();
    }
}