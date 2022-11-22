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

pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ProgressBar>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a progress_bar.
pub fn is_progress_bar(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ProgressBar>().is_some()
}

/// A bundle for ProgressBars.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDProgressBarBundle {
    pub progress_bar: GDProgressBar,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub range: GDRange,
    pub true_type: TrueNodeType,
}

impl Default for GDProgressBarBundle {
    fn default() -> Self {
        Self {
            progress_bar: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
range: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ProgressBar".to_string()
            }
        }
    }
}

/// Represents a ProgressBar.
#[derive(Component)]
pub struct GDProgressBar {
    pub percent_visible: bool,
}

impl Default for GDProgressBar {
    fn default() -> Self {
        Self {
            percent_visible: Default::default(),
        }
    }
}

impl NodeClass for GDProgressBar {
    type Parent = GDRange;
    type GodotClass = ProgressBar;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ProgressBar>().unwrap();
        world_commands.insert(entity, GDProgressBar {
            percent_visible: component_ref.is_percent_visible(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDProgressBar {
    
}

fn sync_bevy_owned(query: Query<(&GDProgressBar, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ProgressBar>().unwrap();
        component_ref.set_percent_visible(component.percent_visible);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDProgressBar, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ProgressBar>().unwrap();
        component.percent_visible = component_ref.is_percent_visible();
    }
}