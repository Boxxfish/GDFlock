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

pub struct SeparatorPlugin;

impl Plugin for SeparatorPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a separator.
pub fn is_separator(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Separator>().is_some()
}

/// A bundle for Separators.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDSeparatorBundle {
    pub separator: GDSeparator,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
    pub true_type: TrueNodeType,
}

impl Default for GDSeparatorBundle {
    fn default() -> Self {
        Self {
            separator: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Separator".to_string()
            }
        }
    }
}

/// Represents a Separator.
#[derive(Component)]
pub struct GDSeparator {
    
}

impl Default for GDSeparator {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDSeparator {
    type Parent = GDControl;
    type GodotClass = Separator;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Separator>().unwrap();
        world_commands.insert(entity, GDSeparator {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDSeparator {
    
}

fn sync_bevy_owned(query: Query<(&GDSeparator, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Separator>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDSeparator, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Separator>().unwrap();
        
    }
}