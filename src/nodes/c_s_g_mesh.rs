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

pub struct CSGMeshPlugin;

impl Plugin for CSGMeshPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<CSGMesh>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a c_s_g_mesh.
pub fn is_c_s_g_mesh(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CSGMesh>().is_some()
}

/// A bundle for CSGMeshs.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCSGMeshBundle {
    pub c_s_g_mesh: GDCSGMesh,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub geometry_instance: GDGeometryInstance,
pub c_s_g_shape: GDCSGShape,
pub c_s_g_primitive: GDCSGPrimitive,
    pub true_type: TrueNodeType,
}

impl Default for GDCSGMeshBundle {
    fn default() -> Self {
        Self {
            c_s_g_mesh: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
geometry_instance: Default::default(),
c_s_g_shape: Default::default(),
c_s_g_primitive: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CSGMesh".to_string()
            }
        }
    }
}

/// Represents a CSGMesh.
#[derive(Component)]
pub struct GDCSGMesh {
    pub mesh: Option<Ref<Mesh>>,
}

impl Default for GDCSGMesh {
    fn default() -> Self {
        Self {
            mesh: Default::default(),
        }
    }
}

impl NodeClass for GDCSGMesh {
    type Parent = GDCSGPrimitive;
    type GodotClass = CSGMesh;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CSGMesh>().unwrap();
        world_commands.insert(entity, GDCSGMesh {
            mesh: component_ref.mesh(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCSGMesh {
    
}

fn sync_bevy_owned(query: Query<(&GDCSGMesh, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CSGMesh>().unwrap();
        component_ref.set_mesh(component.mesh.as_ref().unwrap().clone());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCSGMesh, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CSGMesh>().unwrap();
        component.mesh = component_ref.mesh();
    }
}