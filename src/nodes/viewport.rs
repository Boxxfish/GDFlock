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

pub struct ViewportPlugin;

impl Plugin for ViewportPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<Viewport>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a viewport.
pub fn is_viewport(node: &gdnative::prelude::Node) -> bool {
    node.cast::<Viewport>().is_some()
}

/// A bundle for Viewports.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDViewportBundle {
    pub viewport: GDViewport,
    pub node: GDNode,
    pub true_type: TrueNodeType,
}

impl Default for GDViewportBundle {
    fn default() -> Self {
        Self {
            viewport: Default::default(),
            node: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "Viewport".to_string()
            }
        }
    }
}

/// Represents a Viewport.
#[derive(Component)]
pub struct GDViewport {
    pub arvr: bool,
pub audio_listener_enable_2d: bool,
pub audio_listener_enable_3d: bool,
pub canvas_transform: Transform2D,
pub debanding: bool,
pub disable_3d: bool,
pub fxaa: bool,
pub global_canvas_transform: Transform2D,
pub gui_disable_input: bool,
pub gui_snap_controls_to_pixels: bool,
pub handle_input_locally: bool,
pub hdr: bool,
pub keep_3d_linear: bool,
pub own_world: bool,
pub physics_object_picking: bool,
pub render_direct_to_screen: bool,
pub render_target_v_flip: bool,
pub shadow_atlas_size: i64,
pub sharpen_intensity: f64,
pub size: Vector2,
pub size_override_stretch: bool,
pub transparent_bg: bool,
pub use_32_bpc_depth: bool,
}

impl Default for GDViewport {
    fn default() -> Self {
        Self {
            arvr: Default::default(),
audio_listener_enable_2d: Default::default(),
audio_listener_enable_3d: Default::default(),
canvas_transform: Transform2D::IDENTITY,
debanding: Default::default(),
disable_3d: Default::default(),
fxaa: Default::default(),
global_canvas_transform: Transform2D::IDENTITY,
gui_disable_input: Default::default(),
gui_snap_controls_to_pixels: Default::default(),
handle_input_locally: Default::default(),
hdr: Default::default(),
keep_3d_linear: Default::default(),
own_world: Default::default(),
physics_object_picking: Default::default(),
render_direct_to_screen: Default::default(),
render_target_v_flip: Default::default(),
shadow_atlas_size: Default::default(),
sharpen_intensity: Default::default(),
size: Default::default(),
size_override_stretch: Default::default(),
transparent_bg: Default::default(),
use_32_bpc_depth: Default::default(),
        }
    }
}

impl NodeClass for GDViewport {
    type Parent = GDNode;
    type GodotClass = Viewport;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<Viewport>().unwrap();
        world_commands.insert(entity, GDViewport {
            arvr: component_ref.use_arvr(),
audio_listener_enable_2d: component_ref.is_audio_listener_2d(),
audio_listener_enable_3d: component_ref.is_audio_listener(),
canvas_transform: component_ref.canvas_transform(),
debanding: component_ref.use_debanding(),
disable_3d: component_ref.is_3d_disabled(),
fxaa: component_ref.use_fxaa(),
global_canvas_transform: component_ref.global_canvas_transform(),
gui_disable_input: component_ref.is_input_disabled(),
gui_snap_controls_to_pixels: component_ref.is_snap_controls_to_pixels_enabled(),
handle_input_locally: component_ref.is_handling_input_locally(),
hdr: component_ref.hdr(),
keep_3d_linear: component_ref.keep_3d_linear(),
own_world: component_ref.is_using_own_world(),
physics_object_picking: component_ref.physics_object_picking(),
render_direct_to_screen: component_ref.is_using_render_direct_to_screen(),
render_target_v_flip: component_ref.vflip(),
shadow_atlas_size: component_ref.shadow_atlas_size(),
sharpen_intensity: component_ref.sharpen_intensity(),
size: component_ref.size(),
size_override_stretch: component_ref.is_size_override_stretch_enabled(),
transparent_bg: component_ref.has_transparent_background(),
use_32_bpc_depth: component_ref.use_32_bpc_depth(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDViewport {
    
}

fn sync_bevy_owned(query: Query<(&GDViewport, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Viewport>().unwrap();
        component_ref.set_use_arvr(component.arvr);
component_ref.set_as_audio_listener_2d(component.audio_listener_enable_2d);
component_ref.set_as_audio_listener(component.audio_listener_enable_3d);
component_ref.set_canvas_transform(component.canvas_transform);
component_ref.set_use_debanding(component.debanding);
component_ref.set_disable_3d(component.disable_3d);
component_ref.set_use_fxaa(component.fxaa);
component_ref.set_global_canvas_transform(component.global_canvas_transform);
component_ref.set_disable_input(component.gui_disable_input);
component_ref.set_snap_controls_to_pixels(component.gui_snap_controls_to_pixels);
component_ref.set_handle_input_locally(component.handle_input_locally);
component_ref.set_hdr(component.hdr);
component_ref.set_keep_3d_linear(component.keep_3d_linear);
component_ref.set_use_own_world(component.own_world);
component_ref.set_physics_object_picking(component.physics_object_picking);
component_ref.set_use_render_direct_to_screen(component.render_direct_to_screen);
component_ref.set_vflip(component.render_target_v_flip);
component_ref.set_shadow_atlas_size(component.shadow_atlas_size);
component_ref.set_sharpen_intensity(component.sharpen_intensity);
component_ref.set_size(component.size);
component_ref.set_size_override_stretch(component.size_override_stretch);
component_ref.set_transparent_background(component.transparent_bg);
component_ref.set_use_32_bpc_depth(component.use_32_bpc_depth);
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDViewport, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<Viewport>().unwrap();
        component.arvr = component_ref.use_arvr();
component.audio_listener_enable_2d = component_ref.is_audio_listener_2d();
component.audio_listener_enable_3d = component_ref.is_audio_listener();
component.canvas_transform = component_ref.canvas_transform();
component.debanding = component_ref.use_debanding();
component.disable_3d = component_ref.is_3d_disabled();
component.fxaa = component_ref.use_fxaa();
component.global_canvas_transform = component_ref.global_canvas_transform();
component.gui_disable_input = component_ref.is_input_disabled();
component.gui_snap_controls_to_pixels = component_ref.is_snap_controls_to_pixels_enabled();
component.handle_input_locally = component_ref.is_handling_input_locally();
component.hdr = component_ref.hdr();
component.keep_3d_linear = component_ref.keep_3d_linear();
component.own_world = component_ref.is_using_own_world();
component.physics_object_picking = component_ref.physics_object_picking();
component.render_direct_to_screen = component_ref.is_using_render_direct_to_screen();
component.render_target_v_flip = component_ref.vflip();
component.shadow_atlas_size = component_ref.shadow_atlas_size();
component.sharpen_intensity = component_ref.sharpen_intensity();
component.size = component_ref.size();
component.size_override_stretch = component_ref.is_size_override_stretch_enabled();
component.transparent_bg = component_ref.has_transparent_background();
component.use_32_bpc_depth = component_ref.use_32_bpc_depth();
    }
}