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

pub struct SpotLightPlugin;

impl Plugin for SpotLightPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<SpotLight>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a spot_light.
pub fn is_spot_light(node: &gdnative::prelude::Node) -> bool {
    node.cast::<SpotLight>().is_some()
}

/// A bundle for SpotLights.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDSpotLightBundle {
    pub spot_light: GDSpotLight,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub light: GDLight,
    pub true_type: TrueNodeType,
}

impl Default for GDSpotLightBundle {
    fn default() -> Self {
        Self {
            spot_light: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
light: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "SpotLight".to_string()
            }
        }
    }
}

/// Represents a SpotLight.
#[derive(Component)]
pub struct GDSpotLight {
    
}

impl Default for GDSpotLight {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDSpotLight {
    type Parent = GDLight;
    type GodotClass = SpotLight;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<SpotLight>().unwrap();
        world_commands.insert(entity, GDSpotLight {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDSpotLight {
    
}

fn sync_bevy_owned(query: Query<(&GDSpotLight, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<SpotLight>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDSpotLight, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<SpotLight>().unwrap();
        
    }
}