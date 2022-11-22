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

pub struct CollisionShapePlugin;

impl Plugin for CollisionShapePlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<CollisionShape>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a collision_shape.
pub fn is_collision_shape(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CollisionShape>().is_some()
}

/// A bundle for CollisionShapes.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCollisionShapeBundle {
    pub collision_shape: GDCollisionShape,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDCollisionShapeBundle {
    fn default() -> Self {
        Self {
            collision_shape: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CollisionShape".to_string()
            }
        }
    }
}

/// Represents a CollisionShape.
#[derive(Component)]
pub struct GDCollisionShape {
    pub disabled: bool,
}

impl Default for GDCollisionShape {
    fn default() -> Self {
        Self {
            disabled: Default::default(),
        }
    }
}

impl NodeClass for GDCollisionShape {
    type Parent = GDSpatial;
    type GodotClass = CollisionShape;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CollisionShape>().unwrap();
        world_commands.insert(entity, GDCollisionShape {
            disabled: component_ref.is_disabled(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCollisionShape {
    
}

fn sync_bevy_owned(query: Query<(&GDCollisionShape, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CollisionShape>().unwrap();
        component_ref.set_disabled(component.disabled);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCollisionShape, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CollisionShape>().unwrap();
        component.disabled = component_ref.is_disabled();
    }
}