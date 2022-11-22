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

pub struct NinePatchRectPlugin;

impl Plugin for NinePatchRectPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<NinePatchRect>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a nine_patch_rect.
pub fn is_nine_patch_rect(node: &gdnative::prelude::Node) -> bool {
    node.cast::<NinePatchRect>().is_some()
}

/// A bundle for NinePatchRects.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDNinePatchRectBundle {
    pub nine_patch_rect: GDNinePatchRect,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
    pub true_type: TrueNodeType,
}

impl Default for GDNinePatchRectBundle {
    fn default() -> Self {
        Self {
            nine_patch_rect: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "NinePatchRect".to_string()
            }
        }
    }
}

/// Represents a NinePatchRect.
#[derive(Component)]
pub struct GDNinePatchRect {
    pub draw_center: bool,
pub region_rect: Rect2,
pub texture: Option<Ref<Texture>>,
}

impl Default for GDNinePatchRect {
    fn default() -> Self {
        Self {
            draw_center: Default::default(),
region_rect: Rect2::from_components(0.0, 0.0, 0.0, 0.0),
texture: Default::default(),
        }
    }
}

impl NodeClass for GDNinePatchRect {
    type Parent = GDControl;
    type GodotClass = NinePatchRect;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<NinePatchRect>().unwrap();
        world_commands.insert(entity, GDNinePatchRect {
            draw_center: component_ref.is_draw_center_enabled(),
region_rect: component_ref.region_rect(),
texture: component_ref.texture(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDNinePatchRect {
    
}

fn sync_bevy_owned(query: Query<(&GDNinePatchRect, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<NinePatchRect>().unwrap();
        component_ref.set_draw_center(component.draw_center);
component_ref.set_region_rect(component.region_rect);
component_ref.set_texture(component.texture.as_ref().unwrap().clone());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDNinePatchRect, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<NinePatchRect>().unwrap();
        component.draw_center = component_ref.is_draw_center_enabled();
component.region_rect = component_ref.region_rect();
component.texture = component_ref.texture();
    }
}