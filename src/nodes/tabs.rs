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

pub struct TabsPlugin;

impl Plugin for TabsPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Tabs>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a tabs.
pub fn is_tabs(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Tabs>().is_some()
}

/// A bundle for Tabss.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDTabsBundle {
    pub tabs: GDTabs,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
    pub true_type: TrueNodeType,
}

impl Default for GDTabsBundle {
    fn default() -> Self {
        Self {
            tabs: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Tabs".to_string()
            }
        }
    }
}

/// Represents a Tabs.
#[derive(Component)]
pub struct GDTabs {
    pub current_tab: i64,
pub drag_to_rearrange_enabled: bool,
pub scrolling_enabled: bool,
}

impl Default for GDTabs {
    fn default() -> Self {
        Self {
            current_tab: Default::default(),
drag_to_rearrange_enabled: Default::default(),
scrolling_enabled: Default::default(),
        }
    }
}

impl NodeClass for GDTabs {
    type Parent = GDControl;
    type GodotClass = Tabs;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Tabs>().unwrap();
        world_commands.insert(entity, GDTabs {
            current_tab: component_ref.current_tab(),
drag_to_rearrange_enabled: component_ref.drag_to_rearrange_enabled(),
scrolling_enabled: component_ref.scrolling_enabled(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDTabs {
    
}

fn sync_bevy_owned(query: Query<(&GDTabs, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Tabs>().unwrap();
        component_ref.set_current_tab(component.current_tab);
component_ref.set_drag_to_rearrange_enabled(component.drag_to_rearrange_enabled);
component_ref.set_scrolling_enabled(component.scrolling_enabled);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDTabs, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Tabs>().unwrap();
        component.current_tab = component_ref.current_tab();
component.drag_to_rearrange_enabled = component_ref.drag_to_rearrange_enabled();
component.scrolling_enabled = component_ref.scrolling_enabled();
    }
}