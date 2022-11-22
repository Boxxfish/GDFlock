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

pub struct SliderPlugin;

impl Plugin for SliderPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a slider.
pub fn is_slider(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Slider>().is_some()
}

/// A bundle for Sliders.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDSliderBundle {
    pub slider: GDSlider,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub range: GDRange,
    pub true_type: TrueNodeType,
}

impl Default for GDSliderBundle {
    fn default() -> Self {
        Self {
            slider: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
range: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Slider".to_string()
            }
        }
    }
}

/// Represents a Slider.
#[derive(Component)]
pub struct GDSlider {
    pub editable: bool,
pub scrollable: bool,
pub tick_count: i64,
pub ticks_on_borders: bool,
}

impl Default for GDSlider {
    fn default() -> Self {
        Self {
            editable: Default::default(),
scrollable: Default::default(),
tick_count: Default::default(),
ticks_on_borders: Default::default(),
        }
    }
}

impl NodeClass for GDSlider {
    type Parent = GDRange;
    type GodotClass = Slider;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Slider>().unwrap();
        world_commands.insert(entity, GDSlider {
            editable: component_ref.is_editable(),
scrollable: component_ref.is_scrollable(),
tick_count: component_ref.ticks(),
ticks_on_borders: component_ref.ticks_on_borders(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDSlider {
    
}

fn sync_bevy_owned(query: Query<(&GDSlider, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Slider>().unwrap();
        component_ref.set_editable(component.editable);
component_ref.set_scrollable(component.scrollable);
component_ref.set_ticks(component.tick_count);
component_ref.set_ticks_on_borders(component.ticks_on_borders);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDSlider, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Slider>().unwrap();
        component.editable = component_ref.is_editable();
component.scrollable = component_ref.is_scrollable();
component.tick_count = component_ref.ticks();
component.ticks_on_borders = component_ref.ticks_on_borders();
    }
}