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

pub struct ARVRAnchorPlugin;

impl Plugin for ARVRAnchorPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ARVRAnchor>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a a_r_v_r_anchor.
pub fn is_a_r_v_r_anchor(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ARVRAnchor>().is_some()
}

/// A bundle for ARVRAnchors.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDARVRAnchorBundle {
    pub a_r_v_r_anchor: GDARVRAnchor,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDARVRAnchorBundle {
    fn default() -> Self {
        Self {
            a_r_v_r_anchor: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ARVRAnchor".to_string()
            }
        }
    }
}

/// Represents a ARVRAnchor.
#[derive(Component)]
pub struct GDARVRAnchor {
    pub anchor_id: i64,
}

impl Default for GDARVRAnchor {
    fn default() -> Self {
        Self {
            anchor_id: Default::default(),
        }
    }
}

impl NodeClass for GDARVRAnchor {
    type Parent = GDSpatial;
    type GodotClass = ARVRAnchor;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ARVRAnchor>().unwrap();
        world_commands.insert(entity, GDARVRAnchor {
            anchor_id: component_ref.anchor_id(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDARVRAnchor {
    
}

fn sync_bevy_owned(query: Query<(&GDARVRAnchor, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ARVRAnchor>().unwrap();
        component_ref.set_anchor_id(component.anchor_id);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDARVRAnchor, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ARVRAnchor>().unwrap();
        component.anchor_id = component_ref.anchor_id();
    }
}