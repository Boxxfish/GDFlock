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

pub struct Light2DPlugin;

impl Plugin for Light2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Light2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a light_2_d.
pub fn is_light_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Light2D>().is_some()
}

/// A bundle for Light2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDLight2DBundle {
    pub light_2_d: GDLight2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDLight2DBundle {
    fn default() -> Self {
        Self {
            light_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Light2D".to_string()
            }
        }
    }
}

/// Represents a Light2D.
#[derive(Component)]
pub struct GDLight2D {
    pub color: Color,
pub editor_only: bool,
pub enabled: bool,
pub energy: f64,
pub offset: Vector2,
pub range_height: f64,
pub range_item_cull_mask: i64,
pub range_layer_max: i64,
pub range_layer_min: i64,
pub range_z_max: i64,
pub range_z_min: i64,
pub shadow_buffer_size: i64,
pub shadow_color: Color,
pub shadow_enabled: bool,
pub shadow_filter_smooth: f64,
pub shadow_gradient_length: f64,
pub shadow_item_cull_mask: i64,
pub texture: Option<Ref<Texture>>,
pub texture_scale: f64,
}

impl Default for GDLight2D {
    fn default() -> Self {
        Self {
            color: Color::from_rgb(0.0, 0.0, 0.0),
editor_only: Default::default(),
enabled: Default::default(),
energy: Default::default(),
offset: Default::default(),
range_height: Default::default(),
range_item_cull_mask: Default::default(),
range_layer_max: Default::default(),
range_layer_min: Default::default(),
range_z_max: Default::default(),
range_z_min: Default::default(),
shadow_buffer_size: Default::default(),
shadow_color: Color::from_rgb(0.0, 0.0, 0.0),
shadow_enabled: Default::default(),
shadow_filter_smooth: Default::default(),
shadow_gradient_length: Default::default(),
shadow_item_cull_mask: Default::default(),
texture: Default::default(),
texture_scale: Default::default(),
        }
    }
}

impl NodeClass for GDLight2D {
    type Parent = GDNode2D;
    type GodotClass = Light2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Light2D>().unwrap();
        world_commands.insert(entity, GDLight2D {
            color: component_ref.color(),
editor_only: component_ref.is_editor_only(),
enabled: component_ref.is_enabled(),
energy: component_ref.energy(),
offset: component_ref.texture_offset(),
range_height: component_ref.height(),
range_item_cull_mask: component_ref.item_cull_mask(),
range_layer_max: component_ref.layer_range_max(),
range_layer_min: component_ref.layer_range_min(),
range_z_max: component_ref.z_range_max(),
range_z_min: component_ref.z_range_min(),
shadow_buffer_size: component_ref.shadow_buffer_size(),
shadow_color: component_ref.shadow_color(),
shadow_enabled: component_ref.is_shadow_enabled(),
shadow_filter_smooth: component_ref.shadow_smooth(),
shadow_gradient_length: component_ref.shadow_gradient_length(),
shadow_item_cull_mask: component_ref.item_shadow_cull_mask(),
texture: component_ref.texture(),
texture_scale: component_ref.texture_scale(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDLight2D {
    
}

fn sync_bevy_owned(query: Query<(&GDLight2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Light2D>().unwrap();
        component_ref.set_color(component.color);
component_ref.set_editor_only(component.editor_only);
component_ref.set_enabled(component.enabled);
component_ref.set_energy(component.energy);
component_ref.set_texture_offset(component.offset);
component_ref.set_height(component.range_height);
component_ref.set_item_cull_mask(component.range_item_cull_mask);
component_ref.set_layer_range_max(component.range_layer_max);
component_ref.set_layer_range_min(component.range_layer_min);
component_ref.set_z_range_max(component.range_z_max);
component_ref.set_z_range_min(component.range_z_min);
component_ref.set_shadow_buffer_size(component.shadow_buffer_size);
component_ref.set_shadow_color(component.shadow_color);
component_ref.set_shadow_enabled(component.shadow_enabled);
component_ref.set_shadow_smooth(component.shadow_filter_smooth);
component_ref.set_shadow_gradient_length(component.shadow_gradient_length);
component_ref.set_item_shadow_cull_mask(component.shadow_item_cull_mask);
component_ref.set_texture(component.texture.as_ref().unwrap().clone());
component_ref.set_texture_scale(component.texture_scale);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDLight2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Light2D>().unwrap();
        component.color = component_ref.color();
component.editor_only = component_ref.is_editor_only();
component.enabled = component_ref.is_enabled();
component.energy = component_ref.energy();
component.offset = component_ref.texture_offset();
component.range_height = component_ref.height();
component.range_item_cull_mask = component_ref.item_cull_mask();
component.range_layer_max = component_ref.layer_range_max();
component.range_layer_min = component_ref.layer_range_min();
component.range_z_max = component_ref.z_range_max();
component.range_z_min = component_ref.z_range_min();
component.shadow_buffer_size = component_ref.shadow_buffer_size();
component.shadow_color = component_ref.shadow_color();
component.shadow_enabled = component_ref.is_shadow_enabled();
component.shadow_filter_smooth = component_ref.shadow_smooth();
component.shadow_gradient_length = component_ref.shadow_gradient_length();
component.shadow_item_cull_mask = component_ref.item_shadow_cull_mask();
component.texture = component_ref.texture();
component.texture_scale = component_ref.texture_scale();
    }
}