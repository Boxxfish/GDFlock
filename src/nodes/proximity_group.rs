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

pub struct ProximityGroupPlugin;

impl Plugin for ProximityGroupPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ProximityGroup>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a proximity_group.
pub fn is_proximity_group(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ProximityGroup>().is_some()
}

/// A bundle for ProximityGroups.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDProximityGroupBundle {
    pub proximity_group: GDProximityGroup,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDProximityGroupBundle {
    fn default() -> Self {
        Self {
            proximity_group: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ProximityGroup".to_string()
            }
        }
    }
}

/// Represents a ProximityGroup.
#[derive(Component)]
pub struct GDProximityGroup {
    pub grid_radius: Vector3,
pub group_name: String,
}

impl Default for GDProximityGroup {
    fn default() -> Self {
        Self {
            grid_radius: Default::default(),
group_name: Default::default(),
        }
    }
}

impl NodeClass for GDProximityGroup {
    type Parent = GDSpatial;
    type GodotClass = ProximityGroup;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ProximityGroup>().unwrap();
        world_commands.insert(entity, GDProximityGroup {
            grid_radius: component_ref.grid_radius(),
group_name: component_ref.group_name().to_string(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDProximityGroup {
    
}

fn sync_bevy_owned(query: Query<(&GDProximityGroup, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ProximityGroup>().unwrap();
        component_ref.set_grid_radius(component.grid_radius);
component_ref.set_group_name(component.group_name.clone());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDProximityGroup, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ProximityGroup>().unwrap();
        component.grid_radius = component_ref.grid_radius();
component.group_name = component_ref.group_name().to_string();
    }
}