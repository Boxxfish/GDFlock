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

pub struct MultiMeshInstance2DPlugin;

impl Plugin for MultiMeshInstance2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<MultiMeshInstance2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a multi_mesh_instance_2_d.
pub fn is_multi_mesh_instance_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<MultiMeshInstance2D>().is_some()
}

/// A bundle for MultiMeshInstance2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDMultiMeshInstance2DBundle {
    pub multi_mesh_instance_2_d: GDMultiMeshInstance2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDMultiMeshInstance2DBundle {
    fn default() -> Self {
        Self {
            multi_mesh_instance_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "MultiMeshInstance2D".to_string()
            }
        }
    }
}

/// Represents a MultiMeshInstance2D.
#[derive(Component)]
pub struct GDMultiMeshInstance2D {
    pub multimesh: Option<Ref<MultiMesh>>,
pub normal_map: Option<Ref<Texture>>,
pub texture: Option<Ref<Texture>>,
}

impl Default for GDMultiMeshInstance2D {
    fn default() -> Self {
        Self {
            multimesh: Default::default(),
normal_map: Default::default(),
texture: Default::default(),
        }
    }
}

impl NodeClass for GDMultiMeshInstance2D {
    type Parent = GDNode2D;
    type GodotClass = MultiMeshInstance2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<MultiMeshInstance2D>().unwrap();
        world_commands.insert(entity, GDMultiMeshInstance2D {
            multimesh: component_ref.multimesh(),
normal_map: component_ref.normal_map(),
texture: component_ref.texture(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDMultiMeshInstance2D {
    
}

fn sync_bevy_owned(query: Query<(&GDMultiMeshInstance2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<MultiMeshInstance2D>().unwrap();
        component_ref.set_multimesh(component.multimesh.as_ref().unwrap().clone());
component_ref.set_normal_map(component.normal_map.as_ref().unwrap().clone());
component_ref.set_texture(component.texture.as_ref().unwrap().clone());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDMultiMeshInstance2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<MultiMeshInstance2D>().unwrap();
        component.multimesh = component_ref.multimesh();
component.normal_map = component_ref.normal_map();
component.texture = component_ref.texture();
    }
}