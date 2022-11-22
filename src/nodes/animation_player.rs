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

pub struct AnimationPlayerPlugin;

impl Plugin for AnimationPlayerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<AnimationPlayer>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a animation_player.
pub fn is_animation_player(node: &gdnative::prelude::Node) -> bool {
    node.cast::<AnimationPlayer>().is_some()
}

/// A bundle for AnimationPlayers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDAnimationPlayerBundle {
    pub animation_player: GDAnimationPlayer,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDAnimationPlayerBundle {
    fn default() -> Self {
        Self {
            animation_player: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "AnimationPlayer".to_string()
            }
        }
    }
}

/// Represents a AnimationPlayer.
#[derive(Component)]
pub struct GDAnimationPlayer {
    pub assigned_animation: String,
pub autoplay: String,
pub current_animation: String,
pub playback_active: bool,
pub playback_default_blend_time: f64,
pub playback_speed: f64,
pub reset_on_save: bool,
pub root_node: NodePath,
}

impl Default for GDAnimationPlayer {
    fn default() -> Self {
        Self {
            assigned_animation: Default::default(),
autoplay: Default::default(),
current_animation: Default::default(),
playback_active: Default::default(),
playback_default_blend_time: Default::default(),
playback_speed: Default::default(),
reset_on_save: Default::default(),
root_node: Default::default(),
        }
    }
}

impl NodeClass for GDAnimationPlayer {
    type Parent = GDNode;
    type GodotClass = AnimationPlayer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<AnimationPlayer>().unwrap();
        world_commands.insert(entity, GDAnimationPlayer {
            assigned_animation: component_ref.assigned_animation().to_string(),
autoplay: component_ref.autoplay().to_string(),
current_animation: component_ref.current_animation().to_string(),
playback_active: component_ref.is_active(),
playback_default_blend_time: component_ref.default_blend_time(),
playback_speed: component_ref.speed_scale(),
reset_on_save: component_ref.is_reset_on_save_enabled(),
root_node: component_ref.root(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDAnimationPlayer {
    
}

fn sync_bevy_owned(query: Query<(&GDAnimationPlayer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AnimationPlayer>().unwrap();
        component_ref.set_assigned_animation(component.assigned_animation.clone());
component_ref.set_autoplay(component.autoplay.clone());
component_ref.set_current_animation(component.current_animation.clone());
component_ref.set_active(component.playback_active);
component_ref.set_default_blend_time(component.playback_default_blend_time);
component_ref.set_speed_scale(component.playback_speed);
component_ref.set_reset_on_save_enabled(component.reset_on_save);
component_ref.set_root(component.root_node.to_godot_string());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDAnimationPlayer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AnimationPlayer>().unwrap();
        component.assigned_animation = component_ref.assigned_animation().to_string();
component.autoplay = component_ref.autoplay().to_string();
component.current_animation = component_ref.current_animation().to_string();
component.playback_active = component_ref.is_active();
component.playback_default_blend_time = component_ref.default_blend_time();
component.playback_speed = component_ref.speed_scale();
component.reset_on_save = component_ref.is_reset_on_save_enabled();
component.root_node = component_ref.root();
    }
}