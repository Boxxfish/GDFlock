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

pub struct SpriteBase3DPlugin;

impl Plugin for SpriteBase3DPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a sprite_base_3_d.
pub fn is_sprite_base_3_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<SpriteBase3D>().is_some()
}

/// A bundle for SpriteBase3Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDSpriteBase3DBundle {
    pub sprite_base_3_d: GDSpriteBase3D,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub geometry_instance: GDGeometryInstance,
    pub true_type: TrueNodeType,
}

impl Default for GDSpriteBase3DBundle {
    fn default() -> Self {
        Self {
            sprite_base_3_d: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
geometry_instance: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "SpriteBase3D".to_string()
            }
        }
    }
}

/// Represents a SpriteBase3D.
#[derive(Component)]
pub struct GDSpriteBase3D {
    pub centered: bool,
pub flip_h: bool,
pub flip_v: bool,
pub modulate: Color,
pub offset: Vector2,
pub opacity: f64,
pub pixel_size: f64,
pub render_priority: i64,
}

impl Default for GDSpriteBase3D {
    fn default() -> Self {
        Self {
            centered: Default::default(),
flip_h: Default::default(),
flip_v: Default::default(),
modulate: Color::from_rgb(0.0, 0.0, 0.0),
offset: Default::default(),
opacity: Default::default(),
pixel_size: Default::default(),
render_priority: Default::default(),
        }
    }
}

impl NodeClass for GDSpriteBase3D {
    type Parent = GDGeometryInstance;
    type GodotClass = SpriteBase3D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<SpriteBase3D>().unwrap();
        world_commands.insert(entity, GDSpriteBase3D {
            centered: component_ref.is_centered(),
flip_h: component_ref.is_flipped_h(),
flip_v: component_ref.is_flipped_v(),
modulate: component_ref.modulate(),
offset: component_ref.offset(),
opacity: component_ref.opacity(),
pixel_size: component_ref.pixel_size(),
render_priority: component_ref.render_priority(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDSpriteBase3D {
    
}

fn sync_bevy_owned(query: Query<(&GDSpriteBase3D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<SpriteBase3D>().unwrap();
        component_ref.set_centered(component.centered);
component_ref.set_flip_h(component.flip_h);
component_ref.set_flip_v(component.flip_v);
component_ref.set_modulate(component.modulate);
component_ref.set_offset(component.offset);
component_ref.set_opacity(component.opacity);
component_ref.set_pixel_size(component.pixel_size);
component_ref.set_render_priority(component.render_priority);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDSpriteBase3D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<SpriteBase3D>().unwrap();
        component.centered = component_ref.is_centered();
component.flip_h = component_ref.is_flipped_h();
component.flip_v = component_ref.is_flipped_v();
component.modulate = component_ref.modulate();
component.offset = component_ref.offset();
component.opacity = component_ref.opacity();
component.pixel_size = component_ref.pixel_size();
component.render_priority = component_ref.render_priority();
    }
}