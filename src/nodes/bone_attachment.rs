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

pub struct BoneAttachmentPlugin;

impl Plugin for BoneAttachmentPlugin {
    fn build(&self, app: &mut App) {
        app .add_system_to_stage(GodotStages::Add, add_nodes::<BoneAttachment>)
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a bone_attachment.
pub fn is_bone_attachment(node: &gdnative::prelude::Node) -> bool {
    node.cast::<BoneAttachment>().is_some()
}

/// A bundle for BoneAttachments.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDBoneAttachmentBundle {
    pub bone_attachment: GDBoneAttachment,
    pub node: GDNode,
pub spatial: GDSpatial,
    pub true_type: TrueNodeType,
}

impl Default for GDBoneAttachmentBundle {
    fn default() -> Self {
        Self {
            bone_attachment: Default::default(),
            node: Default::default(),
spatial: Default::default(),
            true_type: TrueNodeType {
                node: None,
                class_name: "BoneAttachment".to_string()
            }
        }
    }
}

/// Represents a BoneAttachment.
#[derive(Component)]
pub struct GDBoneAttachment {
    pub bone_name: String,
}

impl Default for GDBoneAttachment {
    fn default() -> Self {
        Self {
            bone_name: Default::default(),
        }
    }
}

impl NodeClass for GDBoneAttachment {
    type Parent = GDSpatial;
    type GodotClass = BoneAttachment;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<BoneAttachment>().unwrap();
        world_commands.insert(entity, GDBoneAttachment {
            bone_name: component_ref.bone_name().to_string(),
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDBoneAttachment {
    
}

fn sync_bevy_owned(query: Query<(&GDBoneAttachment, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<BoneAttachment>().unwrap();
        component_ref.set_bone_name(component.bone_name.clone());
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDBoneAttachment, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<BoneAttachment>().unwrap();
        component.bone_name = component_ref.bone_name().to_string();
    }
}