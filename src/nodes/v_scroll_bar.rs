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

pub struct VScrollBarPlugin;

impl Plugin for VScrollBarPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<VScrollBar>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a v_scroll_bar.
pub fn is_v_scroll_bar(node: &gdnative::prelude::Node) -> bool {
    node.cast::<VScrollBar>().is_some()
}

/// A bundle for VScrollBars.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDVScrollBarBundle {
    pub v_scroll_bar: GDVScrollBar,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub range: GDRange,
pub scroll_bar: GDScrollBar,
    pub true_type: TrueNodeType,
}

impl Default for GDVScrollBarBundle {
    fn default() -> Self {
        Self {
            v_scroll_bar: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
range: Default::default(),
scroll_bar: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "VScrollBar".to_string()
            }
        }
    }
}

/// Represents a VScrollBar.
#[derive(Component)]
pub struct GDVScrollBar {
    
}

impl Default for GDVScrollBar {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDVScrollBar {
    type Parent = GDScrollBar;
    type GodotClass = VScrollBar;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<VScrollBar>().unwrap();
        world_commands.insert(entity, GDVScrollBar {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDVScrollBar {
    
}

fn sync_bevy_owned(query: Query<(&GDVScrollBar, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VScrollBar>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDVScrollBar, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VScrollBar>().unwrap();
        
    }
}