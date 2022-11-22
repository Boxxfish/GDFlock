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

pub struct BakedLightmapPlugin;

impl Plugin for BakedLightmapPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<BakedLightmap>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a baked_lightmap.
pub fn is_baked_lightmap(node: &gdnative::prelude::Node) -> bool {
    node.cast::<BakedLightmap>().is_some()
}

/// A bundle for BakedLightmaps.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDBakedLightmapBundle {
    pub baked_lightmap: GDBakedLightmap,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
    pub true_type: TrueNodeType,
}

impl Default for GDBakedLightmapBundle {
    fn default() -> Self {
        Self {
            baked_lightmap: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "BakedLightmap".to_string()
            }
        }
    }
}

/// Represents a BakedLightmap.
#[derive(Component)]
pub struct GDBakedLightmap {
    pub atlas_generate: bool,
pub atlas_max_size: i64,
pub bias: f64,
pub bounce_indirect_energy: f64,
pub bounces: i64,
pub capture_cell_size: f64,
pub capture_enabled: bool,
pub capture_propagation: f64,
pub default_texels_per_unit: f64,
pub environment_custom_color: Color,
pub environment_custom_energy: f64,
pub environment_custom_sky_rotation_degrees: Vector3,
pub environment_min_light: Color,
pub extents: Vector3,
pub image_path: String,
pub use_color: bool,
pub use_denoiser: bool,
pub use_hdr: bool,
}

impl Default for GDBakedLightmap {
    fn default() -> Self {
        Self {
            atlas_generate: Default::default(),
atlas_max_size: Default::default(),
bias: Default::default(),
bounce_indirect_energy: Default::default(),
bounces: Default::default(),
capture_cell_size: Default::default(),
capture_enabled: Default::default(),
capture_propagation: Default::default(),
default_texels_per_unit: Default::default(),
environment_custom_color: Color::from_rgb(0.0, 0.0, 0.0),
environment_custom_energy: Default::default(),
environment_custom_sky_rotation_degrees: Default::default(),
environment_min_light: Color::from_rgb(0.0, 0.0, 0.0),
extents: Default::default(),
image_path: Default::default(),
use_color: Default::default(),
use_denoiser: Default::default(),
use_hdr: Default::default(),
        }
    }
}

impl NodeClass for GDBakedLightmap {
    type Parent = GDVisualInstance;
    type GodotClass = BakedLightmap;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<BakedLightmap>().unwrap();
        world_commands.insert(entity, GDBakedLightmap {
            atlas_generate: component_ref.is_generate_atlas_enabled(),
atlas_max_size: component_ref.max_atlas_size(),
bias: component_ref.bias(),
bounce_indirect_energy: component_ref.bounce_indirect_energy(),
bounces: component_ref.bounces(),
capture_cell_size: component_ref.capture_cell_size(),
capture_enabled: component_ref.capture_enabled(),
capture_propagation: component_ref.capture_propagation(),
default_texels_per_unit: component_ref.default_texels_per_unit(),
environment_custom_color: component_ref.environment_custom_color(),
environment_custom_energy: component_ref.environment_custom_energy(),
environment_custom_sky_rotation_degrees: component_ref.environment_custom_sky_rotation_degrees(),
environment_min_light: component_ref.environment_min_light(),
extents: component_ref.extents(),
image_path: component_ref.image_path().to_string(),
use_color: component_ref.is_using_color(),
use_denoiser: component_ref.is_using_denoiser(),
use_hdr: component_ref.is_using_hdr(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDBakedLightmap {
    
}

fn sync_bevy_owned(query: Query<(&GDBakedLightmap, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<BakedLightmap>().unwrap();
        component_ref.set_generate_atlas(component.atlas_generate);
component_ref.set_max_atlas_size(component.atlas_max_size);
component_ref.set_bias(component.bias);
component_ref.set_bounce_indirect_energy(component.bounce_indirect_energy);
component_ref.set_bounces(component.bounces);
component_ref.set_capture_cell_size(component.capture_cell_size);
component_ref.set_capture_enabled(component.capture_enabled);
component_ref.set_capture_propagation(component.capture_propagation);
component_ref.set_default_texels_per_unit(component.default_texels_per_unit);
component_ref.set_environment_custom_color(component.environment_custom_color);
component_ref.set_environment_custom_energy(component.environment_custom_energy);
component_ref.set_environment_custom_sky_rotation_degrees(component.environment_custom_sky_rotation_degrees);
component_ref.set_environment_min_light(component.environment_min_light);
component_ref.set_extents(component.extents);
component_ref.set_image_path(component.image_path.clone());
component_ref.set_use_color(component.use_color);
component_ref.set_use_denoiser(component.use_denoiser);
component_ref.set_use_hdr(component.use_hdr);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDBakedLightmap, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<BakedLightmap>().unwrap();
        component.atlas_generate = component_ref.is_generate_atlas_enabled();
component.atlas_max_size = component_ref.max_atlas_size();
component.bias = component_ref.bias();
component.bounce_indirect_energy = component_ref.bounce_indirect_energy();
component.bounces = component_ref.bounces();
component.capture_cell_size = component_ref.capture_cell_size();
component.capture_enabled = component_ref.capture_enabled();
component.capture_propagation = component_ref.capture_propagation();
component.default_texels_per_unit = component_ref.default_texels_per_unit();
component.environment_custom_color = component_ref.environment_custom_color();
component.environment_custom_energy = component_ref.environment_custom_energy();
component.environment_custom_sky_rotation_degrees = component_ref.environment_custom_sky_rotation_degrees();
component.environment_min_light = component_ref.environment_min_light();
component.extents = component_ref.extents();
component.image_path = component_ref.image_path().to_string();
component.use_color = component_ref.is_using_color();
component.use_denoiser = component_ref.is_using_denoiser();
component.use_hdr = component_ref.is_using_hdr();
    }
}