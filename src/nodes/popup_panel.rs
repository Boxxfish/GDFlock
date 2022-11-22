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

pub struct PopupPanelPlugin;

impl Plugin for PopupPanelPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<PopupPanel>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a popup_panel.
pub fn is_popup_panel(node: &gdnative::prelude::Node) -> bool {
    node.cast::<PopupPanel>().is_some()
}

/// A bundle for PopupPanels.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDPopupPanelBundle {
    pub popup_panel: GDPopupPanel,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub popup: GDPopup,
    pub true_type: TrueNodeType,
}

impl Default for GDPopupPanelBundle {
    fn default() -> Self {
        Self {
            popup_panel: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
popup: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "PopupPanel".to_string()
            }
        }
    }
}

/// Represents a PopupPanel.
#[derive(Component)]
pub struct GDPopupPanel {
    
}

impl Default for GDPopupPanel {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDPopupPanel {
    type Parent = GDPopup;
    type GodotClass = PopupPanel;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<PopupPanel>().unwrap();
        world_commands.insert(entity, GDPopupPanel {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDPopupPanel {
    
}

fn sync_bevy_owned(query: Query<(&GDPopupPanel, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PopupPanel>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDPopupPanel, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PopupPanel>().unwrap();
        
    }
}