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

pub struct GraphNodePlugin;

impl Plugin for GraphNodePlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<GraphNode>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a graph_node.
pub fn is_graph_node(node: &gdnative::prelude::Node) -> bool {
    node.cast::<GraphNode>().is_some()
}

/// A bundle for GraphNodes.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDGraphNodeBundle {
    pub graph_node: GDGraphNode,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDGraphNodeBundle {
    fn default() -> Self {
        Self {
            graph_node: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "GraphNode".to_string()
            }
        }
    }
}

/// Represents a GraphNode.
#[derive(Component)]
pub struct GDGraphNode {
    pub comment: bool,
pub offset: Vector2,
pub resizable: bool,
pub selected: bool,
pub show_close: bool,
pub title: String,
}

impl Default for GDGraphNode {
    fn default() -> Self {
        Self {
            comment: Default::default(),
offset: Default::default(),
resizable: Default::default(),
selected: Default::default(),
show_close: Default::default(),
title: Default::default(),
        }
    }
}

impl NodeClass for GDGraphNode {
    type Parent = GDContainer;
    type GodotClass = GraphNode;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<GraphNode>().unwrap();
        world_commands.insert(entity, GDGraphNode {
            comment: component_ref.is_comment(),
offset: component_ref.offset(),
resizable: component_ref.is_resizable(),
selected: component_ref.is_selected(),
show_close: component_ref.is_close_button_visible(),
title: component_ref.title().to_string(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDGraphNode {
    
}

fn sync_bevy_owned(query: Query<(&GDGraphNode, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<GraphNode>().unwrap();
        component_ref.set_comment(component.comment);
component_ref.set_offset(component.offset);
component_ref.set_resizable(component.resizable);
component_ref.set_selected(component.selected);
component_ref.set_show_close_button(component.show_close);
component_ref.set_title(component.title.clone());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDGraphNode, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<GraphNode>().unwrap();
        component.comment = component_ref.is_comment();
component.offset = component_ref.offset();
component.resizable = component_ref.is_resizable();
component.selected = component_ref.is_selected();
component.show_close = component_ref.is_close_button_visible();
component.title = component_ref.title().to_string();
    }
}