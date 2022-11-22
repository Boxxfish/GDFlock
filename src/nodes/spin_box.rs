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

pub struct SpinBoxPlugin;

impl Plugin for SpinBoxPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<SpinBox>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a spin_box.
pub fn is_spin_box(node: &gdnative::prelude::Node) -> bool {
    node.cast::<SpinBox>().is_some()
}

/// A bundle for SpinBoxs.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDSpinBoxBundle {
    pub spin_box: GDSpinBox,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub range: GDRange,
    pub true_type: TrueNodeType,
}

impl Default for GDSpinBoxBundle {
    fn default() -> Self {
        Self {
            spin_box: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
range: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "SpinBox".to_string()
            }
        }
    }
}

/// Represents a SpinBox.
#[derive(Component)]
pub struct GDSpinBox {
    pub editable: bool,
pub prefix: String,
pub suffix: String,
}

impl Default for GDSpinBox {
    fn default() -> Self {
        Self {
            editable: Default::default(),
prefix: Default::default(),
suffix: Default::default(),
        }
    }
}

impl NodeClass for GDSpinBox {
    type Parent = GDRange;
    type GodotClass = SpinBox;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<SpinBox>().unwrap();
        world_commands.insert(entity, GDSpinBox {
            editable: component_ref.is_editable(),
prefix: component_ref.prefix().to_string(),
suffix: component_ref.suffix().to_string(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDSpinBox {
    
}

fn sync_bevy_owned(query: Query<(&GDSpinBox, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<SpinBox>().unwrap();
        component_ref.set_editable(component.editable);
component_ref.set_prefix(component.prefix.clone());
component_ref.set_suffix(component.suffix.clone());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDSpinBox, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<SpinBox>().unwrap();
        component.editable = component_ref.is_editable();
component.prefix = component_ref.prefix().to_string();
component.suffix = component_ref.suffix().to_string();
    }
}