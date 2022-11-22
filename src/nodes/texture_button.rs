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

pub struct TextureButtonPlugin;

impl Plugin for TextureButtonPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<TextureButton>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a texture_button.
pub fn is_texture_button(node: &gdnative::prelude::Node) -> bool {
    node.cast::<TextureButton>().is_some()
}

/// A bundle for TextureButtons.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDTextureButtonBundle {
    pub texture_button: GDTextureButton,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub base_button: GDBaseButton,
    pub true_type: TrueNodeType,
}

impl Default for GDTextureButtonBundle {
    fn default() -> Self {
        Self {
            texture_button: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
base_button: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "TextureButton".to_string()
            }
        }
    }
}

/// Represents a TextureButton.
#[derive(Component)]
pub struct GDTextureButton {
    pub expand: bool,
pub flip_h: bool,
pub flip_v: bool,
pub texture_disabled: Option<Ref<Texture>>,
pub texture_focused: Option<Ref<Texture>>,
pub texture_hover: Option<Ref<Texture>>,
pub texture_normal: Option<Ref<Texture>>,
pub texture_pressed: Option<Ref<Texture>>,
}

impl Default for GDTextureButton {
    fn default() -> Self {
        Self {
            expand: Default::default(),
flip_h: Default::default(),
flip_v: Default::default(),
texture_disabled: Default::default(),
texture_focused: Default::default(),
texture_hover: Default::default(),
texture_normal: Default::default(),
texture_pressed: Default::default(),
        }
    }
}

impl NodeClass for GDTextureButton {
    type Parent = GDBaseButton;
    type GodotClass = TextureButton;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<TextureButton>().unwrap();
        world_commands.insert(entity, GDTextureButton {
            expand: component_ref.expand(),
flip_h: component_ref.is_flipped_h(),
flip_v: component_ref.is_flipped_v(),
texture_disabled: component_ref.disabled_texture(),
texture_focused: component_ref.focused_texture(),
texture_hover: component_ref.hover_texture(),
texture_normal: component_ref.normal_texture(),
texture_pressed: component_ref.pressed_texture(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDTextureButton {
    
}

fn sync_bevy_owned(query: Query<(&GDTextureButton, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<TextureButton>().unwrap();
        component_ref.set_expand(component.expand);
component_ref.set_flip_h(component.flip_h);
component_ref.set_flip_v(component.flip_v);
component_ref.set_disabled_texture(component.texture_disabled.as_ref().unwrap().clone());
component_ref.set_focused_texture(component.texture_focused.as_ref().unwrap().clone());
component_ref.set_hover_texture(component.texture_hover.as_ref().unwrap().clone());
component_ref.set_normal_texture(component.texture_normal.as_ref().unwrap().clone());
component_ref.set_pressed_texture(component.texture_pressed.as_ref().unwrap().clone());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDTextureButton, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<TextureButton>().unwrap();
        component.expand = component_ref.expand();
component.flip_h = component_ref.is_flipped_h();
component.flip_v = component_ref.is_flipped_v();
component.texture_disabled = component_ref.disabled_texture();
component.texture_focused = component_ref.focused_texture();
component.texture_hover = component_ref.hover_texture();
component.texture_normal = component_ref.normal_texture();
component.texture_pressed = component_ref.pressed_texture();
    }
}