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

pub struct Line2DPlugin;

impl Plugin for Line2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Line2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a line_2_d.
pub fn is_line_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Line2D>().is_some()
}

/// A bundle for Line2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDLine2DBundle {
    pub line_2_d: GDLine2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDLine2DBundle {
    fn default() -> Self {
        Self {
            line_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Line2D".to_string()
            }
        }
    }
}

/// Represents a Line2D.
#[derive(Component)]
pub struct GDLine2D {
    pub antialiased: bool,
pub default_color: Color,
pub points: Vec<Vector2>,
pub round_precision: i64,
pub sharp_limit: f64,
pub texture: Option<Ref<Texture>>,
pub width: f64,
}

impl Default for GDLine2D {
    fn default() -> Self {
        Self {
            antialiased: Default::default(),
default_color: Color::from_rgb(0.0, 0.0, 0.0),
points: Default::default(),
round_precision: Default::default(),
sharp_limit: Default::default(),
texture: Default::default(),
width: Default::default(),
        }
    }
}

impl NodeClass for GDLine2D {
    type Parent = GDNode2D;
    type GodotClass = Line2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Line2D>().unwrap();
        world_commands.insert(entity, GDLine2D {
            antialiased: component_ref.antialiased(),
default_color: component_ref.default_color(),
points: component_ref.points().to_vec(),
round_precision: component_ref.round_precision(),
sharp_limit: component_ref.sharp_limit(),
texture: component_ref.texture(),
width: component_ref.width(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDLine2D {
    
}

fn sync_bevy_owned(query: Query<(&GDLine2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Line2D>().unwrap();
        component_ref.set_antialiased(component.antialiased);
component_ref.set_default_color(component.default_color);
component_ref.set_points(Vector2Array::from_vec(component.points.clone()));
component_ref.set_round_precision(component.round_precision);
component_ref.set_sharp_limit(component.sharp_limit);
component_ref.set_texture(component.texture.as_ref().unwrap().clone());
component_ref.set_width(component.width);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDLine2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Line2D>().unwrap();
        component.antialiased = component_ref.antialiased();
component.default_color = component_ref.default_color();
component.points = component_ref.points().to_vec();
component.round_precision = component_ref.round_precision();
component.sharp_limit = component_ref.sharp_limit();
component.texture = component_ref.texture();
component.width = component_ref.width();
    }
}