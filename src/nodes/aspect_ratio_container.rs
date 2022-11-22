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

pub struct AspectRatioContainerPlugin;

impl Plugin for AspectRatioContainerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<AspectRatioContainer>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a aspect_ratio_container.
pub fn is_aspect_ratio_container(node: &gdnative::prelude::Node) -> bool {
    node.cast::<AspectRatioContainer>().is_some()
}

/// A bundle for AspectRatioContainers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDAspectRatioContainerBundle {
    pub aspect_ratio_container: GDAspectRatioContainer,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDAspectRatioContainerBundle {
    fn default() -> Self {
        Self {
            aspect_ratio_container: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "AspectRatioContainer".to_string()
            }
        }
    }
}

/// Represents a AspectRatioContainer.
#[derive(Component)]
pub struct GDAspectRatioContainer {
    pub ratio: f64,
}

impl Default for GDAspectRatioContainer {
    fn default() -> Self {
        Self {
            ratio: Default::default(),
        }
    }
}

impl NodeClass for GDAspectRatioContainer {
    type Parent = GDContainer;
    type GodotClass = AspectRatioContainer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<AspectRatioContainer>().unwrap();
        world_commands.insert(entity, GDAspectRatioContainer {
            ratio: component_ref.ratio(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDAspectRatioContainer {
    
}

fn sync_bevy_owned(query: Query<(&GDAspectRatioContainer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AspectRatioContainer>().unwrap();
        component_ref.set_ratio(component.ratio);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDAspectRatioContainer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AspectRatioContainer>().unwrap();
        component.ratio = component_ref.ratio();
    }
}