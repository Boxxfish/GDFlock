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

pub struct ParallaxBackgroundPlugin;

impl Plugin for ParallaxBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ParallaxBackground>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a parallax_background.
pub fn is_parallax_background(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ParallaxBackground>().is_some()
}

/// A bundle for ParallaxBackgrounds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDParallaxBackgroundBundle {
    pub parallax_background: GDParallaxBackground,
    pub node: GDNode,
pub canvas_layer: GDCanvasLayer,
    pub true_type: TrueNodeType,
}

impl Default for GDParallaxBackgroundBundle {
    fn default() -> Self {
        Self {
            parallax_background: Default::default(),
            node: Default::default(),
canvas_layer: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ParallaxBackground".to_string()
            }
        }
    }
}

/// Represents a ParallaxBackground.
#[derive(Component)]
pub struct GDParallaxBackground {
    pub scroll_base_offset: Vector2,
pub scroll_base_scale: Vector2,
pub scroll_ignore_camera_zoom: bool,
pub scroll_limit_begin: Vector2,
pub scroll_limit_end: Vector2,
pub scroll_offset: Vector2,
}

impl Default for GDParallaxBackground {
    fn default() -> Self {
        Self {
            scroll_base_offset: Default::default(),
scroll_base_scale: Default::default(),
scroll_ignore_camera_zoom: Default::default(),
scroll_limit_begin: Default::default(),
scroll_limit_end: Default::default(),
scroll_offset: Default::default(),
        }
    }
}

impl NodeClass for GDParallaxBackground {
    type Parent = GDCanvasLayer;
    type GodotClass = ParallaxBackground;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ParallaxBackground>().unwrap();
        world_commands.insert(entity, GDParallaxBackground {
            scroll_base_offset: component_ref.scroll_base_offset(),
scroll_base_scale: component_ref.scroll_base_scale(),
scroll_ignore_camera_zoom: component_ref.is_ignore_camera_zoom(),
scroll_limit_begin: component_ref.limit_begin(),
scroll_limit_end: component_ref.limit_end(),
scroll_offset: component_ref.scroll_offset(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDParallaxBackground {
    
}

fn sync_bevy_owned(query: Query<(&GDParallaxBackground, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ParallaxBackground>().unwrap();
        component_ref.set_scroll_base_offset(component.scroll_base_offset);
component_ref.set_scroll_base_scale(component.scroll_base_scale);
component_ref.set_ignore_camera_zoom(component.scroll_ignore_camera_zoom);
component_ref.set_limit_begin(component.scroll_limit_begin);
component_ref.set_limit_end(component.scroll_limit_end);
component_ref.set_scroll_offset(component.scroll_offset);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDParallaxBackground, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ParallaxBackground>().unwrap();
        component.scroll_base_offset = component_ref.scroll_base_offset();
component.scroll_base_scale = component_ref.scroll_base_scale();
component.scroll_ignore_camera_zoom = component_ref.is_ignore_camera_zoom();
component.scroll_limit_begin = component_ref.limit_begin();
component.scroll_limit_end = component_ref.limit_end();
component.scroll_offset = component_ref.scroll_offset();
    }
}