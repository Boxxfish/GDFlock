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

pub struct ColorRectPlugin;

impl Plugin for ColorRectPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ColorRect>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a color_rect.
pub fn is_color_rect(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ColorRect>().is_some()
}

/// A bundle for ColorRects.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDColorRectBundle {
    pub color_rect: GDColorRect,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
    pub true_type: TrueNodeType,
}

impl Default for GDColorRectBundle {
    fn default() -> Self {
        Self {
            color_rect: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ColorRect".to_string()
            }
        }
    }
}

/// Represents a ColorRect.
#[derive(Component)]
pub struct GDColorRect {
    pub color: Color,
}

impl Default for GDColorRect {
    fn default() -> Self {
        Self {
            color: Color::from_rgb(0.0, 0.0, 0.0),
        }
    }
}

impl NodeClass for GDColorRect {
    type Parent = GDControl;
    type GodotClass = ColorRect;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ColorRect>().unwrap();
        world_commands.insert(entity, GDColorRect {
            color: component_ref.frame_color(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDColorRect {
    
}

fn sync_bevy_owned(query: Query<(&GDColorRect, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ColorRect>().unwrap();
        component_ref.set_frame_color(component.color);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDColorRect, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ColorRect>().unwrap();
        component.color = component_ref.frame_color();
    }
}