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

pub struct CSGCylinderPlugin;

impl Plugin for CSGCylinderPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<CSGCylinder>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a c_s_g_cylinder.
pub fn is_c_s_g_cylinder(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CSGCylinder>().is_some()
}

/// A bundle for CSGCylinders.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCSGCylinderBundle {
    pub c_s_g_cylinder: GDCSGCylinder,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub geometry_instance: GDGeometryInstance,
pub c_s_g_shape: GDCSGShape,
pub c_s_g_primitive: GDCSGPrimitive,
    pub true_type: TrueNodeType,
}

impl Default for GDCSGCylinderBundle {
    fn default() -> Self {
        Self {
            c_s_g_cylinder: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
geometry_instance: Default::default(),
c_s_g_shape: Default::default(),
c_s_g_primitive: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CSGCylinder".to_string()
            }
        }
    }
}

/// Represents a CSGCylinder.
#[derive(Component)]
pub struct GDCSGCylinder {
    pub cone: bool,
pub height: f64,
pub radius: f64,
pub sides: i64,
pub smooth_faces: bool,
}

impl Default for GDCSGCylinder {
    fn default() -> Self {
        Self {
            cone: Default::default(),
height: Default::default(),
radius: Default::default(),
sides: Default::default(),
smooth_faces: Default::default(),
        }
    }
}

impl NodeClass for GDCSGCylinder {
    type Parent = GDCSGPrimitive;
    type GodotClass = CSGCylinder;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CSGCylinder>().unwrap();
        world_commands.insert(entity, GDCSGCylinder {
            cone: component_ref.is_cone(),
height: component_ref.height(),
radius: component_ref.radius(),
sides: component_ref.sides(),
smooth_faces: component_ref.smooth_faces(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCSGCylinder {
    
}

fn sync_bevy_owned(query: Query<(&GDCSGCylinder, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CSGCylinder>().unwrap();
        component_ref.set_cone(component.cone);
component_ref.set_height(component.height);
component_ref.set_radius(component.radius);
component_ref.set_sides(component.sides);
component_ref.set_smooth_faces(component.smooth_faces);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCSGCylinder, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CSGCylinder>().unwrap();
        component.cone = component_ref.is_cone();
component.height = component_ref.height();
component.radius = component_ref.radius();
component.sides = component_ref.sides();
component.smooth_faces = component_ref.smooth_faces();
    }
}