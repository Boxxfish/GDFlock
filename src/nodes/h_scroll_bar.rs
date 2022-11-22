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

pub struct HScrollBarPlugin;

impl Plugin for HScrollBarPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<HScrollBar>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a h_scroll_bar.
pub fn is_h_scroll_bar(node: &gdnative::prelude::Node) -> bool {
    node.cast::<HScrollBar>().is_some()
}

/// A bundle for HScrollBars.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDHScrollBarBundle {
    pub h_scroll_bar: GDHScrollBar,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub range: GDRange,
pub scroll_bar: GDScrollBar,
    pub true_type: TrueNodeType,
}

impl Default for GDHScrollBarBundle {
    fn default() -> Self {
        Self {
            h_scroll_bar: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
range: Default::default(),
scroll_bar: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "HScrollBar".to_string()
            }
        }
    }
}

/// Represents a HScrollBar.
#[derive(Component)]
pub struct GDHScrollBar {
    
}

impl Default for GDHScrollBar {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDHScrollBar {
    type Parent = GDScrollBar;
    type GodotClass = HScrollBar;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<HScrollBar>().unwrap();
        world_commands.insert(entity, GDHScrollBar {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDHScrollBar {
    
}

fn sync_bevy_owned(query: Query<(&GDHScrollBar, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<HScrollBar>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDHScrollBar, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<HScrollBar>().unwrap();
        
    }
}