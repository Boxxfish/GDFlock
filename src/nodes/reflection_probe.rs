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

pub struct ReflectionProbePlugin;

impl Plugin for ReflectionProbePlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<ReflectionProbe>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a reflection_probe.
pub fn is_reflection_probe(node: &gdnative::prelude::Node) -> bool {
    node.cast::<ReflectionProbe>().is_some()
}

/// A bundle for ReflectionProbes.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDReflectionProbeBundle {
    pub reflection_probe: GDReflectionProbe,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
    pub true_type: TrueNodeType,
}

impl Default for GDReflectionProbeBundle {
    fn default() -> Self {
        Self {
            reflection_probe: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "ReflectionProbe".to_string()
            }
        }
    }
}

/// Represents a ReflectionProbe.
#[derive(Component)]
pub struct GDReflectionProbe {
    pub box_projection: bool,
pub cull_mask: i64,
pub enable_shadows: bool,
pub extents: Vector3,
pub intensity: f64,
pub interior_ambient_color: Color,
pub interior_ambient_contrib: f64,
pub interior_ambient_energy: f64,
pub interior_enable: bool,
pub max_distance: f64,
pub origin_offset: Vector3,
}

impl Default for GDReflectionProbe {
    fn default() -> Self {
        Self {
            box_projection: Default::default(),
cull_mask: Default::default(),
enable_shadows: Default::default(),
extents: Default::default(),
intensity: Default::default(),
interior_ambient_color: Color::from_rgb(0.0, 0.0, 0.0),
interior_ambient_contrib: Default::default(),
interior_ambient_energy: Default::default(),
interior_enable: Default::default(),
max_distance: Default::default(),
origin_offset: Default::default(),
        }
    }
}

impl NodeClass for GDReflectionProbe {
    type Parent = GDVisualInstance;
    type GodotClass = ReflectionProbe;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<ReflectionProbe>().unwrap();
        world_commands.insert(entity, GDReflectionProbe {
            box_projection: component_ref.is_box_projection_enabled(),
cull_mask: component_ref.cull_mask(),
enable_shadows: component_ref.are_shadows_enabled(),
extents: component_ref.extents(),
intensity: component_ref.intensity(),
interior_ambient_color: component_ref.interior_ambient(),
interior_ambient_contrib: component_ref.interior_ambient_probe_contribution(),
interior_ambient_energy: component_ref.interior_ambient_energy(),
interior_enable: component_ref.is_set_as_interior(),
max_distance: component_ref.max_distance(),
origin_offset: component_ref.origin_offset(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDReflectionProbe {
    
}

fn sync_bevy_owned(query: Query<(&GDReflectionProbe, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ReflectionProbe>().unwrap();
        component_ref.set_enable_box_projection(component.box_projection);
component_ref.set_cull_mask(component.cull_mask);
component_ref.set_enable_shadows(component.enable_shadows);
component_ref.set_extents(component.extents);
component_ref.set_intensity(component.intensity);
component_ref.set_interior_ambient(component.interior_ambient_color);
component_ref.set_interior_ambient_probe_contribution(component.interior_ambient_contrib);
component_ref.set_interior_ambient_energy(component.interior_ambient_energy);
component_ref.set_as_interior(component.interior_enable);
component_ref.set_max_distance(component.max_distance);
component_ref.set_origin_offset(component.origin_offset);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDReflectionProbe, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<ReflectionProbe>().unwrap();
        component.box_projection = component_ref.is_box_projection_enabled();
component.cull_mask = component_ref.cull_mask();
component.enable_shadows = component_ref.are_shadows_enabled();
component.extents = component_ref.extents();
component.intensity = component_ref.intensity();
component.interior_ambient_color = component_ref.interior_ambient();
component.interior_ambient_contrib = component_ref.interior_ambient_probe_contribution();
component.interior_ambient_energy = component_ref.interior_ambient_energy();
component.interior_enable = component_ref.is_set_as_interior();
component.max_distance = component_ref.max_distance();
component.origin_offset = component_ref.origin_offset();
    }
}