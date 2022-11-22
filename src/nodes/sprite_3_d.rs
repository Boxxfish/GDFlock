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

pub struct Sprite3DPlugin;

impl Plugin for Sprite3DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Sprite3D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a sprite_3_d.
pub fn is_sprite_3_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Sprite3D>().is_some()
}

/// A bundle for Sprite3Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDSprite3DBundle {
    pub sprite_3_d: GDSprite3D,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub geometry_instance: GDGeometryInstance,
pub sprite_base_3_d: GDSpriteBase3D,
    pub true_type: TrueNodeType,
}

impl Default for GDSprite3DBundle {
    fn default() -> Self {
        Self {
            sprite_3_d: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
geometry_instance: Default::default(),
sprite_base_3_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Sprite3D".to_string()
            }
        }
    }
}

/// Represents a Sprite3D.
#[derive(Component)]
pub struct GDSprite3D {
    pub frame: i64,
pub frame_coords: Vector2,
pub hframes: i64,
pub region_enabled: bool,
pub region_rect: Rect2,
pub texture: Option<Ref<Texture>>,
pub vframes: i64,
}

impl Default for GDSprite3D {
    fn default() -> Self {
        Self {
            frame: Default::default(),
frame_coords: Default::default(),
hframes: Default::default(),
region_enabled: Default::default(),
region_rect: Rect2::from_components(0.0, 0.0, 0.0, 0.0),
texture: Default::default(),
vframes: Default::default(),
        }
    }
}

impl NodeClass for GDSprite3D {
    type Parent = GDSpriteBase3D;
    type GodotClass = Sprite3D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Sprite3D>().unwrap();
        world_commands.insert(entity, GDSprite3D {
            frame: component_ref.frame(),
frame_coords: component_ref.frame_coords(),
hframes: component_ref.hframes(),
region_enabled: component_ref.is_region(),
region_rect: component_ref.region_rect(),
texture: component_ref.texture(),
vframes: component_ref.vframes(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDSprite3D {
    
}

fn sync_bevy_owned(query: Query<(&GDSprite3D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Sprite3D>().unwrap();
        component_ref.set_frame(component.frame);
component_ref.set_frame_coords(component.frame_coords);
component_ref.set_hframes(component.hframes);
component_ref.set_region(component.region_enabled);
component_ref.set_region_rect(component.region_rect);
component_ref.set_texture(component.texture.as_ref().unwrap().clone());
component_ref.set_vframes(component.vframes);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDSprite3D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Sprite3D>().unwrap();
        component.frame = component_ref.frame();
component.frame_coords = component_ref.frame_coords();
component.hframes = component_ref.hframes();
component.region_enabled = component_ref.is_region();
component.region_rect = component_ref.region_rect();
component.texture = component_ref.texture();
component.vframes = component_ref.vframes();
    }
}