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

pub struct AudioStreamPlayerPlugin;

impl Plugin for AudioStreamPlayerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<AudioStreamPlayer>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a audio_stream_player.
pub fn is_audio_stream_player(node: &gdnative::prelude::Node) -> bool {
    node.cast::<AudioStreamPlayer>().is_some()
}

/// A bundle for AudioStreamPlayers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDAudioStreamPlayerBundle {
    pub audio_stream_player: GDAudioStreamPlayer,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDAudioStreamPlayerBundle {
    fn default() -> Self {
        Self {
            audio_stream_player: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "AudioStreamPlayer".to_string()
            }
        }
    }
}

/// Represents a AudioStreamPlayer.
#[derive(Component)]
pub struct GDAudioStreamPlayer {
    pub autoplay: bool,
pub bus: String,
pub pitch_scale: f64,
pub stream_paused: bool,
pub volume_db: f64,
}

impl Default for GDAudioStreamPlayer {
    fn default() -> Self {
        Self {
            autoplay: Default::default(),
bus: Default::default(),
pitch_scale: Default::default(),
stream_paused: Default::default(),
volume_db: Default::default(),
        }
    }
}

impl NodeClass for GDAudioStreamPlayer {
    type Parent = GDNode;
    type GodotClass = AudioStreamPlayer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<AudioStreamPlayer>().unwrap();
        world_commands.insert(entity, GDAudioStreamPlayer {
            autoplay: component_ref.is_autoplay_enabled(),
bus: component_ref.bus().to_string(),
pitch_scale: component_ref.pitch_scale(),
stream_paused: component_ref.stream_paused(),
volume_db: component_ref.volume_db(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDAudioStreamPlayer {
    
}

fn sync_bevy_owned(query: Query<(&GDAudioStreamPlayer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AudioStreamPlayer>().unwrap();
        component_ref.set_autoplay(component.autoplay);
component_ref.set_bus(component.bus.clone());
component_ref.set_pitch_scale(component.pitch_scale);
component_ref.set_stream_paused(component.stream_paused);
component_ref.set_volume_db(component.volume_db);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDAudioStreamPlayer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<AudioStreamPlayer>().unwrap();
        component.autoplay = component_ref.is_autoplay_enabled();
component.bus = component_ref.bus().to_string();
component.pitch_scale = component_ref.pitch_scale();
component.stream_paused = component_ref.stream_paused();
component.volume_db = component_ref.volume_db();
    }
}