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

pub struct RemoteTransform2DPlugin;

impl Plugin for RemoteTransform2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<RemoteTransform2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a remote_transform_2_d.
pub fn is_remote_transform_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<RemoteTransform2D>().is_some()
}

/// A bundle for RemoteTransform2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDRemoteTransform2DBundle {
    pub remote_transform_2_d: GDRemoteTransform2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDRemoteTransform2DBundle {
    fn default() -> Self {
        Self {
            remote_transform_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "RemoteTransform2D".to_string()
            }
        }
    }
}

/// Represents a RemoteTransform2D.
#[derive(Component)]
pub struct GDRemoteTransform2D {
    pub remote_path: NodePath,
pub update_position: bool,
pub update_rotation: bool,
pub update_scale: bool,
pub use_global_coordinates: bool,
}

impl Default for GDRemoteTransform2D {
    fn default() -> Self {
        Self {
            remote_path: Default::default(),
update_position: Default::default(),
update_rotation: Default::default(),
update_scale: Default::default(),
use_global_coordinates: Default::default(),
        }
    }
}

impl NodeClass for GDRemoteTransform2D {
    type Parent = GDNode2D;
    type GodotClass = RemoteTransform2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<RemoteTransform2D>().unwrap();
        world_commands.insert(entity, GDRemoteTransform2D {
            remote_path: component_ref.remote_node(),
update_position: component_ref.update_position(),
update_rotation: component_ref.update_rotation(),
update_scale: component_ref.update_scale(),
use_global_coordinates: component_ref.use_global_coordinates(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDRemoteTransform2D {
    
}

fn sync_bevy_owned(query: Query<(&GDRemoteTransform2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<RemoteTransform2D>().unwrap();
        component_ref.set_remote_node(component.remote_path.to_godot_string());
component_ref.set_update_position(component.update_position);
component_ref.set_update_rotation(component.update_rotation);
component_ref.set_update_scale(component.update_scale);
component_ref.set_use_global_coordinates(component.use_global_coordinates);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDRemoteTransform2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<RemoteTransform2D>().unwrap();
        component.remote_path = component_ref.remote_node();
component.update_position = component_ref.update_position();
component.update_rotation = component_ref.update_rotation();
component.update_scale = component_ref.update_scale();
component.use_global_coordinates = component_ref.use_global_coordinates();
    }
}