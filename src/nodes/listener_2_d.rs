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

pub struct Listener2DPlugin;

impl Plugin for Listener2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Listener2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a listener_2_d.
pub fn is_listener_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Listener2D>().is_some()
}

/// A bundle for Listener2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDListener2DBundle {
    pub listener_2_d: GDListener2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDListener2DBundle {
    fn default() -> Self {
        Self {
            listener_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Listener2D".to_string()
            }
        }
    }
}

/// Represents a Listener2D.
#[derive(Component)]
pub struct GDListener2D {
    
}

impl Default for GDListener2D {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDListener2D {
    type Parent = GDNode2D;
    type GodotClass = Listener2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Listener2D>().unwrap();
        world_commands.insert(entity, GDListener2D {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDListener2D {
    
}

fn sync_bevy_owned(query: Query<(&GDListener2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Listener2D>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDListener2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Listener2D>().unwrap();
        
    }
}