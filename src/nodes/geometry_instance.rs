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

pub struct GeometryInstancePlugin;

impl Plugin for GeometryInstancePlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a geometry_instance.
pub fn is_geometry_instance(node: &gdnative::prelude::Node) -> bool {
    node.cast::<GeometryInstance>().is_some()
}

/// A bundle for GeometryInstances.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDGeometryInstanceBundle {
    pub geometry_instance: GDGeometryInstance,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
    pub true_type: TrueNodeType,
}

impl Default for GDGeometryInstanceBundle {
    fn default() -> Self {
        Self {
            geometry_instance: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "GeometryInstance".to_string()
            }
        }
    }
}

/// Represents a GeometryInstance.
#[derive(Component)]
pub struct GDGeometryInstance {
    pub extra_cull_margin: f64,
pub generate_lightmap: bool,
pub lod_max_distance: f64,
pub lod_max_hysteresis: f64,
pub lod_min_distance: f64,
pub lod_min_hysteresis: f64,
}

impl Default for GDGeometryInstance {
    fn default() -> Self {
        Self {
            extra_cull_margin: Default::default(),
generate_lightmap: Default::default(),
lod_max_distance: Default::default(),
lod_max_hysteresis: Default::default(),
lod_min_distance: Default::default(),
lod_min_hysteresis: Default::default(),
        }
    }
}

impl NodeClass for GDGeometryInstance {
    type Parent = GDVisualInstance;
    type GodotClass = GeometryInstance;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<GeometryInstance>().unwrap();
        world_commands.insert(entity, GDGeometryInstance {
            extra_cull_margin: component_ref.extra_cull_margin(),
generate_lightmap: component_ref.generate_lightmap(),
lod_max_distance: component_ref.lod_max_distance(),
lod_max_hysteresis: component_ref.lod_max_hysteresis(),
lod_min_distance: component_ref.lod_min_distance(),
lod_min_hysteresis: component_ref.lod_min_hysteresis(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDGeometryInstance {
    
}

fn sync_bevy_owned(query: Query<(&GDGeometryInstance, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<GeometryInstance>().unwrap();
        component_ref.set_extra_cull_margin(component.extra_cull_margin);
component_ref.set_generate_lightmap(component.generate_lightmap);
component_ref.set_lod_max_distance(component.lod_max_distance);
component_ref.set_lod_max_hysteresis(component.lod_max_hysteresis);
component_ref.set_lod_min_distance(component.lod_min_distance);
component_ref.set_lod_min_hysteresis(component.lod_min_hysteresis);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDGeometryInstance, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<GeometryInstance>().unwrap();
        component.extra_cull_margin = component_ref.extra_cull_margin();
component.generate_lightmap = component_ref.generate_lightmap();
component.lod_max_distance = component_ref.lod_max_distance();
component.lod_max_hysteresis = component_ref.lod_max_hysteresis();
component.lod_min_distance = component_ref.lod_min_distance();
component.lod_min_hysteresis = component_ref.lod_min_hysteresis();
    }
}