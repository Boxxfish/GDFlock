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

pub struct Polygon2DPlugin;

impl Plugin for Polygon2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Polygon2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a polygon_2_d.
pub fn is_polygon_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Polygon2D>().is_some()
}

/// A bundle for Polygon2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDPolygon2DBundle {
    pub polygon_2_d: GDPolygon2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDPolygon2DBundle {
    fn default() -> Self {
        Self {
            polygon_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Polygon2D".to_string()
            }
        }
    }
}

/// Represents a Polygon2D.
#[derive(Component)]
pub struct GDPolygon2D {
    pub antialiased: bool,
pub color: Color,
pub internal_vertex_count: i64,
pub invert_border: f64,
pub invert_enable: bool,
pub offset: Vector2,
pub polygon: Vec<Vector2>,
pub skeleton: NodePath,
pub texture: Option<Ref<Texture>>,
pub texture_offset: Vector2,
pub texture_rotation: f64,
pub texture_rotation_degrees: f64,
pub texture_scale: Vector2,
pub uv: Vec<Vector2>,
pub vertex_colors: Vec<Color>,
}

impl Default for GDPolygon2D {
    fn default() -> Self {
        Self {
            antialiased: Default::default(),
color: Color::from_rgb(0.0, 0.0, 0.0),
internal_vertex_count: Default::default(),
invert_border: Default::default(),
invert_enable: Default::default(),
offset: Default::default(),
polygon: Default::default(),
skeleton: Default::default(),
texture: Default::default(),
texture_offset: Default::default(),
texture_rotation: Default::default(),
texture_rotation_degrees: Default::default(),
texture_scale: Default::default(),
uv: Default::default(),
vertex_colors: Default::default(),
        }
    }
}

impl NodeClass for GDPolygon2D {
    type Parent = GDNode2D;
    type GodotClass = Polygon2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Polygon2D>().unwrap();
        world_commands.insert(entity, GDPolygon2D {
            antialiased: component_ref.antialiased(),
color: component_ref.color(),
internal_vertex_count: component_ref.internal_vertex_count(),
invert_border: component_ref.invert_border(),
invert_enable: component_ref.invert(),
offset: component_ref.offset(),
polygon: component_ref.polygon().to_vec(),
skeleton: component_ref.skeleton(),
texture: component_ref.texture(),
texture_offset: component_ref.texture_offset(),
texture_rotation: component_ref.texture_rotation(),
texture_rotation_degrees: component_ref.texture_rotation_degrees(),
texture_scale: component_ref.texture_scale(),
uv: component_ref.uv().to_vec(),
vertex_colors: component_ref.vertex_colors().to_vec(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDPolygon2D {
    
}

fn sync_bevy_owned(query: Query<(&GDPolygon2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Polygon2D>().unwrap();
        component_ref.set_antialiased(component.antialiased);
component_ref.set_color(component.color);
component_ref.set_internal_vertex_count(component.internal_vertex_count);
component_ref.set_invert_border(component.invert_border);
component_ref.set_invert(component.invert_enable);
component_ref.set_offset(component.offset);
component_ref.set_polygon(Vector2Array::from_vec(component.polygon.clone()));
component_ref.set_skeleton(component.skeleton.to_godot_string());
component_ref.set_texture(component.texture.as_ref().unwrap().clone());
component_ref.set_texture_offset(component.texture_offset);
component_ref.set_texture_rotation(component.texture_rotation);
component_ref.set_texture_rotation_degrees(component.texture_rotation_degrees);
component_ref.set_texture_scale(component.texture_scale);
component_ref.set_uv(Vector2Array::from_vec(component.uv.clone()));
component_ref.set_vertex_colors(ColorArray::from_vec(component.vertex_colors.clone()));
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDPolygon2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Polygon2D>().unwrap();
        component.antialiased = component_ref.antialiased();
component.color = component_ref.color();
component.internal_vertex_count = component_ref.internal_vertex_count();
component.invert_border = component_ref.invert_border();
component.invert_enable = component_ref.invert();
component.offset = component_ref.offset();
component.polygon = component_ref.polygon().to_vec();
component.skeleton = component_ref.skeleton();
component.texture = component_ref.texture();
component.texture_offset = component_ref.texture_offset();
component.texture_rotation = component_ref.texture_rotation();
component.texture_rotation_degrees = component_ref.texture_rotation_degrees();
component.texture_scale = component_ref.texture_scale();
component.uv = component_ref.uv().to_vec();
component.vertex_colors = component_ref.vertex_colors().to_vec();
    }
}