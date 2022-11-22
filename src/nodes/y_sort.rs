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

pub struct YSortPlugin;

impl Plugin for YSortPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<YSort>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a y_sort.
pub fn is_y_sort(node: &gdnative::prelude::Node) -> bool {
    node.cast::<YSort>().is_some()
}

/// A bundle for YSorts.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDYSortBundle {
    pub y_sort: GDYSort,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDYSortBundle {
    fn default() -> Self {
        Self {
            y_sort: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "YSort".to_string()
            }
        }
    }
}

/// Represents a YSort.
#[derive(Component)]
pub struct GDYSort {
    pub sort_enabled: bool,
}

impl Default for GDYSort {
    fn default() -> Self {
        Self {
            sort_enabled: Default::default(),
        }
    }
}

impl NodeClass for GDYSort {
    type Parent = GDNode2D;
    type GodotClass = YSort;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<YSort>().unwrap();
        world_commands.insert(entity, GDYSort {
            sort_enabled: component_ref.is_sort_enabled(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDYSort {
    
}

fn sync_bevy_owned(query: Query<(&GDYSort, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<YSort>().unwrap();
        component_ref.set_sort_enabled(component.sort_enabled);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDYSort, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<YSort>().unwrap();
        component.sort_enabled = component_ref.is_sort_enabled();
    }
}