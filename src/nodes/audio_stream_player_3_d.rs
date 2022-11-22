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

pub struct AudioStreamPlayer3DPlugin;

impl Plugin for AudioStreamPlayer3DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<AudioStreamPlayer3D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a audio_stream_player_3_d.
pub fn is_audio_stream_player_3_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<AudioStreamPlayer3D>().is_some()
}

/// A bundle for AudioStreamPlayer3Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDAudioStreamPlayer3DBundle {
    pub audio_stream_player_3_d: GDAudioStreamPlayer3D,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDAudioStreamPlayer3DBundle {
    fn default() -> Self {
        Self {
            audio_stream_player_3_d: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "AudioStreamPlayer3D".to_string()
            }
        }
    }
}

/// Represents a AudioStreamPlayer3D.
#[derive(Component)]
pub struct GDAudioStreamPlayer3D {
    pub area_mask: i64,
pub attenuation_filter_cutoff_hz: f64,
pub attenuation_filter_db: f64,
pub autoplay: bool,
pub bus: String,
pub emission_angle_degrees: f64,
pub emission_angle_enabled: bool,
pub emission_angle_filter_attenuation_db: f64,
pub max_db: f64,
pub max_distance: f64,
pub pitch_scale: f64,
pub stream_paused: bool,
pub unit_db: f64,
pub unit_size: f64,
}

impl Default for GDAudioStreamPlayer3D {
    fn default() -> Self {
        Self {
            area_mask: Default::default(),
attenuation_filter_cutoff_hz: Default::default(),
attenuation_filter_db: Default::default(),
autoplay: Default::default(),
bus: Default::default(),
emission_angle_degrees: Default::default(),
emission_angle_enabled: Default::default(),
emission_angle_filter_attenuation_db: Default::default(),
max_db: Default::default(),
max_distance: Default::default(),
pitch_scale: Default::default(),
stream_paused: Default::default(),
unit_db: Default::default(),
unit_size: Default::default(),
        }
    }
}

impl NodeClass for GDAudioStreamPlayer3D {
    type Parent = GDSpatial;
    type GodotClass = AudioStreamPlayer3D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<AudioStreamPlayer3D>().unwrap();
        world_commands.insert(entity, GDAudioStreamPlayer3D {
            area_mask: component_ref.area_mask(),
attenuation_filter_cutoff_hz: component_ref.attenuation_filter_cutoff_hz(),
attenuation_filter_db: component_ref.attenuation_filter_db(),
autoplay: component_ref.is_autoplay_enabled(),
bus: component_ref.bus().to_string(),
emission_angle_degrees: component_ref.emission_angle(),
emission_angle_enabled: component_ref.is_emission_angle_enabled(),
emission_angle_filter_attenuation_db: component_ref.emission_angle_filter_attenuation_db(),
max_db: component_ref.max_db(),
max_distance: component_ref.max_distance(),
pitch_scale: component_ref.pitch_scale(),
stream_paused: component_ref.stream_paused(),
unit_db: component_ref.unit_db(),
unit_size: component_ref.unit_size(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDAudioStreamPlayer3D {
    
}

fn sync_bevy_owned(query: Query<(&GDAudioStreamPlayer3D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AudioStreamPlayer3D>().unwrap();
        component_ref.set_area_mask(component.area_mask);
component_ref.set_attenuation_filter_cutoff_hz(component.attenuation_filter_cutoff_hz);
component_ref.set_attenuation_filter_db(component.attenuation_filter_db);
component_ref.set_autoplay(component.autoplay);
component_ref.set_bus(component.bus.clone());
component_ref.set_emission_angle(component.emission_angle_degrees);
component_ref.set_emission_angle_enabled(component.emission_angle_enabled);
component_ref.set_emission_angle_filter_attenuation_db(component.emission_angle_filter_attenuation_db);
component_ref.set_max_db(component.max_db);
component_ref.set_max_distance(component.max_distance);
component_ref.set_pitch_scale(component.pitch_scale);
component_ref.set_stream_paused(component.stream_paused);
component_ref.set_unit_db(component.unit_db);
component_ref.set_unit_size(component.unit_size);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDAudioStreamPlayer3D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AudioStreamPlayer3D>().unwrap();
        component.area_mask = component_ref.area_mask();
component.attenuation_filter_cutoff_hz = component_ref.attenuation_filter_cutoff_hz();
component.attenuation_filter_db = component_ref.attenuation_filter_db();
component.autoplay = component_ref.is_autoplay_enabled();
component.bus = component_ref.bus().to_string();
component.emission_angle_degrees = component_ref.emission_angle();
component.emission_angle_enabled = component_ref.is_emission_angle_enabled();
component.emission_angle_filter_attenuation_db = component_ref.emission_angle_filter_attenuation_db();
component.max_db = component_ref.max_db();
component.max_distance = component_ref.max_distance();
component.pitch_scale = component_ref.pitch_scale();
component.stream_paused = component_ref.stream_paused();
component.unit_db = component_ref.unit_db();
component.unit_size = component_ref.unit_size();
    }
}