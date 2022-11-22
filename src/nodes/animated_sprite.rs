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

pub struct AnimatedSpritePlugin;

impl Plugin for AnimatedSpritePlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<AnimatedSprite>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a animated_sprite.
pub fn is_animated_sprite(node: &gdnative::prelude::Node) -> bool {
    node.cast::<AnimatedSprite>().is_some()
}

/// A bundle for AnimatedSprites.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDAnimatedSpriteBundle {
    pub animated_sprite: GDAnimatedSprite,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDAnimatedSpriteBundle {
    fn default() -> Self {
        Self {
            animated_sprite: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "AnimatedSprite".to_string()
            }
        }
    }
}

/// Represents a AnimatedSprite.
#[derive(Component)]
pub struct GDAnimatedSprite {
    pub animation: String,
pub centered: bool,
pub flip_h: bool,
pub flip_v: bool,
pub frame: i64,
pub offset: Vector2,
pub playing: bool,
pub speed_scale: f64,
}

impl Default for GDAnimatedSprite {
    fn default() -> Self {
        Self {
            animation: Default::default(),
centered: Default::default(),
flip_h: Default::default(),
flip_v: Default::default(),
frame: Default::default(),
offset: Default::default(),
playing: Default::default(),
speed_scale: Default::default(),
        }
    }
}

impl NodeClass for GDAnimatedSprite {
    type Parent = GDNode2D;
    type GodotClass = AnimatedSprite;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<AnimatedSprite>().unwrap();
        world_commands.insert(entity, GDAnimatedSprite {
            animation: component_ref.animation().to_string(),
centered: component_ref.is_centered(),
flip_h: component_ref.is_flipped_h(),
flip_v: component_ref.is_flipped_v(),
frame: component_ref.frame(),
offset: component_ref.offset(),
playing: component_ref.is_playing(),
speed_scale: component_ref.speed_scale(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDAnimatedSprite {
    
}

fn sync_bevy_owned(query: Query<(&GDAnimatedSprite, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AnimatedSprite>().unwrap();
        component_ref.set_animation(component.animation.clone());
component_ref.set_centered(component.centered);
component_ref.set_flip_h(component.flip_h);
component_ref.set_flip_v(component.flip_v);
component_ref.set_frame(component.frame);
component_ref.set_offset(component.offset);
component_ref.set_playing(component.playing);
component_ref.set_speed_scale(component.speed_scale);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDAnimatedSprite, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AnimatedSprite>().unwrap();
        component.animation = component_ref.animation().to_string();
component.centered = component_ref.is_centered();
component.flip_h = component_ref.is_flipped_h();
component.flip_v = component_ref.is_flipped_v();
component.frame = component_ref.frame();
component.offset = component_ref.offset();
component.playing = component_ref.is_playing();
component.speed_scale = component_ref.speed_scale();
    }
}