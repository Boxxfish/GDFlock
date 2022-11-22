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

pub struct VideoPlayerPlugin;

impl Plugin for VideoPlayerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<VideoPlayer>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a video_player.
pub fn is_video_player(node: &gdnative::prelude::Node) -> bool {
    node.cast::<VideoPlayer>().is_some()
}

/// A bundle for VideoPlayers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDVideoPlayerBundle {
    pub video_player: GDVideoPlayer,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
    pub true_type: TrueNodeType,
}

impl Default for GDVideoPlayerBundle {
    fn default() -> Self {
        Self {
            video_player: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "VideoPlayer".to_string()
            }
        }
    }
}

/// Represents a VideoPlayer.
#[derive(Component)]
pub struct GDVideoPlayer {
    pub audio_track: i64,
pub autoplay: bool,
pub buffering_msec: i64,
pub bus: String,
pub expand: bool,
pub paused: bool,
pub stream_position: f64,
pub volume: f64,
pub volume_db: f64,
}

impl Default for GDVideoPlayer {
    fn default() -> Self {
        Self {
            audio_track: Default::default(),
autoplay: Default::default(),
buffering_msec: Default::default(),
bus: Default::default(),
expand: Default::default(),
paused: Default::default(),
stream_position: Default::default(),
volume: Default::default(),
volume_db: Default::default(),
        }
    }
}

impl NodeClass for GDVideoPlayer {
    type Parent = GDControl;
    type GodotClass = VideoPlayer;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<VideoPlayer>().unwrap();
        world_commands.insert(entity, GDVideoPlayer {
            audio_track: component_ref.audio_track(),
autoplay: component_ref.has_autoplay(),
buffering_msec: component_ref.buffering_msec(),
bus: component_ref.bus().to_string(),
expand: component_ref.has_expand(),
paused: component_ref.is_paused(),
stream_position: component_ref.stream_position(),
volume: component_ref.volume(),
volume_db: component_ref.volume_db(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDVideoPlayer {
    
}

fn sync_bevy_owned(query: Query<(&GDVideoPlayer, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VideoPlayer>().unwrap();
        component_ref.set_audio_track(component.audio_track);
component_ref.set_autoplay(component.autoplay);
component_ref.set_buffering_msec(component.buffering_msec);
component_ref.set_bus(component.bus.clone());
component_ref.set_expand(component.expand);
component_ref.set_paused(component.paused);
component_ref.set_stream_position(component.stream_position);
component_ref.set_volume(component.volume);
component_ref.set_volume_db(component.volume_db);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDVideoPlayer, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VideoPlayer>().unwrap();
        component.audio_track = component_ref.audio_track();
component.autoplay = component_ref.has_autoplay();
component.buffering_msec = component_ref.buffering_msec();
component.bus = component_ref.bus().to_string();
component.expand = component_ref.has_expand();
component.paused = component_ref.is_paused();
component.stream_position = component_ref.stream_position();
component.volume = component_ref.volume();
component.volume_db = component_ref.volume_db();
    }
}