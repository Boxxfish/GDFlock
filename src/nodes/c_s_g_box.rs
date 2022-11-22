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

pub struct CSGBoxPlugin;

impl Plugin for CSGBoxPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<CSGBox>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a c_s_g_box.
pub fn is_c_s_g_box(node: &gdnative::prelude::Node) -> bool {
    node.cast::<CSGBox>().is_some()
}

/// A bundle for CSGBoxs.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDCSGBoxBundle {
    pub c_s_g_box: GDCSGBox,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub geometry_instance: GDGeometryInstance,
pub c_s_g_shape: GDCSGShape,
pub c_s_g_primitive: GDCSGPrimitive,
    pub true_type: TrueNodeType,
}

impl Default for GDCSGBoxBundle {
    fn default() -> Self {
        Self {
            c_s_g_box: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
geometry_instance: Default::default(),
c_s_g_shape: Default::default(),
c_s_g_primitive: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "CSGBox".to_string()
            }
        }
    }
}

/// Represents a CSGBox.
#[derive(Component)]
pub struct GDCSGBox {
    pub depth: f64,
pub height: f64,
pub width: f64,
}

impl Default for GDCSGBox {
    fn default() -> Self {
        Self {
            depth: Default::default(),
height: Default::default(),
width: Default::default(),
        }
    }
}

impl NodeClass for GDCSGBox {
    type Parent = GDCSGPrimitive;
    type GodotClass = CSGBox;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<CSGBox>().unwrap();
        world_commands.insert(entity, GDCSGBox {
            depth: component_ref.depth(),
height: component_ref.height(),
width: component_ref.width(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDCSGBox {
    
}

fn sync_bevy_owned(query: Query<(&GDCSGBox, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CSGBox>().unwrap();
        component_ref.set_depth(component.depth);
component_ref.set_height(component.height);
component_ref.set_width(component.width);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDCSGBox, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<CSGBox>().unwrap();
        component.depth = component_ref.depth();
component.height = component_ref.height();
component.width = component_ref.width();
    }
}