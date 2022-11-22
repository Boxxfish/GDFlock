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

pub struct RootMotionViewPlugin;

impl Plugin for RootMotionViewPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a root_motion_view.
pub fn is_root_motion_view(node: &gdnative::prelude::Node) -> bool {
    node.cast::<RootMotionView>().is_some()
}

/// A bundle for RootMotionViews.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDRootMotionViewBundle {
    pub root_motion_view: GDRootMotionView,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
    pub true_type: TrueNodeType,
}

impl Default for GDRootMotionViewBundle {
    fn default() -> Self {
        Self {
            root_motion_view: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "RootMotionView".to_string()
            }
        }
    }
}

/// Represents a RootMotionView.
#[derive(Component)]
pub struct GDRootMotionView {
    
}

impl Default for GDRootMotionView {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDRootMotionView {
    type Parent = GDVisualInstance;
    type GodotClass = RootMotionView;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<RootMotionView>().unwrap();
        world_commands.insert(entity, GDRootMotionView {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDRootMotionView {
    
}

fn sync_bevy_owned(query: Query<(&GDRootMotionView, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<RootMotionView>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDRootMotionView, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<RootMotionView>().unwrap();
        
    }
}