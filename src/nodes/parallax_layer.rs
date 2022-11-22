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

pub struct ParallaxLayerPlugin;

impl Plugin for ParallaxLayerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ParallaxLayer>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a parallax_layer.
pub fn is_parallax_layer(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ParallaxLayer>().is_some()
}

/// A bundle for ParallaxLayers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDParallaxLayerBundle {
    pub parallax_layer: GDParallaxLayer,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDParallaxLayerBundle {
    fn default() -> Self {
        Self {
            parallax_layer: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ParallaxLayer".to_string()
            }
        }
    }
}

/// Represents a ParallaxLayer.
#[derive(Component)]
pub struct GDParallaxLayer {
    pub motion_mirroring: Vector2,
pub motion_offset: Vector2,
pub motion_scale: Vector2,
}

impl Default for GDParallaxLayer {
    fn default() -> Self {
        Self {
            motion_mirroring: Default::default(),
motion_offset: Default::default(),
motion_scale: Default::default(),
        }
    }
}

impl NodeClass for GDParallaxLayer {
    type Parent = GDNode2D;
    type GodotClass = ParallaxLayer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ParallaxLayer>().unwrap();
        world_commands.insert(entity, GDParallaxLayer {
            motion_mirroring: component_ref.mirroring(),
motion_offset: component_ref.motion_offset(),
motion_scale: component_ref.motion_scale(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDParallaxLayer {
    
}

fn sync_bevy_owned(query: Query<(&GDParallaxLayer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ParallaxLayer>().unwrap();
        component_ref.set_mirroring(component.motion_mirroring);
component_ref.set_motion_offset(component.motion_offset);
component_ref.set_motion_scale(component.motion_scale);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDParallaxLayer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ParallaxLayer>().unwrap();
        component.motion_mirroring = component_ref.mirroring();
component.motion_offset = component_ref.motion_offset();
component.motion_scale = component_ref.motion_scale();
    }
}