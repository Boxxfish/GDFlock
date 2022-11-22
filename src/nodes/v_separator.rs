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

pub struct VSeparatorPlugin;

impl Plugin for VSeparatorPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<VSeparator>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a v_separator.
pub fn is_v_separator(node: &gdnative::prelude::Node) -> bool {
    node.cast::<VSeparator>().is_some()
}

/// A bundle for VSeparators.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDVSeparatorBundle {
    pub v_separator: GDVSeparator,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub separator: GDSeparator,
    pub true_type: TrueNodeType,
}

impl Default for GDVSeparatorBundle {
    fn default() -> Self {
        Self {
            v_separator: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
separator: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "VSeparator".to_string()
            }
        }
    }
}

/// Represents a VSeparator.
#[derive(Component)]
pub struct GDVSeparator {
    
}

impl Default for GDVSeparator {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDVSeparator {
    type Parent = GDSeparator;
    type GodotClass = VSeparator;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<VSeparator>().unwrap();
        world_commands.insert(entity, GDVSeparator {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDVSeparator {
    
}

fn sync_bevy_owned(query: Query<(&GDVSeparator, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VSeparator>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDVSeparator, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VSeparator>().unwrap();
        
    }
}