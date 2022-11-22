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

pub struct VisibilityEnablerPlugin;

impl Plugin for VisibilityEnablerPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<VisibilityEnabler>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a visibility_enabler.
pub fn is_visibility_enabler(node: &gdnative::prelude::Node) -> bool {
    node.cast::<VisibilityEnabler>().is_some()
}

/// A bundle for VisibilityEnablers.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDVisibilityEnablerBundle {
    pub visibility_enabler: GDVisibilityEnabler,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visibility_notifier: GDVisibilityNotifier,
    pub true_type: TrueNodeType,
}

impl Default for GDVisibilityEnablerBundle {
    fn default() -> Self {
        Self {
            visibility_enabler: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visibility_notifier: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "VisibilityEnabler".to_string()
            }
        }
    }
}

/// Represents a VisibilityEnabler.
#[derive(Component)]
pub struct GDVisibilityEnabler {
    
}

impl Default for GDVisibilityEnabler {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDVisibilityEnabler {
    type Parent = GDVisibilityNotifier;
    type GodotClass = VisibilityEnabler;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<VisibilityEnabler>().unwrap();
        world_commands.insert(entity, GDVisibilityEnabler {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDVisibilityEnabler {
    
}

fn sync_bevy_owned(query: Query<(&GDVisibilityEnabler, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VisibilityEnabler>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDVisibilityEnabler, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VisibilityEnabler>().unwrap();
        
    }
}