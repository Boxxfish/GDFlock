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

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Sprite>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a sprite.
pub fn is_sprite(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Sprite>().is_some()
}

/// A bundle for Sprites.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDSpriteBundle {
    pub sprite: GDSprite,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDSpriteBundle {
    fn default() -> Self {
        Self {
            sprite: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Sprite".to_string()
            }
        }
    }
}

/// Represents a Sprite.
#[derive(Component)]
pub struct GDSprite {
    pub centered: bool,
pub flip_h: bool,
pub flip_v: bool,
pub frame: i64,
pub frame_coords: Vector2,
pub hframes: i64,
pub normal_map: Option<Ref<Texture>>,
pub offset: Vector2,
pub region_enabled: bool,
pub region_filter_clip: bool,
pub region_rect: Rect2,
pub texture: Option<Ref<Texture>>,
pub vframes: i64,
}

impl Default for GDSprite {
    fn default() -> Self {
        Self {
            centered: Default::default(),
flip_h: Default::default(),
flip_v: Default::default(),
frame: Default::default(),
frame_coords: Default::default(),
hframes: Default::default(),
normal_map: Default::default(),
offset: Default::default(),
region_enabled: Default::default(),
region_filter_clip: Default::default(),
region_rect: Rect2::from_components(0.0, 0.0, 0.0, 0.0),
texture: Default::default(),
vframes: Default::default(),
        }
    }
}

impl NodeClass for GDSprite {
    type Parent = GDNode2D;
    type GodotClass = Sprite;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Sprite>().unwrap();
        world_commands.insert(entity, GDSprite {
            centered: component_ref.is_centered(),
flip_h: component_ref.is_flipped_h(),
flip_v: component_ref.is_flipped_v(),
frame: component_ref.frame(),
frame_coords: component_ref.frame_coords(),
hframes: component_ref.hframes(),
normal_map: component_ref.normal_map(),
offset: component_ref.offset(),
region_enabled: component_ref.is_region(),
region_filter_clip: component_ref.is_region_filter_clip_enabled(),
region_rect: component_ref.region_rect(),
texture: component_ref.texture(),
vframes: component_ref.vframes(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDSprite {
    
}

fn sync_bevy_owned(query: Query<(&GDSprite, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Sprite>().unwrap();
        component_ref.set_centered(component.centered);
component_ref.set_flip_h(component.flip_h);
component_ref.set_flip_v(component.flip_v);
component_ref.set_frame(component.frame);
component_ref.set_frame_coords(component.frame_coords);
component_ref.set_hframes(component.hframes);
component_ref.set_normal_map(component.normal_map.as_ref().unwrap().clone());
component_ref.set_offset(component.offset);
component_ref.set_region(component.region_enabled);
component_ref.set_region_filter_clip(component.region_filter_clip);
component_ref.set_region_rect(component.region_rect);
component_ref.set_texture(component.texture.as_ref().unwrap().clone());
component_ref.set_vframes(component.vframes);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDSprite, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Sprite>().unwrap();
        component.centered = component_ref.is_centered();
component.flip_h = component_ref.is_flipped_h();
component.flip_v = component_ref.is_flipped_v();
component.frame = component_ref.frame();
component.frame_coords = component_ref.frame_coords();
component.hframes = component_ref.hframes();
component.normal_map = component_ref.normal_map();
component.offset = component_ref.offset();
component.region_enabled = component_ref.is_region();
component.region_filter_clip = component_ref.is_region_filter_clip_enabled();
component.region_rect = component_ref.region_rect();
component.texture = component_ref.texture();
component.vframes = component_ref.vframes();
    }
}