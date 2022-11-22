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

pub struct PathPlugin;

impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Path>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a path.
pub fn is_path(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Path>().is_some()
}

/// A bundle for Paths.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDPathBundle {
    pub path: GDPath,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDPathBundle {
    fn default() -> Self {
        Self {
            path: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Path".to_string()
            }
        }
    }
}

/// Represents a Path.
#[derive(Component)]
pub struct GDPath {
    
}

impl Default for GDPath {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDPath {
    type Parent = GDSpatial;
    type GodotClass = Path;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Path>().unwrap();
        world_commands.insert(entity, GDPath {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDPath {
    
}

fn sync_bevy_owned(query: Query<(&GDPath, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Path>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDPath, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Path>().unwrap();
        
    }
}