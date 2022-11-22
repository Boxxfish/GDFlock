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

pub struct PhysicsBodyPlugin;

impl Plugin for PhysicsBodyPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a physics_body.
pub fn is_physics_body(node: &gdnative::prelude::Node) -> bool {
    node.cast::<PhysicsBody>().is_some()
}

/// A bundle for PhysicsBodys.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDPhysicsBodyBundle {
    pub physics_body: GDPhysicsBody,
    pub node: GDNode,
pub spatial: GDSpatial,
pub collision_object: GDCollisionObject,
    pub true_type: TrueNodeType,
}

impl Default for GDPhysicsBodyBundle {
    fn default() -> Self {
        Self {
            physics_body: Default::default(),
            node: Default::default(),
spatial: Default::default(),
collision_object: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "PhysicsBody".to_string()
            }
        }
    }
}

/// Represents a PhysicsBody.
#[derive(Component)]
pub struct GDPhysicsBody {
    
}

impl Default for GDPhysicsBody {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDPhysicsBody {
    type Parent = GDCollisionObject;
    type GodotClass = PhysicsBody;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<PhysicsBody>().unwrap();
        world_commands.insert(entity, GDPhysicsBody {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDPhysicsBody {
    
}

fn sync_bevy_owned(query: Query<(&GDPhysicsBody, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PhysicsBody>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDPhysicsBody, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PhysicsBody>().unwrap();
        
    }
}