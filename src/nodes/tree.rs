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

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Tree>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a tree.
pub fn is_tree(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Tree>().is_some()
}

/// A bundle for Trees.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDTreeBundle {
    pub tree: GDTree,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
    pub true_type: TrueNodeType,
}

impl Default for GDTreeBundle {
    fn default() -> Self {
        Self {
            tree: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Tree".to_string()
            }
        }
    }
}

/// Represents a Tree.
#[derive(Component)]
pub struct GDTree {
    pub allow_reselect: bool,
pub allow_rmb_select: bool,
pub column_titles_visible: bool,
pub columns: i64,
pub drop_mode_flags: i64,
pub hide_folding: bool,
pub hide_root: bool,
}

impl Default for GDTree {
    fn default() -> Self {
        Self {
            allow_reselect: Default::default(),
allow_rmb_select: Default::default(),
column_titles_visible: Default::default(),
columns: Default::default(),
drop_mode_flags: Default::default(),
hide_folding: Default::default(),
hide_root: Default::default(),
        }
    }
}

impl NodeClass for GDTree {
    type Parent = GDControl;
    type GodotClass = Tree;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Tree>().unwrap();
        world_commands.insert(entity, GDTree {
            allow_reselect: component_ref.allow_reselect(),
allow_rmb_select: component_ref.allow_rmb_select(),
column_titles_visible: component_ref.are_column_titles_visible(),
columns: component_ref.columns(),
drop_mode_flags: component_ref.drop_mode_flags(),
hide_folding: component_ref.is_folding_hidden(),
hide_root: component_ref.is_root_hidden(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDTree {
    
}

fn sync_bevy_owned(query: Query<(&GDTree, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Tree>().unwrap();
        component_ref.set_allow_reselect(component.allow_reselect);
component_ref.set_allow_rmb_select(component.allow_rmb_select);
component_ref.set_column_titles_visible(component.column_titles_visible);
component_ref.set_columns(component.columns);
component_ref.set_drop_mode_flags(component.drop_mode_flags);
component_ref.set_hide_folding(component.hide_folding);
component_ref.set_hide_root(component.hide_root);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDTree, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Tree>().unwrap();
        component.allow_reselect = component_ref.allow_reselect();
component.allow_rmb_select = component_ref.allow_rmb_select();
component.column_titles_visible = component_ref.are_column_titles_visible();
component.columns = component_ref.columns();
component.drop_mode_flags = component_ref.drop_mode_flags();
component.hide_folding = component_ref.is_folding_hidden();
component.hide_root = component_ref.is_root_hidden();
    }
}