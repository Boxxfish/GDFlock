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

pub struct CSGPolygonPlugin;

impl Plugin for CSGPolygonPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<CSGPolygon>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a c_s_g_polygon.
pub fn is_c_s_g_polygon(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CSGPolygon>().is_some()
}

/// A bundle for CSGPolygons.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCSGPolygonBundle {
    pub c_s_g_polygon: GDCSGPolygon,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub geometry_instance: GDGeometryInstance,
pub c_s_g_shape: GDCSGShape,
pub c_s_g_primitive: GDCSGPrimitive,
    pub true_type: TrueNodeType,
}

impl Default for GDCSGPolygonBundle {
    fn default() -> Self {
        Self {
            c_s_g_polygon: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
geometry_instance: Default::default(),
c_s_g_shape: Default::default(),
c_s_g_primitive: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CSGPolygon".to_string()
            }
        }
    }
}

/// Represents a CSGPolygon.
#[derive(Component)]
pub struct GDCSGPolygon {
    pub depth: f64,
pub path_continuous_u: bool,
pub path_interval: f64,
pub path_joined: bool,
pub path_local: bool,
pub path_node: NodePath,
pub path_simplify_angle: f64,
pub path_u_distance: f64,
pub polygon: Vec<Vector2>,
pub smooth_faces: bool,
pub spin_degrees: f64,
pub spin_sides: i64,
}

impl Default for GDCSGPolygon {
    fn default() -> Self {
        Self {
            depth: Default::default(),
path_continuous_u: Default::default(),
path_interval: Default::default(),
path_joined: Default::default(),
path_local: Default::default(),
path_node: Default::default(),
path_simplify_angle: Default::default(),
path_u_distance: Default::default(),
polygon: Default::default(),
smooth_faces: Default::default(),
spin_degrees: Default::default(),
spin_sides: Default::default(),
        }
    }
}

impl NodeClass for GDCSGPolygon {
    type Parent = GDCSGPrimitive;
    type GodotClass = CSGPolygon;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CSGPolygon>().unwrap();
        world_commands.insert(entity, GDCSGPolygon {
            depth: component_ref.depth(),
path_continuous_u: component_ref.is_path_continuous_u(),
path_interval: component_ref.path_interval(),
path_joined: component_ref.is_path_joined(),
path_local: component_ref.is_path_local(),
path_node: component_ref.path_node(),
path_simplify_angle: component_ref.path_simplify_angle(),
path_u_distance: component_ref.path_u_distance(),
polygon: component_ref.polygon().to_vec(),
smooth_faces: component_ref.smooth_faces(),
spin_degrees: component_ref.spin_degrees(),
spin_sides: component_ref.spin_sides(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCSGPolygon {
    
}

fn sync_bevy_owned(query: Query<(&GDCSGPolygon, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CSGPolygon>().unwrap();
        component_ref.set_depth(component.depth);
component_ref.set_path_continuous_u(component.path_continuous_u);
component_ref.set_path_interval(component.path_interval);
component_ref.set_path_joined(component.path_joined);
component_ref.set_path_local(component.path_local);
component_ref.set_path_node(component.path_node.to_godot_string());
component_ref.set_path_simplify_angle(component.path_simplify_angle);
component_ref.set_path_u_distance(component.path_u_distance);
component_ref.set_polygon(Vector2Array::from_vec(component.polygon.clone()));
component_ref.set_smooth_faces(component.smooth_faces);
component_ref.set_spin_degrees(component.spin_degrees);
component_ref.set_spin_sides(component.spin_sides);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCSGPolygon, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CSGPolygon>().unwrap();
        component.depth = component_ref.depth();
component.path_continuous_u = component_ref.is_path_continuous_u();
component.path_interval = component_ref.path_interval();
component.path_joined = component_ref.is_path_joined();
component.path_local = component_ref.is_path_local();
component.path_node = component_ref.path_node();
component.path_simplify_angle = component_ref.path_simplify_angle();
component.path_u_distance = component_ref.path_u_distance();
component.polygon = component_ref.polygon().to_vec();
component.smooth_faces = component_ref.smooth_faces();
component.spin_degrees = component_ref.spin_degrees();
component.spin_sides = component_ref.spin_sides();
    }
}