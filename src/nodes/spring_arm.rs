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

pub struct SpringArmPlugin;

impl Plugin for SpringArmPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<SpringArm>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a spring_arm.
pub fn is_spring_arm(node: &gdnative::prelude::Node) -> bool {
    node.cast::<SpringArm>().is_some()
}

/// A bundle for SpringArms.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDSpringArmBundle {
    pub spring_arm: GDSpringArm,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDSpringArmBundle {
    fn default() -> Self {
        Self {
            spring_arm: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "SpringArm".to_string()
            }
        }
    }
}

/// Represents a SpringArm.
#[derive(Component)]
pub struct GDSpringArm {
    pub collision_mask: i64,
pub margin: f64,
pub spring_length: f64,
}

impl Default for GDSpringArm {
    fn default() -> Self {
        Self {
            collision_mask: Default::default(),
margin: Default::default(),
spring_length: Default::default(),
        }
    }
}

impl NodeClass for GDSpringArm {
    type Parent = GDSpatial;
    type GodotClass = SpringArm;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<SpringArm>().unwrap();
        world_commands.insert(entity, GDSpringArm {
            collision_mask: component_ref.collision_mask(),
margin: component_ref.margin(),
spring_length: component_ref.length(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDSpringArm {
    
}

fn sync_bevy_owned(query: Query<(&GDSpringArm, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<SpringArm>().unwrap();
        component_ref.set_collision_mask(component.collision_mask);
component_ref.set_margin(component.margin);
component_ref.set_length(component.spring_length);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDSpringArm, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<SpringArm>().unwrap();
        component.collision_mask = component_ref.collision_mask();
component.margin = component_ref.margin();
component.spring_length = component_ref.length();
    }
}