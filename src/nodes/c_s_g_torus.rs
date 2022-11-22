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

pub struct CSGTorusPlugin;

impl Plugin for CSGTorusPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<CSGTorus>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a c_s_g_torus.
pub fn is_c_s_g_torus(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CSGTorus>().is_some()
}

/// A bundle for CSGToruss.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCSGTorusBundle {
    pub c_s_g_torus: GDCSGTorus,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub geometry_instance: GDGeometryInstance,
pub c_s_g_shape: GDCSGShape,
pub c_s_g_primitive: GDCSGPrimitive,
    pub true_type: TrueNodeType,
}

impl Default for GDCSGTorusBundle {
    fn default() -> Self {
        Self {
            c_s_g_torus: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
geometry_instance: Default::default(),
c_s_g_shape: Default::default(),
c_s_g_primitive: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CSGTorus".to_string()
            }
        }
    }
}

/// Represents a CSGTorus.
#[derive(Component)]
pub struct GDCSGTorus {
    pub inner_radius: f64,
pub outer_radius: f64,
pub ring_sides: i64,
pub sides: i64,
pub smooth_faces: bool,
}

impl Default for GDCSGTorus {
    fn default() -> Self {
        Self {
            inner_radius: Default::default(),
outer_radius: Default::default(),
ring_sides: Default::default(),
sides: Default::default(),
smooth_faces: Default::default(),
        }
    }
}

impl NodeClass for GDCSGTorus {
    type Parent = GDCSGPrimitive;
    type GodotClass = CSGTorus;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CSGTorus>().unwrap();
        world_commands.insert(entity, GDCSGTorus {
            inner_radius: component_ref.inner_radius(),
outer_radius: component_ref.outer_radius(),
ring_sides: component_ref.ring_sides(),
sides: component_ref.sides(),
smooth_faces: component_ref.smooth_faces(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCSGTorus {
    
}

fn sync_bevy_owned(query: Query<(&GDCSGTorus, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CSGTorus>().unwrap();
        component_ref.set_inner_radius(component.inner_radius);
component_ref.set_outer_radius(component.outer_radius);
component_ref.set_ring_sides(component.ring_sides);
component_ref.set_sides(component.sides);
component_ref.set_smooth_faces(component.smooth_faces);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCSGTorus, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CSGTorus>().unwrap();
        component.inner_radius = component_ref.inner_radius();
component.outer_radius = component_ref.outer_radius();
component.ring_sides = component_ref.ring_sides();
component.sides = component_ref.sides();
component.smooth_faces = component_ref.smooth_faces();
    }
}