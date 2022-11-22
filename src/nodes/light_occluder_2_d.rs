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

pub struct LightOccluder2DPlugin;

impl Plugin for LightOccluder2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<LightOccluder2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a light_occluder_2_d.
pub fn is_light_occluder_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<LightOccluder2D>().is_some()
}

/// A bundle for LightOccluder2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDLightOccluder2DBundle {
    pub light_occluder_2_d: GDLightOccluder2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDLightOccluder2DBundle {
    fn default() -> Self {
        Self {
            light_occluder_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "LightOccluder2D".to_string()
            }
        }
    }
}

/// Represents a LightOccluder2D.
#[derive(Component)]
pub struct GDLightOccluder2D {
    pub light_mask: i64,
}

impl Default for GDLightOccluder2D {
    fn default() -> Self {
        Self {
            light_mask: Default::default(),
        }
    }
}

impl NodeClass for GDLightOccluder2D {
    type Parent = GDNode2D;
    type GodotClass = LightOccluder2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<LightOccluder2D>().unwrap();
        world_commands.insert(entity, GDLightOccluder2D {
            light_mask: component_ref.occluder_light_mask(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDLightOccluder2D {
    
}

fn sync_bevy_owned(query: Query<(&GDLightOccluder2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<LightOccluder2D>().unwrap();
        component_ref.set_occluder_light_mask(component.light_mask);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDLightOccluder2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<LightOccluder2D>().unwrap();
        component.light_mask = component_ref.occluder_light_mask();
    }
}