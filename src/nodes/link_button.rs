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

pub struct LinkButtonPlugin;

impl Plugin for LinkButtonPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<LinkButton>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a link_button.
pub fn is_link_button(node: &gdnative::prelude::Node) -> bool {
    node.cast::<LinkButton>().is_some()
}

/// A bundle for LinkButtons.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDLinkButtonBundle {
    pub link_button: GDLinkButton,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub base_button: GDBaseButton,
    pub true_type: TrueNodeType,
}

impl Default for GDLinkButtonBundle {
    fn default() -> Self {
        Self {
            link_button: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
base_button: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "LinkButton".to_string()
            }
        }
    }
}

/// Represents a LinkButton.
#[derive(Component)]
pub struct GDLinkButton {
    pub text: String,
}

impl Default for GDLinkButton {
    fn default() -> Self {
        Self {
            text: Default::default(),
        }
    }
}

impl NodeClass for GDLinkButton {
    type Parent = GDBaseButton;
    type GodotClass = LinkButton;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<LinkButton>().unwrap();
        world_commands.insert(entity, GDLinkButton {
            text: component_ref.text().to_string(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDLinkButton {
    
}

fn sync_bevy_owned(query: Query<(&GDLinkButton, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<LinkButton>().unwrap();
        component_ref.set_text(component.text.clone());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDLinkButton, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<LinkButton>().unwrap();
        component.text = component_ref.text().to_string();
    }
}