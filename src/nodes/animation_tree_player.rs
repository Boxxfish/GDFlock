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

pub struct AnimationTreePlayerPlugin;

impl Plugin for AnimationTreePlayerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<AnimationTreePlayer>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a animation_tree_player.
pub fn is_animation_tree_player(node: &gdnative::prelude::Node) -> bool {
    node.cast::<AnimationTreePlayer>().is_some()
}

/// A bundle for AnimationTreePlayers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDAnimationTreePlayerBundle {
    pub animation_tree_player: GDAnimationTreePlayer,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDAnimationTreePlayerBundle {
    fn default() -> Self {
        Self {
            animation_tree_player: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "AnimationTreePlayer".to_string()
            }
        }
    }
}

/// Represents a AnimationTreePlayer.
#[derive(Component)]
pub struct GDAnimationTreePlayer {
    pub active: bool,
pub base_path: NodePath,
pub master_player: NodePath,
}

impl Default for GDAnimationTreePlayer {
    fn default() -> Self {
        Self {
            active: Default::default(),
base_path: Default::default(),
master_player: Default::default(),
        }
    }
}

impl NodeClass for GDAnimationTreePlayer {
    type Parent = GDNode;
    type GodotClass = AnimationTreePlayer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<AnimationTreePlayer>().unwrap();
        world_commands.insert(entity, GDAnimationTreePlayer {
            active: component_ref.is_active(),
base_path: component_ref.base_path(),
master_player: component_ref.master_player(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDAnimationTreePlayer {
    
}

fn sync_bevy_owned(query: Query<(&GDAnimationTreePlayer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AnimationTreePlayer>().unwrap();
        component_ref.set_active(component.active);
component_ref.set_base_path(component.base_path.to_godot_string());
component_ref.set_master_player(component.master_player.to_godot_string());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDAnimationTreePlayer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AnimationTreePlayer>().unwrap();
        component.active = component_ref.is_active();
component.base_path = component_ref.base_path();
component.master_player = component_ref.master_player();
    }
}