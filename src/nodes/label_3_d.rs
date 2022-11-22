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

pub struct Label3DPlugin;

impl Plugin for Label3DPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Label3D>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a label_3_d.
pub fn is_label_3_d(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Label3D>().is_some()
}

/// A bundle for Label3Ds.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDLabel3DBundle {
    pub label_3_d: GDLabel3D,
    pub node: GDNode,
pub spatial: GDSpatial,
pub cull_instance: GDCullInstance,
pub visual_instance: GDVisualInstance,
pub geometry_instance: GDGeometryInstance,
    pub true_type: TrueNodeType,
}

impl Default for GDLabel3DBundle {
    fn default() -> Self {
        Self {
            label_3_d: Default::default(),
            node: Default::default(),
spatial: Default::default(),
cull_instance: Default::default(),
visual_instance: Default::default(),
geometry_instance: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Label3D".to_string()
            }
        }
    }
}

/// Represents a Label3D.
#[derive(Component)]
pub struct GDLabel3D {
    pub alpha_scissor_threshold: f64,
pub autowrap: bool,
pub line_spacing: f64,
pub modulate: Color,
pub offset: Vector2,
pub outline_modulate: Color,
pub outline_render_priority: i64,
pub pixel_size: f64,
pub render_priority: i64,
pub text: String,
pub uppercase: bool,
pub width: f64,
}

impl Default for GDLabel3D {
    fn default() -> Self {
        Self {
            alpha_scissor_threshold: Default::default(),
autowrap: Default::default(),
line_spacing: Default::default(),
modulate: Color::from_rgb(0.0, 0.0, 0.0),
offset: Default::default(),
outline_modulate: Color::from_rgb(0.0, 0.0, 0.0),
outline_render_priority: Default::default(),
pixel_size: Default::default(),
render_priority: Default::default(),
text: Default::default(),
uppercase: Default::default(),
width: Default::default(),
        }
    }
}

impl NodeClass for GDLabel3D {
    type Parent = GDGeometryInstance;
    type GodotClass = Label3D;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Label3D>().unwrap();
        world_commands.insert(entity, GDLabel3D {
            alpha_scissor_threshold: component_ref.alpha_scissor_threshold(),
autowrap: component_ref.autowrap(),
line_spacing: component_ref.line_spacing(),
modulate: component_ref.modulate(),
offset: component_ref.offset(),
outline_modulate: component_ref.outline_modulate(),
outline_render_priority: component_ref.outline_render_priority(),
pixel_size: component_ref.pixel_size(),
render_priority: component_ref.render_priority(),
text: component_ref.text().to_string(),
uppercase: component_ref.is_uppercase(),
width: component_ref.width(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDLabel3D {
    
}

fn sync_bevy_owned(query: Query<(&GDLabel3D, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Label3D>().unwrap();
        component_ref.set_alpha_scissor_threshold(component.alpha_scissor_threshold);
component_ref.set_autowrap(component.autowrap);
component_ref.set_line_spacing(component.line_spacing);
component_ref.set_modulate(component.modulate);
component_ref.set_offset(component.offset);
component_ref.set_outline_modulate(component.outline_modulate);
component_ref.set_outline_render_priority(component.outline_render_priority);
component_ref.set_pixel_size(component.pixel_size);
component_ref.set_render_priority(component.render_priority);
component_ref.set_text(component.text.clone());
component_ref.set_uppercase(component.uppercase);
component_ref.set_width(component.width);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDLabel3D, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Label3D>().unwrap();
        component.alpha_scissor_threshold = component_ref.alpha_scissor_threshold();
component.autowrap = component_ref.autowrap();
component.line_spacing = component_ref.line_spacing();
component.modulate = component_ref.modulate();
component.offset = component_ref.offset();
component.outline_modulate = component_ref.outline_modulate();
component.outline_render_priority = component_ref.outline_render_priority();
component.pixel_size = component_ref.pixel_size();
component.render_priority = component_ref.render_priority();
component.text = component_ref.text().to_string();
component.uppercase = component_ref.is_uppercase();
component.width = component_ref.width();
    }
}