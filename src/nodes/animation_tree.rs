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

pub struct AnimationTreePlugin;

impl Plugin for AnimationTreePlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<AnimationTree>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a animation_tree.
pub fn is_animation_tree(node: &gdnative::prelude::Node) -> bool {
    node.cast::<AnimationTree>().is_some()
}

/// A bundle for AnimationTrees.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDAnimationTreeBundle {
    pub animation_tree: GDAnimationTree,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDAnimationTreeBundle {
    fn default() -> Self {
        Self {
            animation_tree: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "AnimationTree".to_string()
            }
        }
    }
}

/// Represents a AnimationTree.
#[derive(Component)]
pub struct GDAnimationTree {
    pub active: bool,
pub anim_player: NodePath,
pub root_motion_track: NodePath,
}

impl Default for GDAnimationTree {
    fn default() -> Self {
        Self {
            active: Default::default(),
anim_player: Default::default(),
root_motion_track: Default::default(),
        }
    }
}

impl NodeClass for GDAnimationTree {
    type Parent = GDNode;
    type GodotClass = AnimationTree;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<AnimationTree>().unwrap();
        world_commands.insert(entity, GDAnimationTree {
            active: component_ref.is_active(),
anim_player: component_ref.animation_player(),
root_motion_track: component_ref.root_motion_track(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDAnimationTree {
    
}

fn sync_bevy_owned(query: Query<(&GDAnimationTree, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AnimationTree>().unwrap();
        component_ref.set_active(component.active);
component_ref.set_animation_player(component.anim_player.to_godot_string());
component_ref.set_root_motion_track(component.root_motion_track.to_godot_string());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDAnimationTree, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AnimationTree>().unwrap();
        component.active = component_ref.is_active();
component.anim_player = component_ref.animation_player();
component.root_motion_track = component_ref.root_motion_track();
    }
}