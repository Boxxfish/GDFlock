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

pub struct ViewportContainerPlugin;

impl Plugin for ViewportContainerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ViewportContainer>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a viewport_container.
pub fn is_viewport_container(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ViewportContainer>().is_some()
}

/// A bundle for ViewportContainers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDViewportContainerBundle {
    pub viewport_container: GDViewportContainer,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDViewportContainerBundle {
    fn default() -> Self {
        Self {
            viewport_container: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ViewportContainer".to_string()
            }
        }
    }
}

/// Represents a ViewportContainer.
#[derive(Component)]
pub struct GDViewportContainer {
    pub stretch: bool,
pub stretch_shrink: i64,
}

impl Default for GDViewportContainer {
    fn default() -> Self {
        Self {
            stretch: Default::default(),
stretch_shrink: Default::default(),
        }
    }
}

impl NodeClass for GDViewportContainer {
    type Parent = GDContainer;
    type GodotClass = ViewportContainer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ViewportContainer>().unwrap();
        world_commands.insert(entity, GDViewportContainer {
            stretch: component_ref.is_stretch_enabled(),
stretch_shrink: component_ref.stretch_shrink(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDViewportContainer {
    
}

fn sync_bevy_owned(query: Query<(&GDViewportContainer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ViewportContainer>().unwrap();
        component_ref.set_stretch(component.stretch);
component_ref.set_stretch_shrink(component.stretch_shrink);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDViewportContainer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ViewportContainer>().unwrap();
        component.stretch = component_ref.is_stretch_enabled();
component.stretch_shrink = component_ref.stretch_shrink();
    }
}