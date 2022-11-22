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

pub struct AnimatedSprite3DPlugin;

impl Plugin for AnimatedSprite3DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<AnimatedSprite3D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a animated_sprite_3_d.
pub fn is_animated_sprite_3_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<AnimatedSprite3D>().is_some()
}

/// A bundle for AnimatedSprite3Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDAnimatedSprite3DBundle {
    pub animated_sprite_3_d: GDAnimatedSprite3D,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub geometry_instance: GDGeometryInstance,
pub sprite_base_3_d: GDSpriteBase3D,
    pub true_type: TrueNodeType,
}

impl Default for GDAnimatedSprite3DBundle {
    fn default() -> Self {
        Self {
            animated_sprite_3_d: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
geometry_instance: Default::default(),
sprite_base_3_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "AnimatedSprite3D".to_string()
            }
        }
    }
}

/// Represents a AnimatedSprite3D.
#[derive(Component)]
pub struct GDAnimatedSprite3D {
    pub animation: String,
pub frame: i64,
}

impl Default for GDAnimatedSprite3D {
    fn default() -> Self {
        Self {
            animation: Default::default(),
frame: Default::default(),
        }
    }
}

impl NodeClass for GDAnimatedSprite3D {
    type Parent = GDSpriteBase3D;
    type GodotClass = AnimatedSprite3D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<AnimatedSprite3D>().unwrap();
        world_commands.insert(entity, GDAnimatedSprite3D {
            animation: component_ref.animation().to_string(),
frame: component_ref.frame(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDAnimatedSprite3D {
    
}

fn sync_bevy_owned(query: Query<(&GDAnimatedSprite3D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AnimatedSprite3D>().unwrap();
        component_ref.set_animation(component.animation.clone());
component_ref.set_frame(component.frame);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDAnimatedSprite3D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AnimatedSprite3D>().unwrap();
        component.animation = component_ref.animation().to_string();
component.frame = component_ref.frame();
    }
}