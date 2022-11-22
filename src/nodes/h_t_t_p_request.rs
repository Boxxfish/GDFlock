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

pub struct HTTPRequestPlugin;

impl Plugin for HTTPRequestPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<HTTPRequest>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a h_t_t_p_request.
pub fn is_h_t_t_p_request(node: &gdnative::prelude::Node) -> bool {
    node.cast::<HTTPRequest>().is_some()
}

/// A bundle for HTTPRequests.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDHTTPRequestBundle {
    pub h_t_t_p_request: GDHTTPRequest,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDHTTPRequestBundle {
    fn default() -> Self {
        Self {
            h_t_t_p_request: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "HTTPRequest".to_string()
            }
        }
    }
}

/// Represents a HTTPRequest.
#[derive(Component)]
pub struct GDHTTPRequest {
    pub body_size_limit: i64,
pub download_chunk_size: i64,
pub download_file: String,
pub max_redirects: i64,
pub timeout: f64,
pub use_threads: bool,
}

impl Default for GDHTTPRequest {
    fn default() -> Self {
        Self {
            body_size_limit: Default::default(),
download_chunk_size: Default::default(),
download_file: Default::default(),
max_redirects: Default::default(),
timeout: Default::default(),
use_threads: Default::default(),
        }
    }
}

impl NodeClass for GDHTTPRequest {
    type Parent = GDNode;
    type GodotClass = HTTPRequest;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<HTTPRequest>().unwrap();
        world_commands.insert(entity, GDHTTPRequest {
            body_size_limit: component_ref.body_size_limit(),
download_chunk_size: component_ref.download_chunk_size(),
download_file: component_ref.download_file().to_string(),
max_redirects: component_ref.max_redirects(),
timeout: component_ref.timeout(),
use_threads: component_ref.is_using_threads(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDHTTPRequest {
    
}

fn sync_bevy_owned(query: Query<(&GDHTTPRequest, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<HTTPRequest>().unwrap();
        component_ref.set_body_size_limit(component.body_size_limit);
component_ref.set_download_chunk_size(component.download_chunk_size);
component_ref.set_download_file(component.download_file.clone());
component_ref.set_max_redirects(component.max_redirects);
component_ref.set_timeout(component.timeout);
component_ref.set_use_threads(component.use_threads);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDHTTPRequest, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<HTTPRequest>().unwrap();
        component.body_size_limit = component_ref.body_size_limit();
component.download_chunk_size = component_ref.download_chunk_size();
component.download_file = component_ref.download_file().to_string();
component.max_redirects = component_ref.max_redirects();
component.timeout = component_ref.timeout();
component.use_threads = component_ref.is_using_threads();
    }
}