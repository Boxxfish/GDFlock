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

pub struct KinematicBodyPlugin;

impl Plugin for KinematicBodyPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<KinematicBody>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a kinematic_body.
pub fn is_kinematic_body(node: &gdnative::prelude::Node) -> bool {
    node.cast::<KinematicBody>().is_some()
}

/// A bundle for KinematicBodys.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDKinematicBodyBundle {
    pub kinematic_body: GDKinematicBody,
    pub node: GDNode,
pub spatial: GDSpatial,
pub collision_object: GDCollisionObject,
pub physics_body: GDPhysicsBody,
    pub true_type: TrueNodeType,
}

impl Default for GDKinematicBodyBundle {
    fn default() -> Self {
        Self {
            kinematic_body: Default::default(),
            node: Default::default(),
spatial: Default::default(),
collision_object: Default::default(),
physics_body: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "KinematicBody".to_string()
            }
        }
    }
}

/// Represents a KinematicBody.
#[derive(Component)]
pub struct GDKinematicBody {
    
}

impl Default for GDKinematicBody {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDKinematicBody {
    type Parent = GDPhysicsBody;
    type GodotClass = KinematicBody;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<KinematicBody>().unwrap();
        world_commands.insert(entity, GDKinematicBody {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDKinematicBody {
    
}

fn sync_bevy_owned(query: Query<(&GDKinematicBody, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<KinematicBody>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDKinematicBody, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<KinematicBody>().unwrap();
        
    }
}