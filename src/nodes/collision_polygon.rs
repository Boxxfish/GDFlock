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

pub struct CollisionPolygonPlugin;

impl Plugin for CollisionPolygonPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<CollisionPolygon>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a collision_polygon.
pub fn is_collision_polygon(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CollisionPolygon>().is_some()
}

/// A bundle for CollisionPolygons.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCollisionPolygonBundle {
    pub collision_polygon: GDCollisionPolygon,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDCollisionPolygonBundle {
    fn default() -> Self {
        Self {
            collision_polygon: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CollisionPolygon".to_string()
            }
        }
    }
}

/// Represents a CollisionPolygon.
#[derive(Component)]
pub struct GDCollisionPolygon {
    pub depth: f64,
pub disabled: bool,
pub margin: f64,
pub polygon: Vec<Vector2>,
}

impl Default for GDCollisionPolygon {
    fn default() -> Self {
        Self {
            depth: Default::default(),
disabled: Default::default(),
margin: Default::default(),
polygon: Default::default(),
        }
    }
}

impl NodeClass for GDCollisionPolygon {
    type Parent = GDSpatial;
    type GodotClass = CollisionPolygon;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CollisionPolygon>().unwrap();
        world_commands.insert(entity, GDCollisionPolygon {
            depth: component_ref.depth(),
disabled: component_ref.is_disabled(),
margin: component_ref.margin(),
polygon: component_ref.polygon().to_vec(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCollisionPolygon {
    
}

fn sync_bevy_owned(query: Query<(&GDCollisionPolygon, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CollisionPolygon>().unwrap();
        component_ref.set_depth(component.depth);
component_ref.set_disabled(component.disabled);
component_ref.set_margin(component.margin);
component_ref.set_polygon(Vector2Array::from_vec(component.polygon.clone()));
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCollisionPolygon, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CollisionPolygon>().unwrap();
        component.depth = component_ref.depth();
component.disabled = component_ref.is_disabled();
component.margin = component_ref.margin();
component.polygon = component_ref.polygon().to_vec();
    }
}