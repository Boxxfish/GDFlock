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

pub struct CanvasItemPlugin;

impl Plugin for CanvasItemPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a canvas_item.
pub fn is_canvas_item(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CanvasItem>().is_some()
}

/// A bundle for CanvasItems.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCanvasItemBundle {
    pub canvas_item: GDCanvasItem,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDCanvasItemBundle {
    fn default() -> Self {
        Self {
            canvas_item: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CanvasItem".to_string()
            }
        }
    }
}

/// Represents a CanvasItem.
#[derive(Component)]
pub struct GDCanvasItem {
    pub light_mask: i64,
pub modulate: Color,
pub self_modulate: Color,
pub show_behind_parent: bool,
pub use_parent_material: bool,
pub visible: bool,
}

impl Default for GDCanvasItem {
    fn default() -> Self {
        Self {
            light_mask: Default::default(),
modulate: Color::from_rgb(0.0, 0.0, 0.0),
self_modulate: Color::from_rgb(0.0, 0.0, 0.0),
show_behind_parent: Default::default(),
use_parent_material: Default::default(),
visible: Default::default(),
        }
    }
}

impl NodeClass for GDCanvasItem {
    type Parent = GDNode;
    type GodotClass = CanvasItem;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CanvasItem>().unwrap();
        world_commands.insert(entity, GDCanvasItem {
            light_mask: component_ref.light_mask(),
modulate: component_ref.modulate(),
self_modulate: component_ref.self_modulate(),
show_behind_parent: component_ref.is_draw_behind_parent_enabled(),
use_parent_material: component_ref.use_parent_material(),
visible: component_ref.is_visible(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCanvasItem {
    
}

fn sync_bevy_owned(query: Query<(&GDCanvasItem, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CanvasItem>().unwrap();
        component_ref.set_light_mask(component.light_mask);
component_ref.set_modulate(component.modulate);
component_ref.set_self_modulate(component.self_modulate);
component_ref.set_draw_behind_parent(component.show_behind_parent);
component_ref.set_use_parent_material(component.use_parent_material);
component_ref.set_visible(component.visible);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCanvasItem, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CanvasItem>().unwrap();
        component.light_mask = component_ref.light_mask();
component.modulate = component_ref.modulate();
component.self_modulate = component_ref.self_modulate();
component.show_behind_parent = component_ref.is_draw_behind_parent_enabled();
component.use_parent_material = component_ref.use_parent_material();
component.visible = component_ref.is_visible();
    }
}