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

pub struct WorldEnvironmentPlugin;

impl Plugin for WorldEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<WorldEnvironment>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a world_environment.
pub fn is_world_environment(node: &gdnative::prelude::Node) -> bool {
    node.cast::<WorldEnvironment>().is_some()
}

/// A bundle for WorldEnvironments.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDWorldEnvironmentBundle {
    pub world_environment: GDWorldEnvironment,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDWorldEnvironmentBundle {
    fn default() -> Self {
        Self {
            world_environment: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "WorldEnvironment".to_string()
            }
        }
    }
}

/// Represents a WorldEnvironment.
#[derive(Component)]
pub struct GDWorldEnvironment {
    
}

impl Default for GDWorldEnvironment {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDWorldEnvironment {
    type Parent = GDNode;
    type GodotClass = WorldEnvironment;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<WorldEnvironment>().unwrap();
        world_commands.insert(entity, GDWorldEnvironment {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDWorldEnvironment {
    
}

fn sync_bevy_owned(query: Query<(&GDWorldEnvironment, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<WorldEnvironment>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDWorldEnvironment, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<WorldEnvironment>().unwrap();
        
    }
}