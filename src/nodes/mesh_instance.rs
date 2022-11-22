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

pub struct MeshInstancePlugin;

impl Plugin for MeshInstancePlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<MeshInstance>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a mesh_instance.
pub fn is_mesh_instance(node: &gdnative::prelude::Node) -> bool {
    node.cast::<MeshInstance>().is_some()
}

/// A bundle for MeshInstances.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDMeshInstanceBundle {
    pub mesh_instance: GDMeshInstance,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub geometry_instance: GDGeometryInstance,
    pub true_type: TrueNodeType,
}

impl Default for GDMeshInstanceBundle {
    fn default() -> Self {
        Self {
            mesh_instance: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
geometry_instance: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "MeshInstance".to_string()
            }
        }
    }
}

/// Represents a MeshInstance.
#[derive(Component)]
pub struct GDMeshInstance {
    pub mesh: Option<Ref<Mesh>>,
pub skeleton: NodePath,
pub software_skinning_transform_normals: bool,
}

impl Default for GDMeshInstance {
    fn default() -> Self {
        Self {
            mesh: Default::default(),
skeleton: Default::default(),
software_skinning_transform_normals: Default::default(),
        }
    }
}

impl NodeClass for GDMeshInstance {
    type Parent = GDGeometryInstance;
    type GodotClass = MeshInstance;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<MeshInstance>().unwrap();
        world_commands.insert(entity, GDMeshInstance {
            mesh: component_ref.mesh(),
skeleton: component_ref.skeleton_path(),
software_skinning_transform_normals: component_ref.is_software_skinning_transform_normals_enabled(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDMeshInstance {
    
}

fn sync_bevy_owned(query: Query<(&GDMeshInstance, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<MeshInstance>().unwrap();
        component_ref.set_mesh(component.mesh.as_ref().unwrap().clone());
component_ref.set_skeleton_path(component.skeleton.to_godot_string());
component_ref.set_software_skinning_transform_normals(component.software_skinning_transform_normals);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDMeshInstance, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<MeshInstance>().unwrap();
        component.mesh = component_ref.mesh();
component.skeleton = component_ref.skeleton_path();
component.software_skinning_transform_normals = component_ref.is_software_skinning_transform_normals_enabled();
    }
}