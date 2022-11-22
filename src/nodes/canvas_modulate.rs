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

pub struct CanvasModulatePlugin;

impl Plugin for CanvasModulatePlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<CanvasModulate>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a canvas_modulate.
pub fn is_canvas_modulate(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CanvasModulate>().is_some()
}

/// A bundle for CanvasModulates.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCanvasModulateBundle {
    pub canvas_modulate: GDCanvasModulate,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDCanvasModulateBundle {
    fn default() -> Self {
        Self {
            canvas_modulate: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CanvasModulate".to_string()
            }
        }
    }
}

/// Represents a CanvasModulate.
#[derive(Component)]
pub struct GDCanvasModulate {
    pub color: Color,
}

impl Default for GDCanvasModulate {
    fn default() -> Self {
        Self {
            color: Color::from_rgb(0.0, 0.0, 0.0),
        }
    }
}

impl NodeClass for GDCanvasModulate {
    type Parent = GDNode2D;
    type GodotClass = CanvasModulate;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CanvasModulate>().unwrap();
        world_commands.insert(entity, GDCanvasModulate {
            color: component_ref.color(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCanvasModulate {
    
}

fn sync_bevy_owned(query: Query<(&GDCanvasModulate, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CanvasModulate>().unwrap();
        component_ref.set_color(component.color);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCanvasModulate, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CanvasModulate>().unwrap();
        component.color = component_ref.color();
    }
}