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

pub struct VisibilityNotifier2DPlugin;

impl Plugin for VisibilityNotifier2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<VisibilityNotifier2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a visibility_notifier_2_d.
pub fn is_visibility_notifier_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<VisibilityNotifier2D>().is_some()
}

/// A bundle for VisibilityNotifier2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDVisibilityNotifier2DBundle {
    pub visibility_notifier_2_d: GDVisibilityNotifier2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDVisibilityNotifier2DBundle {
    fn default() -> Self {
        Self {
            visibility_notifier_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "VisibilityNotifier2D".to_string()
            }
        }
    }
}

/// Represents a VisibilityNotifier2D.
#[derive(Component)]
pub struct GDVisibilityNotifier2D {
    pub rect: Rect2,
}

impl Default for GDVisibilityNotifier2D {
    fn default() -> Self {
        Self {
            rect: Rect2::from_components(0.0, 0.0, 0.0, 0.0),
        }
    }
}

impl NodeClass for GDVisibilityNotifier2D {
    type Parent = GDNode2D;
    type GodotClass = VisibilityNotifier2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<VisibilityNotifier2D>().unwrap();
        world_commands.insert(entity, GDVisibilityNotifier2D {
            rect: component_ref.rect(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDVisibilityNotifier2D {
    
}

fn sync_bevy_owned(query: Query<(&GDVisibilityNotifier2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VisibilityNotifier2D>().unwrap();
        component_ref.set_rect(component.rect);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDVisibilityNotifier2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VisibilityNotifier2D>().unwrap();
        component.rect = component_ref.rect();
    }
}