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

pub struct PhysicalBonePlugin;

impl Plugin for PhysicalBonePlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<PhysicalBone>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a physical_bone.
pub fn is_physical_bone(node: &gdnative::prelude::Node) -> bool {
    node.cast::<PhysicalBone>().is_some()
}

/// A bundle for PhysicalBones.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDPhysicalBoneBundle {
    pub physical_bone: GDPhysicalBone,
    pub node: GDNode,
pub spatial: GDSpatial,
pub collision_object: GDCollisionObject,
pub physics_body: GDPhysicsBody,
    pub true_type: TrueNodeType,
}

impl Default for GDPhysicalBoneBundle {
    fn default() -> Self {
        Self {
            physical_bone: Default::default(),
            node: Default::default(),
spatial: Default::default(),
collision_object: Default::default(),
physics_body: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "PhysicalBone".to_string()
            }
        }
    }
}

/// Represents a PhysicalBone.
#[derive(Component)]
pub struct GDPhysicalBone {
    pub body_offset: Transform,
pub bounce: f64,
pub friction: f64,
pub gravity_scale: f64,
pub joint_offset: Transform,
pub mass: f64,
pub weight: f64,
}

impl Default for GDPhysicalBone {
    fn default() -> Self {
        Self {
            body_offset: Transform::IDENTITY,
bounce: Default::default(),
friction: Default::default(),
gravity_scale: Default::default(),
joint_offset: Transform::IDENTITY,
mass: Default::default(),
weight: Default::default(),
        }
    }
}

impl NodeClass for GDPhysicalBone {
    type Parent = GDPhysicsBody;
    type GodotClass = PhysicalBone;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<PhysicalBone>().unwrap();
        world_commands.insert(entity, GDPhysicalBone {
            body_offset: component_ref.body_offset(),
bounce: component_ref.bounce(),
friction: component_ref.friction(),
gravity_scale: component_ref.gravity_scale(),
joint_offset: component_ref.joint_offset(),
mass: component_ref.mass(),
weight: component_ref.weight(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDPhysicalBone {
    
}

fn sync_bevy_owned(query: Query<(&GDPhysicalBone, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PhysicalBone>().unwrap();
        component_ref.set_body_offset(component.body_offset);
component_ref.set_bounce(component.bounce);
component_ref.set_friction(component.friction);
component_ref.set_gravity_scale(component.gravity_scale);
component_ref.set_joint_offset(component.joint_offset);
component_ref.set_mass(component.mass);
component_ref.set_weight(component.weight);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDPhysicalBone, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<PhysicalBone>().unwrap();
        component.body_offset = component_ref.body_offset();
component.bounce = component_ref.bounce();
component.friction = component_ref.friction();
component.gravity_scale = component_ref.gravity_scale();
component.joint_offset = component_ref.joint_offset();
component.mass = component_ref.mass();
component.weight = component_ref.weight();
    }
}