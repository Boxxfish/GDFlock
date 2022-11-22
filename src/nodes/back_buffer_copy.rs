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

pub struct BackBufferCopyPlugin;

impl Plugin for BackBufferCopyPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<BackBufferCopy>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a back_buffer_copy.
pub fn is_back_buffer_copy(node: &gdnative::prelude::Node) -> bool {
    node.cast::<BackBufferCopy>().is_some()
}

/// A bundle for BackBufferCopys.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDBackBufferCopyBundle {
    pub back_buffer_copy: GDBackBufferCopy,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDBackBufferCopyBundle {
    fn default() -> Self {
        Self {
            back_buffer_copy: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "BackBufferCopy".to_string()
            }
        }
    }
}

/// Represents a BackBufferCopy.
#[derive(Component)]
pub struct GDBackBufferCopy {
    pub rect: Rect2,
}

impl Default for GDBackBufferCopy {
    fn default() -> Self {
        Self {
            rect: Rect2::from_components(0.0, 0.0, 0.0, 0.0),
        }
    }
}

impl NodeClass for GDBackBufferCopy {
    type Parent = GDNode2D;
    type GodotClass = BackBufferCopy;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<BackBufferCopy>().unwrap();
        world_commands.insert(entity, GDBackBufferCopy {
            rect: component_ref.rect(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDBackBufferCopy {
    
}

fn sync_bevy_owned(query: Query<(&GDBackBufferCopy, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<BackBufferCopy>().unwrap();
        component_ref.set_rect(component.rect);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDBackBufferCopy, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<BackBufferCopy>().unwrap();
        component.rect = component_ref.rect();
    }
}