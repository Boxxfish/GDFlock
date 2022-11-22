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

pub struct SplitContainerPlugin;

impl Plugin for SplitContainerPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a split_container.
pub fn is_split_container(node: &gdnative::prelude::Node) -> bool {
    node.cast::<SplitContainer>().is_some()
}

/// A bundle for SplitContainers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDSplitContainerBundle {
    pub split_container: GDSplitContainer,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDSplitContainerBundle {
    fn default() -> Self {
        Self {
            split_container: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "SplitContainer".to_string()
            }
        }
    }
}

/// Represents a SplitContainer.
#[derive(Component)]
pub struct GDSplitContainer {
    pub collapsed: bool,
pub split_offset: i64,
}

impl Default for GDSplitContainer {
    fn default() -> Self {
        Self {
            collapsed: Default::default(),
split_offset: Default::default(),
        }
    }
}

impl NodeClass for GDSplitContainer {
    type Parent = GDContainer;
    type GodotClass = SplitContainer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<SplitContainer>().unwrap();
        world_commands.insert(entity, GDSplitContainer {
            collapsed: component_ref.is_collapsed(),
split_offset: component_ref.split_offset(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDSplitContainer {
    
}

fn sync_bevy_owned(query: Query<(&GDSplitContainer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<SplitContainer>().unwrap();
        component_ref.set_collapsed(component.collapsed);
component_ref.set_split_offset(component.split_offset);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDSplitContainer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<SplitContainer>().unwrap();
        component.collapsed = component_ref.is_collapsed();
component.split_offset = component_ref.split_offset();
    }
}