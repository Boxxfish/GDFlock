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

pub struct CSGShapePlugin;

impl Plugin for CSGShapePlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a c_s_g_shape.
pub fn is_c_s_g_shape(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CSGShape>().is_some()
}

/// A bundle for CSGShapes.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCSGShapeBundle {
    pub c_s_g_shape: GDCSGShape,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub geometry_instance: GDGeometryInstance,
    pub true_type: TrueNodeType,
}

impl Default for GDCSGShapeBundle {
    fn default() -> Self {
        Self {
            c_s_g_shape: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
geometry_instance: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CSGShape".to_string()
            }
        }
    }
}

/// Represents a CSGShape.
#[derive(Component)]
pub struct GDCSGShape {
    pub calculate_tangents: bool,
pub collision_layer: i64,
pub collision_mask: i64,
pub snap: f64,
pub use_collision: bool,
}

impl Default for GDCSGShape {
    fn default() -> Self {
        Self {
            calculate_tangents: Default::default(),
collision_layer: Default::default(),
collision_mask: Default::default(),
snap: Default::default(),
use_collision: Default::default(),
        }
    }
}

impl NodeClass for GDCSGShape {
    type Parent = GDGeometryInstance;
    type GodotClass = CSGShape;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CSGShape>().unwrap();
        world_commands.insert(entity, GDCSGShape {
            calculate_tangents: component_ref.is_calculating_tangents(),
collision_layer: component_ref.collision_layer(),
collision_mask: component_ref.collision_mask(),
snap: component_ref.snap(),
use_collision: component_ref.is_using_collision(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCSGShape {
    
}

fn sync_bevy_owned(query: Query<(&GDCSGShape, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CSGShape>().unwrap();
        component_ref.set_calculate_tangents(component.calculate_tangents);
component_ref.set_collision_layer(component.collision_layer);
component_ref.set_collision_mask(component.collision_mask);
component_ref.set_snap(component.snap);
component_ref.set_use_collision(component.use_collision);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCSGShape, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CSGShape>().unwrap();
        component.calculate_tangents = component_ref.is_calculating_tangents();
component.collision_layer = component_ref.collision_layer();
component.collision_mask = component_ref.collision_mask();
component.snap = component_ref.snap();
component.use_collision = component_ref.is_using_collision();
    }
}