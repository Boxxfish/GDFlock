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

pub struct RangePlugin;

impl Plugin for RangePlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a range.
pub fn is_range(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Range>().is_some()
}

/// A bundle for Ranges.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDRangeBundle {
    pub range: GDRange,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
    pub true_type: TrueNodeType,
}

impl Default for GDRangeBundle {
    fn default() -> Self {
        Self {
            range: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Range".to_string()
            }
        }
    }
}

/// Represents a Range.
#[derive(Component)]
pub struct GDRange {
    pub allow_greater: bool,
pub allow_lesser: bool,
pub exp_edit: bool,
pub max_value: f64,
pub min_value: f64,
pub page: f64,
pub ratio: f64,
pub rounded: bool,
pub step: f64,
pub value: f64,
}

impl Default for GDRange {
    fn default() -> Self {
        Self {
            allow_greater: Default::default(),
allow_lesser: Default::default(),
exp_edit: Default::default(),
max_value: Default::default(),
min_value: Default::default(),
page: Default::default(),
ratio: Default::default(),
rounded: Default::default(),
step: Default::default(),
value: Default::default(),
        }
    }
}

impl NodeClass for GDRange {
    type Parent = GDControl;
    type GodotClass = Range;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Range>().unwrap();
        world_commands.insert(entity, GDRange {
            allow_greater: component_ref.is_greater_allowed(),
allow_lesser: component_ref.is_lesser_allowed(),
exp_edit: component_ref.is_ratio_exp(),
max_value: component_ref.max(),
min_value: component_ref.min(),
page: component_ref.page(),
ratio: component_ref.as_ratio(),
rounded: component_ref.is_using_rounded_values(),
step: component_ref.step(),
value: component_ref.value(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDRange {
    
}

fn sync_bevy_owned(query: Query<(&GDRange, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Range>().unwrap();
        component_ref.set_allow_greater(component.allow_greater);
component_ref.set_allow_lesser(component.allow_lesser);
component_ref.set_exp_ratio(component.exp_edit);
component_ref.set_max(component.max_value);
component_ref.set_min(component.min_value);
component_ref.set_page(component.page);
component_ref.set_as_ratio(component.ratio);
component_ref.set_use_rounded_values(component.rounded);
component_ref.set_step(component.step);
component_ref.set_value(component.value);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDRange, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Range>().unwrap();
        component.allow_greater = component_ref.is_greater_allowed();
component.allow_lesser = component_ref.is_lesser_allowed();
component.exp_edit = component_ref.is_ratio_exp();
component.max_value = component_ref.max();
component.min_value = component_ref.min();
component.page = component_ref.page();
component.ratio = component_ref.as_ratio();
component.rounded = component_ref.is_using_rounded_values();
component.step = component_ref.step();
component.value = component_ref.value();
    }
}