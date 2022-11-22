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

pub struct CanvasLayerPlugin;

impl Plugin for CanvasLayerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<CanvasLayer>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a canvas_layer.
pub fn is_canvas_layer(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CanvasLayer>().is_some()
}

/// A bundle for CanvasLayers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCanvasLayerBundle {
    pub canvas_layer: GDCanvasLayer,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDCanvasLayerBundle {
    fn default() -> Self {
        Self {
            canvas_layer: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CanvasLayer".to_string()
            }
        }
    }
}

/// Represents a CanvasLayer.
#[derive(Component)]
pub struct GDCanvasLayer {
    pub follow_viewport_enable: bool,
pub follow_viewport_scale: f64,
pub layer: i64,
pub offset: Vector2,
pub rotation: f64,
pub rotation_degrees: f64,
pub scale: Vector2,
pub transform: Transform2D,
pub visible: bool,
}

impl Default for GDCanvasLayer {
    fn default() -> Self {
        Self {
            follow_viewport_enable: Default::default(),
follow_viewport_scale: Default::default(),
layer: Default::default(),
offset: Default::default(),
rotation: Default::default(),
rotation_degrees: Default::default(),
scale: Default::default(),
transform: Transform2D::IDENTITY,
visible: Default::default(),
        }
    }
}

impl NodeClass for GDCanvasLayer {
    type Parent = GDNode;
    type GodotClass = CanvasLayer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CanvasLayer>().unwrap();
        world_commands.insert(entity, GDCanvasLayer {
            follow_viewport_enable: component_ref.is_following_viewport(),
follow_viewport_scale: component_ref.follow_viewport_scale(),
layer: component_ref.layer(),
offset: component_ref.offset(),
rotation: component_ref.rotation(),
rotation_degrees: component_ref.rotation_degrees(),
scale: component_ref.scale(),
transform: component_ref.transform(),
visible: component_ref.is_visible(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCanvasLayer {
    
}

fn sync_bevy_owned(query: Query<(&GDCanvasLayer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CanvasLayer>().unwrap();
        component_ref.set_follow_viewport(component.follow_viewport_enable);
component_ref.set_follow_viewport_scale(component.follow_viewport_scale);
component_ref.set_layer(component.layer);
component_ref.set_offset(component.offset);
component_ref.set_rotation(component.rotation);
component_ref.set_rotation_degrees(component.rotation_degrees);
component_ref.set_scale(component.scale);
component_ref.set_transform(component.transform);
component_ref.set_visible(component.visible);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCanvasLayer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CanvasLayer>().unwrap();
        component.follow_viewport_enable = component_ref.is_following_viewport();
component.follow_viewport_scale = component_ref.follow_viewport_scale();
component.layer = component_ref.layer();
component.offset = component_ref.offset();
component.rotation = component_ref.rotation();
component.rotation_degrees = component_ref.rotation_degrees();
component.scale = component_ref.scale();
component.transform = component_ref.transform();
component.visible = component_ref.is_visible();
    }
}