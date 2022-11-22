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

pub struct ReferenceRectPlugin;

impl Plugin for ReferenceRectPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ReferenceRect>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a reference_rect.
pub fn is_reference_rect(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ReferenceRect>().is_some()
}

/// A bundle for ReferenceRects.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDReferenceRectBundle {
    pub reference_rect: GDReferenceRect,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
    pub true_type: TrueNodeType,
}

impl Default for GDReferenceRectBundle {
    fn default() -> Self {
        Self {
            reference_rect: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ReferenceRect".to_string()
            }
        }
    }
}

/// Represents a ReferenceRect.
#[derive(Component)]
pub struct GDReferenceRect {
    pub border_color: Color,
pub border_width: f64,
pub editor_only: bool,
}

impl Default for GDReferenceRect {
    fn default() -> Self {
        Self {
            border_color: Color::from_rgb(0.0, 0.0, 0.0),
border_width: Default::default(),
editor_only: Default::default(),
        }
    }
}

impl NodeClass for GDReferenceRect {
    type Parent = GDControl;
    type GodotClass = ReferenceRect;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ReferenceRect>().unwrap();
        world_commands.insert(entity, GDReferenceRect {
            border_color: component_ref.border_color(),
border_width: component_ref.border_width(),
editor_only: component_ref.editor_only(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDReferenceRect {
    
}

fn sync_bevy_owned(query: Query<(&GDReferenceRect, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ReferenceRect>().unwrap();
        component_ref.set_border_color(component.border_color);
component_ref.set_border_width(component.border_width);
component_ref.set_editor_only(component.editor_only);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDReferenceRect, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ReferenceRect>().unwrap();
        component.border_color = component_ref.border_color();
component.border_width = component_ref.border_width();
component.editor_only = component_ref.editor_only();
    }
}