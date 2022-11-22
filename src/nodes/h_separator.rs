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

pub struct HSeparatorPlugin;

impl Plugin for HSeparatorPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<HSeparator>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a h_separator.
pub fn is_h_separator(node: &gdnative::prelude::Node) -> bool {
    node.cast::<HSeparator>().is_some()
}

/// A bundle for HSeparators.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDHSeparatorBundle {
    pub h_separator: GDHSeparator,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub separator: GDSeparator,
    pub true_type: TrueNodeType,
}

impl Default for GDHSeparatorBundle {
    fn default() -> Self {
        Self {
            h_separator: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
separator: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "HSeparator".to_string()
            }
        }
    }
}

/// Represents a HSeparator.
#[derive(Component)]
pub struct GDHSeparator {
    
}

impl Default for GDHSeparator {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDHSeparator {
    type Parent = GDSeparator;
    type GodotClass = HSeparator;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<HSeparator>().unwrap();
        world_commands.insert(entity, GDHSeparator {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDHSeparator {
    
}

fn sync_bevy_owned(query: Query<(&GDHSeparator, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<HSeparator>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDHSeparator, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<HSeparator>().unwrap();
        
    }
}