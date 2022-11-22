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

pub struct Path2DPlugin;

impl Plugin for Path2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Path2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a path_2_d.
pub fn is_path_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Path2D>().is_some()
}

/// A bundle for Path2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDPath2DBundle {
    pub path_2_d: GDPath2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDPath2DBundle {
    fn default() -> Self {
        Self {
            path_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Path2D".to_string()
            }
        }
    }
}

/// Represents a Path2D.
#[derive(Component)]
pub struct GDPath2D {
    
}

impl Default for GDPath2D {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDPath2D {
    type Parent = GDNode2D;
    type GodotClass = Path2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Path2D>().unwrap();
        world_commands.insert(entity, GDPath2D {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDPath2D {
    
}

fn sync_bevy_owned(query: Query<(&GDPath2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Path2D>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDPath2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Path2D>().unwrap();
        
    }
}