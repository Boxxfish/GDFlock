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

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a light.
pub fn is_light(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Light>().is_some()
}

/// A bundle for Lights.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDLightBundle {
    pub light: GDLight,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
    pub true_type: TrueNodeType,
}

impl Default for GDLightBundle {
    fn default() -> Self {
        Self {
            light: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Light".to_string()
            }
        }
    }
}

/// Represents a Light.
#[derive(Component)]
pub struct GDLight {
    pub editor_only: bool,
pub light_color: Color,
pub light_cull_mask: i64,
pub light_negative: bool,
pub shadow_color: Color,
pub shadow_enabled: bool,
pub shadow_reverse_cull_face: bool,
}

impl Default for GDLight {
    fn default() -> Self {
        Self {
            editor_only: Default::default(),
light_color: Color::from_rgb(0.0, 0.0, 0.0),
light_cull_mask: Default::default(),
light_negative: Default::default(),
shadow_color: Color::from_rgb(0.0, 0.0, 0.0),
shadow_enabled: Default::default(),
shadow_reverse_cull_face: Default::default(),
        }
    }
}

impl NodeClass for GDLight {
    type Parent = GDVisualInstance;
    type GodotClass = Light;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Light>().unwrap();
        world_commands.insert(entity, GDLight {
            editor_only: component_ref.is_editor_only(),
light_color: component_ref.color(),
light_cull_mask: component_ref.cull_mask(),
light_negative: component_ref.is_negative(),
shadow_color: component_ref.shadow_color(),
shadow_enabled: component_ref.has_shadow(),
shadow_reverse_cull_face: component_ref.shadow_reverse_cull_face(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDLight {
    
}

fn sync_bevy_owned(query: Query<(&GDLight, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Light>().unwrap();
        component_ref.set_editor_only(component.editor_only);
component_ref.set_color(component.light_color);
component_ref.set_cull_mask(component.light_cull_mask);
component_ref.set_negative(component.light_negative);
component_ref.set_shadow_color(component.shadow_color);
component_ref.set_shadow(component.shadow_enabled);
component_ref.set_shadow_reverse_cull_face(component.shadow_reverse_cull_face);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDLight, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Light>().unwrap();
        component.editor_only = component_ref.is_editor_only();
component.light_color = component_ref.color();
component.light_cull_mask = component_ref.cull_mask();
component.light_negative = component_ref.is_negative();
component.shadow_color = component_ref.shadow_color();
component.shadow_enabled = component_ref.has_shadow();
component.shadow_reverse_cull_face = component_ref.shadow_reverse_cull_face();
    }
}