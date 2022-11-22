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

pub struct TabContainerPlugin;

impl Plugin for TabContainerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<TabContainer>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a tab_container.
pub fn is_tab_container(node: &gdnative::prelude::Node) -> bool {
    node.cast::<TabContainer>().is_some()
}

/// A bundle for TabContainers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDTabContainerBundle {
    pub tab_container: GDTabContainer,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDTabContainerBundle {
    fn default() -> Self {
        Self {
            tab_container: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "TabContainer".to_string()
            }
        }
    }
}

/// Represents a TabContainer.
#[derive(Component)]
pub struct GDTabContainer {
    pub all_tabs_in_front: bool,
pub current_tab: i64,
pub drag_to_rearrange_enabled: bool,
pub tabs_visible: bool,
pub use_hidden_tabs_for_min_size: bool,
}

impl Default for GDTabContainer {
    fn default() -> Self {
        Self {
            all_tabs_in_front: Default::default(),
current_tab: Default::default(),
drag_to_rearrange_enabled: Default::default(),
tabs_visible: Default::default(),
use_hidden_tabs_for_min_size: Default::default(),
        }
    }
}

impl NodeClass for GDTabContainer {
    type Parent = GDContainer;
    type GodotClass = TabContainer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<TabContainer>().unwrap();
        world_commands.insert(entity, GDTabContainer {
            all_tabs_in_front: component_ref.is_all_tabs_in_front(),
current_tab: component_ref.current_tab(),
drag_to_rearrange_enabled: component_ref.drag_to_rearrange_enabled(),
tabs_visible: component_ref.are_tabs_visible(),
use_hidden_tabs_for_min_size: component_ref.use_hidden_tabs_for_min_size(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDTabContainer {
    
}

fn sync_bevy_owned(query: Query<(&GDTabContainer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<TabContainer>().unwrap();
        component_ref.set_all_tabs_in_front(component.all_tabs_in_front);
component_ref.set_current_tab(component.current_tab);
component_ref.set_drag_to_rearrange_enabled(component.drag_to_rearrange_enabled);
component_ref.set_tabs_visible(component.tabs_visible);
component_ref.set_use_hidden_tabs_for_min_size(component.use_hidden_tabs_for_min_size);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDTabContainer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<TabContainer>().unwrap();
        component.all_tabs_in_front = component_ref.is_all_tabs_in_front();
component.current_tab = component_ref.current_tab();
component.drag_to_rearrange_enabled = component_ref.drag_to_rearrange_enabled();
component.tabs_visible = component_ref.are_tabs_visible();
component.use_hidden_tabs_for_min_size = component_ref.use_hidden_tabs_for_min_size();
    }
}