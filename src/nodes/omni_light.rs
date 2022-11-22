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

pub struct OmniLightPlugin;

impl Plugin for OmniLightPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<OmniLight>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a omni_light.
pub fn is_omni_light(node: &gdnative::prelude::Node) -> bool {
    node.cast::<OmniLight>().is_some()
}

/// A bundle for OmniLights.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDOmniLightBundle {
    pub omni_light: GDOmniLight,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub light: GDLight,
    pub true_type: TrueNodeType,
}

impl Default for GDOmniLightBundle {
    fn default() -> Self {
        Self {
            omni_light: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
light: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "OmniLight".to_string()
            }
        }
    }
}

/// Represents a OmniLight.
#[derive(Component)]
pub struct GDOmniLight {
    
}

impl Default for GDOmniLight {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDOmniLight {
    type Parent = GDLight;
    type GodotClass = OmniLight;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<OmniLight>().unwrap();
        world_commands.insert(entity, GDOmniLight {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDOmniLight {
    
}

fn sync_bevy_owned(query: Query<(&GDOmniLight, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<OmniLight>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDOmniLight, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<OmniLight>().unwrap();
        
    }
}