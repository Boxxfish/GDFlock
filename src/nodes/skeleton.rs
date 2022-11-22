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

pub struct SkeletonPlugin;

impl Plugin for SkeletonPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Skeleton>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a skeleton.
pub fn is_skeleton(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Skeleton>().is_some()
}

/// A bundle for Skeletons.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDSkeletonBundle {
    pub skeleton: GDSkeleton,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDSkeletonBundle {
    fn default() -> Self {
        Self {
            skeleton: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Skeleton".to_string()
            }
        }
    }
}

/// Represents a Skeleton.
#[derive(Component)]
pub struct GDSkeleton {
    
}

impl Default for GDSkeleton {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDSkeleton {
    type Parent = GDSpatial;
    type GodotClass = Skeleton;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Skeleton>().unwrap();
        world_commands.insert(entity, GDSkeleton {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDSkeleton {
    
}

fn sync_bevy_owned(query: Query<(&GDSkeleton, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Skeleton>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDSkeleton, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Skeleton>().unwrap();
        
    }
}