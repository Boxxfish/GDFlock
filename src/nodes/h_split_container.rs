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

pub struct HSplitContainerPlugin;

impl Plugin for HSplitContainerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<HSplitContainer>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a h_split_container.
pub fn is_h_split_container(node: &gdnative::prelude::Node) -> bool {
    node.cast::<HSplitContainer>().is_some()
}

/// A bundle for HSplitContainers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDHSplitContainerBundle {
    pub h_split_container: GDHSplitContainer,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
pub split_container: GDSplitContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDHSplitContainerBundle {
    fn default() -> Self {
        Self {
            h_split_container: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
split_container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "HSplitContainer".to_string()
            }
        }
    }
}

/// Represents a HSplitContainer.
#[derive(Component)]
pub struct GDHSplitContainer {
    
}

impl Default for GDHSplitContainer {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDHSplitContainer {
    type Parent = GDSplitContainer;
    type GodotClass = HSplitContainer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<HSplitContainer>().unwrap();
        world_commands.insert(entity, GDHSplitContainer {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDHSplitContainer {
    
}

fn sync_bevy_owned(query: Query<(&GDHSplitContainer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<HSplitContainer>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDHSplitContainer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<HSplitContainer>().unwrap();
        
    }
}