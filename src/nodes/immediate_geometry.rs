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

pub struct ImmediateGeometryPlugin;

impl Plugin for ImmediateGeometryPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ImmediateGeometry>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a immediate_geometry.
pub fn is_immediate_geometry(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ImmediateGeometry>().is_some()
}

/// A bundle for ImmediateGeometrys.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDImmediateGeometryBundle {
    pub immediate_geometry: GDImmediateGeometry,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub geometry_instance: GDGeometryInstance,
    pub true_type: TrueNodeType,
}

impl Default for GDImmediateGeometryBundle {
    fn default() -> Self {
        Self {
            immediate_geometry: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
geometry_instance: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ImmediateGeometry".to_string()
            }
        }
    }
}

/// Represents a ImmediateGeometry.
#[derive(Component)]
pub struct GDImmediateGeometry {
    
}

impl Default for GDImmediateGeometry {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDImmediateGeometry {
    type Parent = GDGeometryInstance;
    type GodotClass = ImmediateGeometry;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ImmediateGeometry>().unwrap();
        world_commands.insert(entity, GDImmediateGeometry {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDImmediateGeometry {
    
}

fn sync_bevy_owned(query: Query<(&GDImmediateGeometry, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ImmediateGeometry>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDImmediateGeometry, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ImmediateGeometry>().unwrap();
        
    }
}