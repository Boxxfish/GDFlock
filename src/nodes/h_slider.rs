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

pub struct HSliderPlugin;

impl Plugin for HSliderPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<HSlider>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a h_slider.
pub fn is_h_slider(node: &gdnative::prelude::Node) -> bool {
    node.cast::<HSlider>().is_some()
}

/// A bundle for HSliders.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDHSliderBundle {
    pub h_slider: GDHSlider,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub range: GDRange,
pub slider: GDSlider,
    pub true_type: TrueNodeType,
}

impl Default for GDHSliderBundle {
    fn default() -> Self {
        Self {
            h_slider: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
range: Default::default(),
slider: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "HSlider".to_string()
            }
        }
    }
}

/// Represents a HSlider.
#[derive(Component)]
pub struct GDHSlider {
    
}

impl Default for GDHSlider {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDHSlider {
    type Parent = GDSlider;
    type GodotClass = HSlider;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<HSlider>().unwrap();
        world_commands.insert(entity, GDHSlider {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDHSlider {
    
}

fn sync_bevy_owned(query: Query<(&GDHSlider, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<HSlider>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDHSlider, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<HSlider>().unwrap();
        
    }
}