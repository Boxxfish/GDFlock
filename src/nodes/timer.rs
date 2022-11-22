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

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Timer>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a timer.
pub fn is_timer(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Timer>().is_some()
}

/// A bundle for Timers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDTimerBundle {
    pub timer: GDTimer,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDTimerBundle {
    fn default() -> Self {
        Self {
            timer: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Timer".to_string()
            }
        }
    }
}

/// Represents a Timer.
#[derive(Component)]
pub struct GDTimer {
    pub autostart: bool,
pub one_shot: bool,
pub paused: bool,
pub wait_time: f64,
}

impl Default for GDTimer {
    fn default() -> Self {
        Self {
            autostart: Default::default(),
one_shot: Default::default(),
paused: Default::default(),
wait_time: Default::default(),
        }
    }
}

impl NodeClass for GDTimer {
    type Parent = GDNode;
    type GodotClass = Timer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Timer>().unwrap();
        world_commands.insert(entity, GDTimer {
            autostart: component_ref.has_autostart(),
one_shot: component_ref.is_one_shot(),
paused: component_ref.is_paused(),
wait_time: component_ref.wait_time(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDTimer {
    
}

fn sync_bevy_owned(query: Query<(&GDTimer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Timer>().unwrap();
        component_ref.set_autostart(component.autostart);
component_ref.set_one_shot(component.one_shot);
component_ref.set_paused(component.paused);
component_ref.set_wait_time(component.wait_time);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDTimer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Timer>().unwrap();
        component.autostart = component_ref.has_autostart();
component.one_shot = component_ref.is_one_shot();
component.paused = component_ref.is_paused();
component.wait_time = component_ref.wait_time();
    }
}