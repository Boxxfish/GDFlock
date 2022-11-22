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

pub struct CheckBoxPlugin;

impl Plugin for CheckBoxPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<CheckBox>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a check_box.
pub fn is_check_box(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CheckBox>().is_some()
}

/// A bundle for CheckBoxs.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCheckBoxBundle {
    pub check_box: GDCheckBox,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub base_button: GDBaseButton,
pub button: GDButton,
    pub true_type: TrueNodeType,
}

impl Default for GDCheckBoxBundle {
    fn default() -> Self {
        Self {
            check_box: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
base_button: Default::default(),
button: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CheckBox".to_string()
            }
        }
    }
}

/// Represents a CheckBox.
#[derive(Component)]
pub struct GDCheckBox {
    
}

impl Default for GDCheckBox {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDCheckBox {
    type Parent = GDButton;
    type GodotClass = CheckBox;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CheckBox>().unwrap();
        world_commands.insert(entity, GDCheckBox {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCheckBox {
    
}

fn sync_bevy_owned(query: Query<(&GDCheckBox, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CheckBox>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCheckBox, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CheckBox>().unwrap();
        
    }
}