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

pub struct ARVROriginPlugin;

impl Plugin for ARVROriginPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ARVROrigin>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a a_r_v_r_origin.
pub fn is_a_r_v_r_origin(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ARVROrigin>().is_some()
}

/// A bundle for ARVROrigins.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDARVROriginBundle {
    pub a_r_v_r_origin: GDARVROrigin,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDARVROriginBundle {
    fn default() -> Self {
        Self {
            a_r_v_r_origin: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ARVROrigin".to_string()
            }
        }
    }
}

/// Represents a ARVROrigin.
#[derive(Component)]
pub struct GDARVROrigin {
    pub world_scale: f64,
}

impl Default for GDARVROrigin {
    fn default() -> Self {
        Self {
            world_scale: Default::default(),
        }
    }
}

impl NodeClass for GDARVROrigin {
    type Parent = GDSpatial;
    type GodotClass = ARVROrigin;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ARVROrigin>().unwrap();
        world_commands.insert(entity, GDARVROrigin {
            world_scale: component_ref.world_scale(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDARVROrigin {
    
}

fn sync_bevy_owned(query: Query<(&GDARVROrigin, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ARVROrigin>().unwrap();
        component_ref.set_world_scale(component.world_scale);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDARVROrigin, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ARVROrigin>().unwrap();
        component.world_scale = component_ref.world_scale();
    }
}