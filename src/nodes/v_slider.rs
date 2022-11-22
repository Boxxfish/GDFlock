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

pub struct VSliderPlugin;

impl Plugin for VSliderPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<VSlider>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a v_slider.
pub fn is_v_slider(node: &gdnative::prelude::Node) -> bool {
    node.cast::<VSlider>().is_some()
}

/// A bundle for VSliders.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDVSliderBundle {
    pub v_slider: GDVSlider,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub range: GDRange,
pub slider: GDSlider,
    pub true_type: TrueNodeType,
}

impl Default for GDVSliderBundle {
    fn default() -> Self {
        Self {
            v_slider: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
range: Default::default(),
slider: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "VSlider".to_string()
            }
        }
    }
}

/// Represents a VSlider.
#[derive(Component)]
pub struct GDVSlider {
    
}

impl Default for GDVSlider {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl NodeClass for GDVSlider {
    type Parent = GDSlider;
    type GodotClass = VSlider;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<VSlider>().unwrap();
        world_commands.insert(entity, GDVSlider {
            
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDVSlider {
    
}

fn sync_bevy_owned(query: Query<(&GDVSlider, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VSlider>().unwrap();
        
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDVSlider, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<VSlider>().unwrap();
        
    }
}