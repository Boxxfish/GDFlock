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

pub struct ScrollContainerPlugin;

impl Plugin for ScrollContainerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ScrollContainer>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a scroll_container.
pub fn is_scroll_container(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ScrollContainer>().is_some()
}

/// A bundle for ScrollContainers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDScrollContainerBundle {
    pub scroll_container: GDScrollContainer,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub container: GDContainer,
    pub true_type: TrueNodeType,
}

impl Default for GDScrollContainerBundle {
    fn default() -> Self {
        Self {
            scroll_container: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
container: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ScrollContainer".to_string()
            }
        }
    }
}

/// Represents a ScrollContainer.
#[derive(Component)]
pub struct GDScrollContainer {
    pub follow_focus: bool,
pub scroll_deadzone: i64,
pub scroll_horizontal: i64,
pub scroll_horizontal_enabled: bool,
pub scroll_vertical: i64,
pub scroll_vertical_enabled: bool,
}

impl Default for GDScrollContainer {
    fn default() -> Self {
        Self {
            follow_focus: Default::default(),
scroll_deadzone: Default::default(),
scroll_horizontal: Default::default(),
scroll_horizontal_enabled: Default::default(),
scroll_vertical: Default::default(),
scroll_vertical_enabled: Default::default(),
        }
    }
}

impl NodeClass for GDScrollContainer {
    type Parent = GDContainer;
    type GodotClass = ScrollContainer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ScrollContainer>().unwrap();
        world_commands.insert(entity, GDScrollContainer {
            follow_focus: component_ref.is_following_focus(),
scroll_deadzone: component_ref.deadzone(),
scroll_horizontal: component_ref.h_scroll(),
scroll_horizontal_enabled: component_ref.is_h_scroll_enabled(),
scroll_vertical: component_ref.v_scroll(),
scroll_vertical_enabled: component_ref.is_v_scroll_enabled(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDScrollContainer {
    
}

fn sync_bevy_owned(query: Query<(&GDScrollContainer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ScrollContainer>().unwrap();
        component_ref.set_follow_focus(component.follow_focus);
component_ref.set_deadzone(component.scroll_deadzone);
component_ref.set_h_scroll(component.scroll_horizontal);
component_ref.set_enable_h_scroll(component.scroll_horizontal_enabled);
component_ref.set_v_scroll(component.scroll_vertical);
component_ref.set_enable_v_scroll(component.scroll_vertical_enabled);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDScrollContainer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ScrollContainer>().unwrap();
        component.follow_focus = component_ref.is_following_focus();
component.scroll_deadzone = component_ref.deadzone();
component.scroll_horizontal = component_ref.h_scroll();
component.scroll_horizontal_enabled = component_ref.is_h_scroll_enabled();
component.scroll_vertical = component_ref.v_scroll();
component.scroll_vertical_enabled = component_ref.is_v_scroll_enabled();
    }
}