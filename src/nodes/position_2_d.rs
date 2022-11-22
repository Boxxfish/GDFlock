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

pub struct Position2DPlugin;

impl Plugin for Position2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Position2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a position_2_d.
pub fn is_position_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Position2D>().is_some()
}

/// A bundle for Position2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDPosition2DBundle {
    pub position_2_d: GDPosition2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDPosition2DBundle {
    fn default() -> Self {
        Self {
            position_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Position2D".to_string()
            }
        }
    }
}

/// Represents a Position2D.
#[derive(Component)]
pub struct GDPosition2D {
    
}

impl Default for GDPosition2D {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDPosition2D {
    type Parent = GDNode2D;
    type GodotClass = Position2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Position2D>().unwrap();
        world_commands.insert(entity, GDPosition2D {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDPosition2D {
    
}

fn sync_bevy_owned(query: Query<(&GDPosition2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Position2D>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDPosition2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Position2D>().unwrap();
        
    }
}