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

pub struct CenterContainerPlugin;

impl Plugin for CenterContainerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<CenterContainer>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a center_container.
pub fn is_center_container(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CenterContainer>().is_some()
}

/// A bundle for CenterContainers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCenterContainerBundle {
    pub center_container: GDCenterContainer,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDCenterContainerBundle {
    fn default() -> Self {
        Self {
            center_container: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CenterContainer".to_string()
            }
        }
    }
}

/// Represents a CenterContainer.
#[derive(Component)]
pub struct GDCenterContainer {
    pub use_top_left: bool,
}

impl Default for GDCenterContainer {
    fn default() -> Self {
        Self {
            use_top_left: Default::default(),
        }
    }
}

impl NodeClass for GDCenterContainer {
    type Parent = GDContainer;
    type GodotClass = CenterContainer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CenterContainer>().unwrap();
        world_commands.insert(entity, GDCenterContainer {
            use_top_left: component_ref.is_using_top_left(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCenterContainer {
    
}

fn sync_bevy_owned(query: Query<(&GDCenterContainer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CenterContainer>().unwrap();
        component_ref.set_use_top_left(component.use_top_left);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCenterContainer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CenterContainer>().unwrap();
        component.use_top_left = component_ref.is_using_top_left();
    }
}