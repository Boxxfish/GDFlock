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

pub struct CSGSpherePlugin;

impl Plugin for CSGSpherePlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<CSGSphere>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a c_s_g_sphere.
pub fn is_c_s_g_sphere(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CSGSphere>().is_some()
}

/// A bundle for CSGSpheres.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCSGSphereBundle {
    pub c_s_g_sphere: GDCSGSphere,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub geometry_instance: GDGeometryInstance,
pub c_s_g_shape: GDCSGShape,
pub c_s_g_primitive: GDCSGPrimitive,
    pub true_type: TrueNodeType,
}

impl Default for GDCSGSphereBundle {
    fn default() -> Self {
        Self {
            c_s_g_sphere: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
geometry_instance: Default::default(),
c_s_g_shape: Default::default(),
c_s_g_primitive: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CSGSphere".to_string()
            }
        }
    }
}

/// Represents a CSGSphere.
#[derive(Component)]
pub struct GDCSGSphere {
    pub radial_segments: i64,
pub radius: f64,
pub rings: i64,
pub smooth_faces: bool,
}

impl Default for GDCSGSphere {
    fn default() -> Self {
        Self {
            radial_segments: Default::default(),
radius: Default::default(),
rings: Default::default(),
smooth_faces: Default::default(),
        }
    }
}

impl NodeClass for GDCSGSphere {
    type Parent = GDCSGPrimitive;
    type GodotClass = CSGSphere;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CSGSphere>().unwrap();
        world_commands.insert(entity, GDCSGSphere {
            radial_segments: component_ref.radial_segments(),
radius: component_ref.radius(),
rings: component_ref.rings(),
smooth_faces: component_ref.smooth_faces(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCSGSphere {
    
}

fn sync_bevy_owned(query: Query<(&GDCSGSphere, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CSGSphere>().unwrap();
        component_ref.set_radial_segments(component.radial_segments);
component_ref.set_radius(component.radius);
component_ref.set_rings(component.rings);
component_ref.set_smooth_faces(component.smooth_faces);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCSGSphere, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CSGSphere>().unwrap();
        component.radial_segments = component_ref.radial_segments();
component.radius = component_ref.radius();
component.rings = component_ref.rings();
component.smooth_faces = component_ref.smooth_faces();
    }
}