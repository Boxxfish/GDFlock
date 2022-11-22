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

pub struct PortalPlugin;

impl Plugin for PortalPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Portal>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a portal.
pub fn is_portal(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Portal>().is_some()
}

/// A bundle for Portals.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDPortalBundle {
    pub portal: GDPortal,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDPortalBundle {
    fn default() -> Self {
        Self {
            portal: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Portal".to_string()
            }
        }
    }
}

/// Represents a Portal.
#[derive(Component)]
pub struct GDPortal {
    pub linked_room: NodePath,
pub points: Vec<Vector2>,
pub portal_active: bool,
pub portal_margin: f64,
pub two_way: bool,
pub use_default_margin: bool,
}

impl Default for GDPortal {
    fn default() -> Self {
        Self {
            linked_room: Default::default(),
points: Default::default(),
portal_active: Default::default(),
portal_margin: Default::default(),
two_way: Default::default(),
use_default_margin: Default::default(),
        }
    }
}

impl NodeClass for GDPortal {
    type Parent = GDSpatial;
    type GodotClass = Portal;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Portal>().unwrap();
        world_commands.insert(entity, GDPortal {
            linked_room: component_ref.linked_room(),
points: component_ref.points().to_vec(),
portal_active: component_ref.portal_active(),
portal_margin: component_ref.portal_margin(),
two_way: component_ref.is_two_way(),
use_default_margin: component_ref.use_default_margin(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDPortal {
    
}

fn sync_bevy_owned(query: Query<(&GDPortal, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Portal>().unwrap();
        component_ref.set_linked_room(component.linked_room.to_godot_string());
component_ref.set_points(Vector2Array::from_vec(component.points.clone()));
component_ref.set_portal_active(component.portal_active);
component_ref.set_portal_margin(component.portal_margin);
component_ref.set_two_way(component.two_way);
component_ref.set_use_default_margin(component.use_default_margin);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDPortal, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Portal>().unwrap();
        component.linked_room = component_ref.linked_room();
component.points = component_ref.points().to_vec();
component.portal_active = component_ref.portal_active();
component.portal_margin = component_ref.portal_margin();
component.two_way = component_ref.is_two_way();
component.use_default_margin = component_ref.use_default_margin();
    }
}