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

pub struct Position3DPlugin;

impl Plugin for Position3DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Position3D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a position_3_d.
pub fn is_position_3_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Position3D>().is_some()
}

/// A bundle for Position3Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDPosition3DBundle {
    pub position_3_d: GDPosition3D,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDPosition3DBundle {
    fn default() -> Self {
        Self {
            position_3_d: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Position3D".to_string()
            }
        }
    }
}

/// Represents a Position3D.
#[derive(Component)]
pub struct GDPosition3D {
    
}

impl Default for GDPosition3D {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDPosition3D {
    type Parent = GDSpatial;
    type GodotClass = Position3D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Position3D>().unwrap();
        world_commands.insert(entity, GDPosition3D {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDPosition3D {
    
}

fn sync_bevy_owned(query: Query<(&GDPosition3D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Position3D>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDPosition3D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Position3D>().unwrap();
        
    }
}