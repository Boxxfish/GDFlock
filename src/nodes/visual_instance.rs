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

pub struct VisualInstancePlugin;

impl Plugin for VisualInstancePlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a visual_instance.
pub fn is_visual_instance(node: &gdnative::prelude::Node) -> bool {
    node.cast::<VisualInstance>().is_some()
}

/// A bundle for VisualInstances.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDVisualInstanceBundle {
    pub visual_instance: GDVisualInstance,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
    pub true_type: TrueNodeType,
}

impl Default for GDVisualInstanceBundle {
    fn default() -> Self {
        Self {
            visual_instance: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "VisualInstance".to_string()
            }
        }
    }
}

/// Represents a VisualInstance.
#[derive(Component)]
pub struct GDVisualInstance {
    pub layers: i64,
}

impl Default for GDVisualInstance {
    fn default() -> Self {
        Self {
            layers: Default::default(),
        }
    }
}

impl NodeClass for GDVisualInstance {
    type Parent = GDCullInstance;
    type GodotClass = VisualInstance;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<VisualInstance>().unwrap();
        world_commands.insert(entity, GDVisualInstance {
            layers: component_ref.layer_mask(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDVisualInstance {
    
}

fn sync_bevy_owned(query: Query<(&GDVisualInstance, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VisualInstance>().unwrap();
        component_ref.set_layer_mask(component.layers);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDVisualInstance, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VisualInstance>().unwrap();
        component.layers = component_ref.layer_mask();
    }
}