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

pub struct TouchScreenButtonPlugin;

impl Plugin for TouchScreenButtonPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<TouchScreenButton>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a touch_screen_button.
pub fn is_touch_screen_button(node: &gdnative::prelude::Node) -> bool {
    node.cast::<TouchScreenButton>().is_some()
}

/// A bundle for TouchScreenButtons.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDTouchScreenButtonBundle {
    pub touch_screen_button: GDTouchScreenButton,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDTouchScreenButtonBundle {
    fn default() -> Self {
        Self {
            touch_screen_button: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "TouchScreenButton".to_string()
            }
        }
    }
}

/// Represents a TouchScreenButton.
#[derive(Component)]
pub struct GDTouchScreenButton {
    pub action: String,
pub normal: Option<Ref<Texture>>,
pub passby_press: bool,
pub pressed: Option<Ref<Texture>>,
pub shape_centered: bool,
pub shape_visible: bool,
}

impl Default for GDTouchScreenButton {
    fn default() -> Self {
        Self {
            action: Default::default(),
normal: Default::default(),
passby_press: Default::default(),
pressed: Default::default(),
shape_centered: Default::default(),
shape_visible: Default::default(),
        }
    }
}

impl NodeClass for GDTouchScreenButton {
    type Parent = GDNode2D;
    type GodotClass = TouchScreenButton;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<TouchScreenButton>().unwrap();
        world_commands.insert(entity, GDTouchScreenButton {
            action: component_ref.action().to_string(),
normal: component_ref.texture(),
passby_press: component_ref.is_passby_press_enabled(),
pressed: component_ref.texture_pressed(),
shape_centered: component_ref.is_shape_centered(),
shape_visible: component_ref.is_shape_visible(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDTouchScreenButton {
    
}

fn sync_bevy_owned(query: Query<(&GDTouchScreenButton, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<TouchScreenButton>().unwrap();
        component_ref.set_action(component.action.clone());
component_ref.set_texture(component.normal.as_ref().unwrap().clone());
component_ref.set_passby_press(component.passby_press);
component_ref.set_texture_pressed(component.pressed.as_ref().unwrap().clone());
component_ref.set_shape_centered(component.shape_centered);
component_ref.set_shape_visible(component.shape_visible);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDTouchScreenButton, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<TouchScreenButton>().unwrap();
        component.action = component_ref.action().to_string();
component.normal = component_ref.texture();
component.passby_press = component_ref.is_passby_press_enabled();
component.pressed = component_ref.texture_pressed();
component.shape_centered = component_ref.is_shape_centered();
component.shape_visible = component_ref.is_shape_visible();
    }
}