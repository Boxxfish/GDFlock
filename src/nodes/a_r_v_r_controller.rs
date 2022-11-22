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

pub struct ARVRControllerPlugin;

impl Plugin for ARVRControllerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ARVRController>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a a_r_v_r_controller.
pub fn is_a_r_v_r_controller(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ARVRController>().is_some()
}

/// A bundle for ARVRControllers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDARVRControllerBundle {
    pub a_r_v_r_controller: GDARVRController,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDARVRControllerBundle {
    fn default() -> Self {
        Self {
            a_r_v_r_controller: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ARVRController".to_string()
            }
        }
    }
}

/// Represents a ARVRController.
#[derive(Component)]
pub struct GDARVRController {
    pub controller_id: i64,
pub rumble: f64,
}

impl Default for GDARVRController {
    fn default() -> Self {
        Self {
            controller_id: Default::default(),
rumble: Default::default(),
        }
    }
}

impl NodeClass for GDARVRController {
    type Parent = GDSpatial;
    type GodotClass = ARVRController;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ARVRController>().unwrap();
        world_commands.insert(entity, GDARVRController {
            controller_id: component_ref.controller_id(),
rumble: component_ref.rumble(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDARVRController {
    
}

fn sync_bevy_owned(query: Query<(&GDARVRController, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ARVRController>().unwrap();
        component_ref.set_controller_id(component.controller_id);
component_ref.set_rumble(component.rumble);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDARVRController, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ARVRController>().unwrap();
        component.controller_id = component_ref.controller_id();
component.rumble = component_ref.rumble();
    }
}