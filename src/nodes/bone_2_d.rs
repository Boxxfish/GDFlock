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

pub struct Bone2DPlugin;

impl Plugin for Bone2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Bone2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a bone_2_d.
pub fn is_bone_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Bone2D>().is_some()
}

/// A bundle for Bone2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDBone2DBundle {
    pub bone_2_d: GDBone2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDBone2DBundle {
    fn default() -> Self {
        Self {
            bone_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Bone2D".to_string()
            }
        }
    }
}

/// Represents a Bone2D.
#[derive(Component)]
pub struct GDBone2D {
    pub default_length: f64,
pub rest: Transform2D,
}

impl Default for GDBone2D {
    fn default() -> Self {
        Self {
            default_length: Default::default(),
rest: Transform2D::IDENTITY,
        }
    }
}

impl NodeClass for GDBone2D {
    type Parent = GDNode2D;
    type GodotClass = Bone2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Bone2D>().unwrap();
        world_commands.insert(entity, GDBone2D {
            default_length: component_ref.default_length(),
rest: component_ref.rest(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDBone2D {
    
}

fn sync_bevy_owned(query: Query<(&GDBone2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Bone2D>().unwrap();
        component_ref.set_default_length(component.default_length);
component_ref.set_rest(component.rest);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDBone2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Bone2D>().unwrap();
        component.default_length = component_ref.default_length();
component.rest = component_ref.rest();
    }
}