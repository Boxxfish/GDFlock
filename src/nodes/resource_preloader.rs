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

pub struct ResourcePreloaderPlugin;

impl Plugin for ResourcePreloaderPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ResourcePreloader>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a resource_preloader.
pub fn is_resource_preloader(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ResourcePreloader>().is_some()
}

/// A bundle for ResourcePreloaders.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDResourcePreloaderBundle {
    pub resource_preloader: GDResourcePreloader,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDResourcePreloaderBundle {
    fn default() -> Self {
        Self {
            resource_preloader: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ResourcePreloader".to_string()
            }
        }
    }
}

/// Represents a ResourcePreloader.
#[derive(Component)]
pub struct GDResourcePreloader {
    
}

impl Default for GDResourcePreloader {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDResourcePreloader {
    type Parent = GDNode;
    type GodotClass = ResourcePreloader;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ResourcePreloader>().unwrap();
        world_commands.insert(entity, GDResourcePreloader {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDResourcePreloader {
    
}

fn sync_bevy_owned(query: Query<(&GDResourcePreloader, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ResourcePreloader>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDResourcePreloader, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ResourcePreloader>().unwrap();
        
    }
}