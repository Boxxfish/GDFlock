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

pub struct ItemListPlugin;

impl Plugin for ItemListPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ItemList>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a item_list.
pub fn is_item_list(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ItemList>().is_some()
}

/// A bundle for ItemLists.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDItemListBundle {
    pub item_list: GDItemList,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
    pub true_type: TrueNodeType,
}

impl Default for GDItemListBundle {
    fn default() -> Self {
        Self {
            item_list: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ItemList".to_string()
            }
        }
    }
}

/// Represents a ItemList.
#[derive(Component)]
pub struct GDItemList {
    pub allow_reselect: bool,
pub allow_rmb_select: bool,
pub auto_height: bool,
pub fixed_column_width: i64,
pub fixed_icon_size: Vector2,
pub icon_scale: f64,
pub max_columns: i64,
pub max_text_lines: i64,
pub same_column_width: bool,
}

impl Default for GDItemList {
    fn default() -> Self {
        Self {
            allow_reselect: Default::default(),
allow_rmb_select: Default::default(),
auto_height: Default::default(),
fixed_column_width: Default::default(),
fixed_icon_size: Default::default(),
icon_scale: Default::default(),
max_columns: Default::default(),
max_text_lines: Default::default(),
same_column_width: Default::default(),
        }
    }
}

impl NodeClass for GDItemList {
    type Parent = GDControl;
    type GodotClass = ItemList;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ItemList>().unwrap();
        world_commands.insert(entity, GDItemList {
            allow_reselect: component_ref.allow_reselect(),
allow_rmb_select: component_ref.allow_rmb_select(),
auto_height: component_ref.has_auto_height(),
fixed_column_width: component_ref.fixed_column_width(),
fixed_icon_size: component_ref.fixed_icon_size(),
icon_scale: component_ref.icon_scale(),
max_columns: component_ref.max_columns(),
max_text_lines: component_ref.max_text_lines(),
same_column_width: component_ref.is_same_column_width(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDItemList {
    
}

fn sync_bevy_owned(query: Query<(&GDItemList, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ItemList>().unwrap();
        component_ref.set_allow_reselect(component.allow_reselect);
component_ref.set_allow_rmb_select(component.allow_rmb_select);
component_ref.set_auto_height(component.auto_height);
component_ref.set_fixed_column_width(component.fixed_column_width);
component_ref.set_fixed_icon_size(component.fixed_icon_size);
component_ref.set_icon_scale(component.icon_scale);
component_ref.set_max_columns(component.max_columns);
component_ref.set_max_text_lines(component.max_text_lines);
component_ref.set_same_column_width(component.same_column_width);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDItemList, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ItemList>().unwrap();
        component.allow_reselect = component_ref.allow_reselect();
component.allow_rmb_select = component_ref.allow_rmb_select();
component.auto_height = component_ref.has_auto_height();
component.fixed_column_width = component_ref.fixed_column_width();
component.fixed_icon_size = component_ref.fixed_icon_size();
component.icon_scale = component_ref.icon_scale();
component.max_columns = component_ref.max_columns();
component.max_text_lines = component_ref.max_text_lines();
component.same_column_width = component_ref.is_same_column_width();
    }
}