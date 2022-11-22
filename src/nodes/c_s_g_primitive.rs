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

pub struct CSGPrimitivePlugin;

impl Plugin for CSGPrimitivePlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a c_s_g_primitive.
pub fn is_c_s_g_primitive(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CSGPrimitive>().is_some()
}

/// A bundle for CSGPrimitives.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCSGPrimitiveBundle {
    pub c_s_g_primitive: GDCSGPrimitive,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub geometry_instance: GDGeometryInstance,
pub c_s_g_shape: GDCSGShape,
    pub true_type: TrueNodeType,
}

impl Default for GDCSGPrimitiveBundle {
    fn default() -> Self {
        Self {
            c_s_g_primitive: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
geometry_instance: Default::default(),
c_s_g_shape: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CSGPrimitive".to_string()
            }
        }
    }
}

/// Represents a CSGPrimitive.
#[derive(Component)]
pub struct GDCSGPrimitive {
    pub invert_faces: bool,
}

impl Default for GDCSGPrimitive {
    fn default() -> Self {
        Self {
            invert_faces: Default::default(),
        }
    }
}

impl NodeClass for GDCSGPrimitive {
    type Parent = GDCSGShape;
    type GodotClass = CSGPrimitive;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CSGPrimitive>().unwrap();
        world_commands.insert(entity, GDCSGPrimitive {
            invert_faces: component_ref.is_inverting_faces(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCSGPrimitive {
    
}

fn sync_bevy_owned(query: Query<(&GDCSGPrimitive, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CSGPrimitive>().unwrap();
        component_ref.set_invert_faces(component.invert_faces);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCSGPrimitive, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CSGPrimitive>().unwrap();
        component.invert_faces = component_ref.is_inverting_faces();
    }
}