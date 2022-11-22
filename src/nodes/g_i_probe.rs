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

pub struct GIProbePlugin;

impl Plugin for GIProbePlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<GIProbe>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a g_i_probe.
pub fn is_g_i_probe(node: &gdnative::prelude::Node) -> bool {
    node.cast::<GIProbe>().is_some()
}

/// A bundle for GIProbes.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDGIProbeBundle {
    pub g_i_probe: GDGIProbe,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
    pub true_type: TrueNodeType,
}

impl Default for GDGIProbeBundle {
    fn default() -> Self {
        Self {
            g_i_probe: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "GIProbe".to_string()
            }
        }
    }
}

/// Represents a GIProbe.
#[derive(Component)]
pub struct GDGIProbe {
    pub bias: f64,
pub compress: bool,
pub dynamic_range: i64,
pub energy: f64,
pub extents: Vector3,
pub interior: bool,
pub normal_bias: f64,
pub propagation: f64,
}

impl Default for GDGIProbe {
    fn default() -> Self {
        Self {
            bias: Default::default(),
compress: Default::default(),
dynamic_range: Default::default(),
energy: Default::default(),
extents: Default::default(),
interior: Default::default(),
normal_bias: Default::default(),
propagation: Default::default(),
        }
    }
}

impl NodeClass for GDGIProbe {
    type Parent = GDVisualInstance;
    type GodotClass = GIProbe;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<GIProbe>().unwrap();
        world_commands.insert(entity, GDGIProbe {
            bias: component_ref.bias(),
compress: component_ref.is_compressed(),
dynamic_range: component_ref.dynamic_range(),
energy: component_ref.energy(),
extents: component_ref.extents(),
interior: component_ref.is_interior(),
normal_bias: component_ref.normal_bias(),
propagation: component_ref.propagation(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDGIProbe {
    
}

fn sync_bevy_owned(query: Query<(&GDGIProbe, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<GIProbe>().unwrap();
        component_ref.set_bias(component.bias);
component_ref.set_compress(component.compress);
component_ref.set_dynamic_range(component.dynamic_range);
component_ref.set_energy(component.energy);
component_ref.set_extents(component.extents);
component_ref.set_interior(component.interior);
component_ref.set_normal_bias(component.normal_bias);
component_ref.set_propagation(component.propagation);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDGIProbe, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<GIProbe>().unwrap();
        component.bias = component_ref.bias();
component.compress = component_ref.is_compressed();
component.dynamic_range = component_ref.dynamic_range();
component.energy = component_ref.energy();
component.extents = component_ref.extents();
component.interior = component_ref.is_interior();
component.normal_bias = component_ref.normal_bias();
component.propagation = component_ref.propagation();
    }
}