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

pub struct TextureRectPlugin;

impl Plugin for TextureRectPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<TextureRect>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a texture_rect.
pub fn is_texture_rect(node: &gdnative::prelude::Node) -> bool {
    node.cast::<TextureRect>().is_some()
}

/// A bundle for TextureRects.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDTextureRectBundle {
    pub texture_rect: GDTextureRect,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
    pub true_type: TrueNodeType,
}

impl Default for GDTextureRectBundle {
    fn default() -> Self {
        Self {
            texture_rect: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "TextureRect".to_string()
            }
        }
    }
}

/// Represents a TextureRect.
#[derive(Component)]
pub struct GDTextureRect {
    pub expand: bool,
pub flip_h: bool,
pub flip_v: bool,
pub texture: Option<Ref<Texture>>,
}

impl Default for GDTextureRect {
    fn default() -> Self {
        Self {
            expand: Default::default(),
flip_h: Default::default(),
flip_v: Default::default(),
texture: Default::default(),
        }
    }
}

impl NodeClass for GDTextureRect {
    type Parent = GDControl;
    type GodotClass = TextureRect;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<TextureRect>().unwrap();
        world_commands.insert(entity, GDTextureRect {
            expand: component_ref.has_expand(),
flip_h: component_ref.is_flipped_h(),
flip_v: component_ref.is_flipped_v(),
texture: component_ref.texture(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDTextureRect {
    
}

fn sync_bevy_owned(query: Query<(&GDTextureRect, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<TextureRect>().unwrap();
        component_ref.set_expand(component.expand);
component_ref.set_flip_h(component.flip_h);
component_ref.set_flip_v(component.flip_v);
component_ref.set_texture(component.texture.as_ref().unwrap().clone());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDTextureRect, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<TextureRect>().unwrap();
        component.expand = component_ref.has_expand();
component.flip_h = component_ref.is_flipped_h();
component.flip_v = component_ref.is_flipped_v();
component.texture = component_ref.texture();
    }
}