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

pub struct CSGCombinerPlugin;

impl Plugin for CSGCombinerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<CSGCombiner>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a c_s_g_combiner.
pub fn is_c_s_g_combiner(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CSGCombiner>().is_some()
}

/// A bundle for CSGCombiners.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCSGCombinerBundle {
    pub c_s_g_combiner: GDCSGCombiner,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub geometry_instance: GDGeometryInstance,
pub c_s_g_shape: GDCSGShape,
    pub true_type: TrueNodeType,
}

impl Default for GDCSGCombinerBundle {
    fn default() -> Self {
        Self {
            c_s_g_combiner: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
geometry_instance: Default::default(),
c_s_g_shape: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CSGCombiner".to_string()
            }
        }
    }
}

/// Represents a CSGCombiner.
#[derive(Component)]
pub struct GDCSGCombiner {
    
}

impl Default for GDCSGCombiner {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDCSGCombiner {
    type Parent = GDCSGShape;
    type GodotClass = CSGCombiner;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CSGCombiner>().unwrap();
        world_commands.insert(entity, GDCSGCombiner {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCSGCombiner {
    
}

fn sync_bevy_owned(query: Query<(&GDCSGCombiner, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CSGCombiner>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCSGCombiner, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CSGCombiner>().unwrap();
        
    }
}