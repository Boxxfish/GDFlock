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

pub struct TextureProgressPlugin;

impl Plugin for TextureProgressPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<TextureProgress>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a texture_progress.
pub fn is_texture_progress(node: &gdnative::prelude::Node) -> bool {
    node.cast::<TextureProgress>().is_some()
}

/// A bundle for TextureProgresss.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDTextureProgressBundle {
    pub texture_progress: GDTextureProgress,
    pub node: GDNode,
pub canvas_item: GDCanvasItem,
pub control: GDControl,
pub range: GDRange,
    pub true_type: TrueNodeType,
}

impl Default for GDTextureProgressBundle {
    fn default() -> Self {
        Self {
            texture_progress: Default::default(),
            node: Default::default(),
canvas_item: Default::default(),
control: Default::default(),
range: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "TextureProgress".to_string()
            }
        }
    }
}

/// Represents a TextureProgress.
#[derive(Component)]
pub struct GDTextureProgress {
    pub fill_mode: i64,
pub nine_patch_stretch: bool,
pub radial_center_offset: Vector2,
pub radial_fill_degrees: f64,
pub radial_initial_angle: f64,
pub texture_over: Option<Ref<Texture>>,
pub texture_progress: Option<Ref<Texture>>,
pub texture_progress_offset: Vector2,
pub texture_under: Option<Ref<Texture>>,
pub tint_over: Color,
pub tint_progress: Color,
pub tint_under: Color,
}

impl Default for GDTextureProgress {
    fn default() -> Self {
        Self {
            fill_mode: Default::default(),
nine_patch_stretch: Default::default(),
radial_center_offset: Default::default(),
radial_fill_degrees: Default::default(),
radial_initial_angle: Default::default(),
texture_over: Default::default(),
texture_progress: Default::default(),
texture_progress_offset: Default::default(),
texture_under: Default::default(),
tint_over: Color::from_rgb(0.0, 0.0, 0.0),
tint_progress: Color::from_rgb(0.0, 0.0, 0.0),
tint_under: Color::from_rgb(0.0, 0.0, 0.0),
        }
    }
}

impl NodeClass for GDTextureProgress {
    type Parent = GDRange;
    type GodotClass = TextureProgress;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<TextureProgress>().unwrap();
        world_commands.insert(entity, GDTextureProgress {
            fill_mode: component_ref.fill_mode(),
nine_patch_stretch: component_ref.nine_patch_stretch(),
radial_center_offset: component_ref.radial_center_offset(),
radial_fill_degrees: component_ref.fill_degrees(),
radial_initial_angle: component_ref.radial_initial_angle(),
texture_over: component_ref.over_texture(),
texture_progress: component_ref.progress_texture(),
texture_progress_offset: component_ref.texture_progress_offset(),
texture_under: component_ref.under_texture(),
tint_over: component_ref.tint_over(),
tint_progress: component_ref.tint_progress(),
tint_under: component_ref.tint_under(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDTextureProgress {
    
}

fn sync_bevy_owned(query: Query<(&GDTextureProgress, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<TextureProgress>().unwrap();
        component_ref.set_fill_mode(component.fill_mode);
component_ref.set_nine_patch_stretch(component.nine_patch_stretch);
component_ref.set_radial_center_offset(component.radial_center_offset);
component_ref.set_fill_degrees(component.radial_fill_degrees);
component_ref.set_radial_initial_angle(component.radial_initial_angle);
component_ref.set_over_texture(component.texture_over.as_ref().unwrap().clone());
component_ref.set_progress_texture(component.texture_progress.as_ref().unwrap().clone());
component_ref.set_texture_progress_offset(component.texture_progress_offset);
component_ref.set_under_texture(component.texture_under.as_ref().unwrap().clone());
component_ref.set_tint_over(component.tint_over);
component_ref.set_tint_progress(component.tint_progress);
component_ref.set_tint_under(component.tint_under);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDTextureProgress, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<TextureProgress>().unwrap();
        component.fill_mode = component_ref.fill_mode();
component.nine_patch_stretch = component_ref.nine_patch_stretch();
component.radial_center_offset = component_ref.radial_center_offset();
component.radial_fill_degrees = component_ref.fill_degrees();
component.radial_initial_angle = component_ref.radial_initial_angle();
component.texture_over = component_ref.over_texture();
component.texture_progress = component_ref.progress_texture();
component.texture_progress_offset = component_ref.texture_progress_offset();
component.texture_under = component_ref.under_texture();
component.tint_over = component_ref.tint_over();
component.tint_progress = component_ref.tint_progress();
component.tint_under = component_ref.tint_under();
    }
}