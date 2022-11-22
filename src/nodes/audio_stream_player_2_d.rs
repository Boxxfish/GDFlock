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

pub struct AudioStreamPlayer2DPlugin;

impl Plugin for AudioStreamPlayer2DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<AudioStreamPlayer2D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a audio_stream_player_2_d.
pub fn is_audio_stream_player_2_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<AudioStreamPlayer2D>().is_some()
}

/// A bundle for AudioStreamPlayer2Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDAudioStreamPlayer2DBundle {
    pub audio_stream_player_2_d: GDAudioStreamPlayer2D,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub node_2_d: GDNode2D,
    pub true_type: TrueNodeType,
}

impl Default for GDAudioStreamPlayer2DBundle {
    fn default() -> Self {
        Self {
            audio_stream_player_2_d: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
node_2_d: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "AudioStreamPlayer2D".to_string()
            }
        }
    }
}

/// Represents a AudioStreamPlayer2D.
#[derive(Component)]
pub struct GDAudioStreamPlayer2D {
    pub area_mask: i64,
pub attenuation: f64,
pub autoplay: bool,
pub bus: String,
pub max_distance: f64,
pub pitch_scale: f64,
pub stream_paused: bool,
pub volume_db: f64,
}

impl Default for GDAudioStreamPlayer2D {
    fn default() -> Self {
        Self {
            area_mask: Default::default(),
attenuation: Default::default(),
autoplay: Default::default(),
bus: Default::default(),
max_distance: Default::default(),
pitch_scale: Default::default(),
stream_paused: Default::default(),
volume_db: Default::default(),
        }
    }
}

impl NodeClass for GDAudioStreamPlayer2D {
    type Parent = GDNode2D;
    type GodotClass = AudioStreamPlayer2D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<AudioStreamPlayer2D>().unwrap();
        world_commands.insert(entity, GDAudioStreamPlayer2D {
            area_mask: component_ref.area_mask(),
attenuation: component_ref.attenuation(),
autoplay: component_ref.is_autoplay_enabled(),
bus: component_ref.bus().to_string(),
max_distance: component_ref.max_distance(),
pitch_scale: component_ref.pitch_scale(),
stream_paused: component_ref.stream_paused(),
volume_db: component_ref.volume_db(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDAudioStreamPlayer2D {
    
}

fn sync_bevy_owned(query: Query<(&GDAudioStreamPlayer2D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AudioStreamPlayer2D>().unwrap();
        component_ref.set_area_mask(component.area_mask);
component_ref.set_attenuation(component.attenuation);
component_ref.set_autoplay(component.autoplay);
component_ref.set_bus(component.bus.clone());
component_ref.set_max_distance(component.max_distance);
component_ref.set_pitch_scale(component.pitch_scale);
component_ref.set_stream_paused(component.stream_paused);
component_ref.set_volume_db(component.volume_db);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDAudioStreamPlayer2D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AudioStreamPlayer2D>().unwrap();
        component.area_mask = component_ref.area_mask();
component.attenuation = component_ref.attenuation();
component.autoplay = component_ref.is_autoplay_enabled();
component.bus = component_ref.bus().to_string();
component.max_distance = component_ref.max_distance();
component.pitch_scale = component_ref.pitch_scale();
component.stream_paused = component_ref.stream_paused();
component.volume_db = component_ref.volume_db();
    }
}