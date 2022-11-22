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

pub struct TweenPlugin;

impl Plugin for TweenPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Tween>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a tween.
pub fn is_tween(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Tween>().is_some()
}

/// A bundle for Tweens.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDTweenBundle {
    pub tween: GDTween,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDTweenBundle {
    fn default() -> Self {
        Self {
            tween: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Tween".to_string()
            }
        }
    }
}

/// Represents a Tween.
#[derive(Component)]
pub struct GDTween {
    pub playback_speed: f64,
pub repeat: bool,
}

impl Default for GDTween {
    fn default() -> Self {
        Self {
            playback_speed: Default::default(),
repeat: Default::default(),
        }
    }
}

impl NodeClass for GDTween {
    type Parent = GDNode;
    type GodotClass = Tween;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Tween>().unwrap();
        world_commands.insert(entity, GDTween {
            playback_speed: component_ref.speed_scale(),
repeat: component_ref.is_repeat(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDTween {
    
}

fn sync_bevy_owned(query: Query<(&GDTween, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Tween>().unwrap();
        component_ref.set_speed_scale(component.playback_speed);
component_ref.set_repeat(component.repeat);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDTween, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Tween>().unwrap();
        component.playback_speed = component_ref.speed_scale();
component.repeat = component_ref.is_repeat();
    }
}