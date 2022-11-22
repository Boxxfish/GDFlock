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

pub struct ScrollBarPlugin;

impl Plugin for ScrollBarPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a scroll_bar.
pub fn is_scroll_bar(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ScrollBar>().is_some()
}

/// A bundle for ScrollBars.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDScrollBarBundle {
    pub scroll_bar: GDScrollBar,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub range: GDRange,
    pub true_type: TrueNodeType,
}

impl Default for GDScrollBarBundle {
    fn default() -> Self {
        Self {
            scroll_bar: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
range: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ScrollBar".to_string()
            }
        }
    }
}

/// Represents a ScrollBar.
#[derive(Component)]
pub struct GDScrollBar {
    pub custom_step: f64,
}

impl Default for GDScrollBar {
    fn default() -> Self {
        Self {
            custom_step: Default::default(),
        }
    }
}

impl NodeClass for GDScrollBar {
    type Parent = GDRange;
    type GodotClass = ScrollBar;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ScrollBar>().unwrap();
        world_commands.insert(entity, GDScrollBar {
            custom_step: component_ref.custom_step(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDScrollBar {
    
}

fn sync_bevy_owned(query: Query<(&GDScrollBar, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ScrollBar>().unwrap();
        component_ref.set_custom_step(component.custom_step);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDScrollBar, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ScrollBar>().unwrap();
        component.custom_step = component_ref.custom_step();
    }
}